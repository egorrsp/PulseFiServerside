use actix_web::{ web, HttpRequest, HttpResponse, Error, rt };
use actix_ws::{ handle, AggregatedMessage };
use futures_util::{ StreamExt, SinkExt };
use tokio_tungstenite::{ connect_async, tungstenite::Message as WsMessage };

pub async fn ws_proxy(req: HttpRequest, payload: web::Payload) -> Result<HttpResponse, Error> {
    let (res, session, stream) = handle(&req, payload)?;

    let upstream_url = "ws://127.0.0.1:8900";
    let (upstream_ws, _resp) = connect_async(upstream_url).await.map_err(|e| {
        eprintln!("Failed to connect to upstream WS: {:?}", e);
        actix_web::error::ErrorBadGateway("Upstream WS failed")
    })?;

    let (mut upstream_sink, mut upstream_stream) = upstream_ws.split();

    let mut client_stream = stream
        .aggregate_continuations()
        .max_continuation_size(1 << 20);

    let mut client_writer = session.clone();

    rt::spawn(async move {
        while let Some(msg_res) = client_stream.next().await {
            match msg_res {
                Ok(AggregatedMessage::Text(txt)) => {
                    if let Ok(s) = std::str::from_utf8(txt.as_ref()) {
                        if let Err(e) = upstream_sink.send(WsMessage::Text(s.to_string().into())).await {
                            eprintln!("Send to upstream failed (Text): {:?}", e);
                            break;
                        }
                    } else {
                        eprintln!("Non-UTF8 text from client - ignored");
                    }
                }
                Ok(AggregatedMessage::Binary(bin)) => {
                    if let Err(e) = upstream_sink.send(WsMessage::Binary(bin.to_vec().into())).await {
                        eprintln!("Send to upstream failed (Binary): {:?}", e);
                        break;
                    }
                }
                Ok(AggregatedMessage::Ping(p)) => {
                    let _ = upstream_sink.send(WsMessage::Ping(p.to_vec().into())).await;
                }
                Ok(AggregatedMessage::Pong(p)) => {
                    let _ = upstream_sink.send(WsMessage::Pong(p.to_vec().into())).await;
                }
                Ok(AggregatedMessage::Close(_)) => {
                    let _ = upstream_sink.send(WsMessage::Close(None)).await;
                    break;
                }
                Err(e) => {
                    eprintln!("Client stream error: {:?}", e);
                    let _ = upstream_sink.send(WsMessage::Close(None)).await;
                    break;
                }
            }
        }

        let _ = upstream_sink.close().await;
    });

    rt::spawn(async move {
        while let Some(msg_res) = upstream_stream.next().await {
            match msg_res {
                Ok(WsMessage::Text(txt)) => {
                    if let Err(e) = client_writer.text(txt.to_string()).await {
                        eprintln!("Error sending text to client: {:?}", e);
                        break;
                    }
                }
                Ok(WsMessage::Binary(bin)) => {
                    if let Err(e) = client_writer.binary(bin).await {
                        eprintln!("Error sending binary to client: {:?}", e);
                        break;
                    }
                }
                Ok(WsMessage::Ping(payload)) => {
                    let _ = client_writer.pong(payload.as_ref()).await;
                }
                Ok(WsMessage::Pong(payload)) => {
                    let _ = client_writer.pong(payload.as_ref()).await;
                }
                Ok(WsMessage::Close(_)) => {
                    let _ = client_writer.close(None).await;
                    break;
                }
                Err(e) => {
                    eprintln!("Upstream stream error: {:?}", e);
                    let _ = client_writer.close(None).await;
                    break;
                }
                _ => {}
            }
        }
    });

    Ok(res)
}