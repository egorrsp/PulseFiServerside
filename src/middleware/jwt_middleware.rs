use actix_web::{
    dev::{Service, Transform, ServiceRequest, ServiceResponse},
    Error, cookie::{Cookie, SameSite},
};
use futures_util::future::{ok, Ready, LocalBoxFuture};
use std::{rc::Rc, task::{Context, Poll}};
use awc::cookie::time;
use crate::api::refresh_tokens;

pub struct JwtMiddlewareFactory {
    pub jwt_secret: String,
}

pub struct JwtMiddleware<S> {
    service: Rc<S>,
    jwt_secret: String,
}

impl<S, B> Transform<S, ServiceRequest> for JwtMiddlewareFactory
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = JwtMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(JwtMiddleware {
            service: Rc::new(service),
            jwt_secret: self.jwt_secret.clone(),
        })
    }
}

impl<S, B> Service<ServiceRequest> for JwtMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let srv = Rc::clone(&self.service);
        let jwt_secret = self.jwt_secret.clone();

        Box::pin(async move {
            let access_token_opt = req
                .headers()
                .get("Authorization")
                .and_then(|hv| hv.to_str().ok())
                .and_then(|s| s.strip_prefix("Bearer "))
                .map(|s| s.trim().to_string())
                .or_else(|| req.cookie("access_token").map(|c| c.value().to_string()));

            let refresh_token_opt = req.cookie("refresh_token").map(|c| c.value().to_string());

            let mut access_token_valid = false;
            let mut new_tokens: Option<(String, String)> = None;

            if let Some(ref token) = access_token_opt {
                if crate::services::auth_tokens::verify_token(token, &jwt_secret) {
                    access_token_valid = true;
                }
            }

            if !access_token_valid {
                if let Some(refresh_token) = refresh_token_opt {
                    if let Some((new_access, new_refresh)) = refresh_tokens(&refresh_token, &jwt_secret) {
                        new_tokens = Some((new_access, new_refresh));
                        access_token_valid = true;
                    }
                }
            }

            if access_token_valid {
                let mut res = srv.call(req).await?;

                if let Some((access, refresh)) = new_tokens {
                    let access_cookie = Cookie::build("access_token", access)
                        .http_only(true)
                        .secure(false)
                        .same_site(SameSite::Lax)
                        .max_age(time::Duration::hours(1))
                        .path("/")
                        .finish();

                    let refresh_cookie = Cookie::build("refresh_token", refresh)
                        .http_only(true)
                        .secure(false)
                        .same_site(SameSite::Lax)
                        .max_age(time::Duration::days(7))
                        .path("/")
                        .finish();

                    res.response_mut().add_cookie(&access_cookie).ok();
                    res.response_mut().add_cookie(&refresh_cookie).ok();
                }

                Ok(res)
            } else {
                Err(actix_web::error::ErrorUnauthorized("Missing or invalid token"))
            }
        })
    }
}