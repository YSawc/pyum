use crate::web::{auth::check_oauth_secret, controllers::*, middleware::AppState};
use axum::{middleware, routing::post, Router};
use capture::create_capture;

pub fn router(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/api/capture", post(create_capture))
        .layer(middleware::from_fn_with_state(state, check_oauth_secret))
}
