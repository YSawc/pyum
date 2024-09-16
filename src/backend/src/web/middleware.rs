use axum::{
    body::{Body, Bytes},
    extract::Request,
    http::{HeaderValue, StatusCode},
    middleware::Next,
    response::{IntoResponse, Redirect, Response},
};
use http_body_util::BodyExt;

use sea_orm::DatabaseConnection;

#[derive(Clone)]
pub struct AppState {
    pub conn: DatabaseConnection,
}

impl AppState {
    pub fn new(conn: DatabaseConnection) -> Self {
        Self { conn }
    }
}

pub fn error_response(code: u16, message: &str) -> Response {
    let to = format!("/error?code={}&message={}", code, message);
    let r = Redirect::to(&to);
    let mut r = r.into_response();
    let h = r.headers_mut();
    h.insert("HX-Redirect", HeaderValue::from_str(&to).unwrap());
    r
}

pub async fn print_request_response(
    req: Request,
    next: Next,
) -> Result<impl IntoResponse, StatusCode> {
    let (parts, body) = req.into_parts();
    let bytes = buffer_and_print("request", body).await?;
    let req = Request::from_parts(parts, Body::from(bytes));
    let res = next.run(req).await;
    let (parts, body) = res.into_parts();
    let bytes = buffer_and_print("response", body).await?;
    let res = Response::from_parts(parts, Body::from(bytes));

    Ok(res)
}

pub async fn buffer_and_print<B>(direction: &str, body: B) -> Result<Bytes, StatusCode>
where
    B: axum::body::HttpBody<Data = Bytes>,
    B::Error: std::fmt::Display,
{
    let bytes = match body.collect().await {
        Ok(collected) => collected.to_bytes(),
        Err(_err) => return Err(StatusCode::UNAUTHORIZED),
    };

    if let Ok(body) = std::str::from_utf8(&bytes) {
        tracing::debug!("{direction} body = {body:?}");
    }

    Ok(bytes)
}
