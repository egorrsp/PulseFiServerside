use actix_web::{
    dev::{ServiceRequest, ServiceResponse},
    Error, middleware::Next, cookie::{Cookie, SameSite},
};
use awc::cookie::time;
use crate::api::refresh_tokens;


pub async fn jwt_middleware_with_refresh<B>(
    req: ServiceRequest,
    next: Next<B>,
) -> Result<ServiceResponse<B>, Error> {
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
        if crate::services::auth_tokens::verify_token(token) {
            access_token_valid = true;
        }
    }

    if !access_token_valid {
        if let Some(refresh_token) = refresh_token_opt {
            if let Some((new_access, new_refresh)) =
                refresh_tokens(&refresh_token)
            {
                new_tokens = Some((new_access, new_refresh));
                access_token_valid = true;
            }
        }
    }

    if access_token_valid {
        let mut res = next.call(req).await?;

        if let Some((access, refresh)) = new_tokens {
            let access_cookie = Cookie::build("access_token", access)
                .http_only(true)
                .secure(false) // для localhost
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
}