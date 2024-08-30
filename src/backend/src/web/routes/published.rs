use super::super::controllers::*;
use crate::web::middleware::AppState;
use axum::{
    routing::{get, post},
    Router,
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/health_check", get(api::health_check_handler))
        .route("/session/check_valid", post(session::check_valid))
        .route("/assets/images/:path", get(assets::get_image_asset))
        .route("/admin_user/new", post(admin_user::post_create_admin_user))
        .route("/admin_user/login", post(admin_user::post_login_admin_user))
}
