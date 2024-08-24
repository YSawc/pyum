use crate::web::middleware::AppState;
use axum::{
    routing::{get, post},
    Router,
};

use super::super::controllers::*;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/health_check", get(api::health_check_handler))
        .route("/hello", get(hello::hello))
        .route("/session/check_valid", post(session::check_valid))
        .route("/assets/images/:path", get(assets::get_image_asset))
        .route("/admin_user/new", get(admin_user::get_create_admin_user))
        .route("/admin_user/new", post(admin_user::post_create_admin_user))
        .route("/admin_user/login", get(admin_user::get_login_admin_user))
        .route("/admin_user/login", post(admin_user::post_login_admin_user))
}
