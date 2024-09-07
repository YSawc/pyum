use axum::{
    middleware,
    routing::{delete, get, patch, post},
    Router,
};

use crate::web::{auth::check_session_id, controllers::*, middleware::AppState};

pub fn router(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/device", get(device::list_devices))
        .route("/device", post(device::create_device))
        .route("/device/:device_id", get(device::detail_device))
        .route("/device/:device_id", patch(device::edit_device))
        .route("/device/:device_id", delete(device::delete_device))
        .route(
            "/device/:device_id/sensor",
            get(sensor::list_related_device),
        )
        .route("/sensor_purpose", get(sensor_purpose::list))
        .route("/sensor_purpose", post(sensor_purpose::create))
        .route(
            "/sensor_purpose/:sensor_purpose_id",
            get(sensor_purpose::detail),
        )
        .route(
            "/sensor_purpose/:sensor_purpose_id",
            patch(sensor_purpose::edit),
        )
        .route(
            "/sensor_purpose/:sensor_purpose_id",
            delete(sensor_purpose::delete),
        )
        .layer(middleware::from_fn_with_state(state, check_session_id))
}
