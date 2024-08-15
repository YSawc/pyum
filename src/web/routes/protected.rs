use axum::{
    body::Body,
    extract::Request,
    http::StatusCode,
    middleware::{self, Next},
    response::IntoResponse,
    routing::{get, post},
    Router,
};

use crate::web::{
    controllers::*,
    middleware::{buffer_and_print, AppState},
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/device/", get(device::list_devices))
        .route("/device/new", get(device::new_device))
        .route("/device/new", post(device::create_device))
        .route("/device/:device_id", get(device::detail_device))
        .route("/device/:device_id/edit", get(device::get_edit_device))
        .route("/device/:device_id/edit", post(device::post_edit_device))
        .route("/device/:device_id/delete", get(device::delete_device))
        .layer(middleware::from_fn(check_session_id))
}

pub async fn check_session_id(
    req: Request,
    next: Next,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let (parts, body) = req.into_parts();
    println!("{:?}", parts.headers);
    let bytes = buffer_and_print("request", body).await?;
    let req = Request::from_parts(parts, Body::from(bytes));
    let res = next.run(req).await;
    Ok(res)
}
