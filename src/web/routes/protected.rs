use axum::{
    routing::{get, post},
    Router,
};

use crate::{web::controllers::*, web::middleware::AppState};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/device/", get(device::list_devices))
        .route("/device/new", get(device::new_device))
        .route("/device/new", post(device::create_device))
        .route("/device/:device_id", get(device::detail_device))
        .route("/device/:device_id/edit", get(device::get_edit_device))
        .route("/device/:device_id/edit", post(device::post_edit_device))
        .route("/device/:device_id/delete", get(device::delete_device))
}
