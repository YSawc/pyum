pub mod api;
use crate::web::{auth::check_session_id, controllers::*, middleware::AppState};
use axum::{
    middleware,
    routing::{delete, get, patch, post},
    Router,
};

pub fn router(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/device", get(device::list_devices))
        .route("/device", post(device::create_device))
        .route("/device/:device_id", get(device::detail_device))
        .route("/device/:device_id", patch(device::edit_device))
        .route("/device/:device_id", delete(device::delete_device))
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
        .route("/sensor", get(sensor::list_related_sensor))
        .route("/sensor", post(sensor::create))
        .route("/sensor/:sensor_id", get(sensor::detail))
        .route("/sensor/:sensor_id", patch(sensor::edit))
        .route("/sensor/:sensor_id", delete(sensor::delete))
        .route("/capture", get(capture::list_related_capture))
        .layer(middleware::from_fn_with_state(state, check_session_id))
}
