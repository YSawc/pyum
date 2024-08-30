use axum::{
    middleware,
    routing::{delete, get, patch, post},
    Router,
};

use crate::web::{auth::check_session_id, controllers::*, middleware::AppState};

pub fn router(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/device", get(device::list_devices))
        .route("/device/new", post(device::create_device))
        .route("/device/:device_id", get(device::detail_device))
        .route("/device/:device_id", patch(device::edit_device))
        .route("/device/:device_id", delete(device::delete_device))
        .layer(middleware::from_fn_with_state(state, check_session_id))
}
