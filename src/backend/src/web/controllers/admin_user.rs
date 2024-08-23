use axum::extract::State;

use axum::http::{HeaderMap, HeaderValue, StatusCode};
use axum::response::Html;
use axum::Json;
use model_entity::models::{admin_user, session};
use serde::Serialize;

use crate::web::middleware::AppState;

#[derive(Serialize)]
pub struct SimpleRes {
    message: String,
}

pub async fn get_create_admin_user(
    state: State<AppState>,
) -> Result<Html<String>, (StatusCode, &'static str)> {
    let ctx = tera::Context::new();
    let body = state
        .templates
        .render("pages/admin_user/new.html", &ctx)
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Template error"))?;

    Ok(Html(body))
}

pub async fn post_create_admin_user(
    state: State<AppState>,
    Json(admin_user): Json<admin_user::model::Model>,
) -> Result<Json<SimpleRes>, (StatusCode, &'static str)> {
    admin_user::mutation::create(&state.conn, admin_user)
        .await
        .expect("failed to create admin user.");

    Ok(Json(SimpleRes {
        message: "success to create admin user.".to_string(),
    }))
}

pub async fn get_login_admin_user(
    state: State<AppState>,
) -> Result<Html<String>, (StatusCode, &'static str)> {
    let ctx = tera::Context::new();
    let body = state
        .templates
        .render("pages/admin_user/login.html", &ctx)
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Template error"))?;

    Ok(Html(body))
}

#[derive(Serialize)]
pub struct PostLoginAdminUserRes {
    message: String,
    cid: String,
}

pub async fn post_login_admin_user(
    state: State<AppState>,
    Json(admin_user): Json<admin_user::model::Model>,
) -> Result<Json<PostLoginAdminUserRes>, (StatusCode, &'static str)> {
    if let Some(admin_user) = admin_user::mutation::find_by_name(&state.conn, admin_user)
        .await
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "failed to finding admin user by name.",
            )
        })
        .unwrap()
    {
        session::mutation::seed_unexpired_with_admin_user_id(&state.conn, admin_user.id)
            .await
            .expect("failed to create session");
        let all_session =
            session::mutation::find_unexpired_by_admin_user_id(&state.conn, admin_user.id)
                .await
                .expect("");
        let first_session = all_session.first().unwrap();
        let mut headers = HeaderMap::new();
        headers.insert(
            "cookie",
            HeaderValue::from_str(format!("cid={:?}", first_session.cookie_id.clone()).as_str())
                .unwrap(),
        );
        Ok(Json(PostLoginAdminUserRes {
            message: "success to login admin user.".to_string(),
            cid: first_session.cookie_id.clone().to_string(),
        }))
    } else {
        Err((StatusCode::INTERNAL_SERVER_ERROR, "user not found"))
    }
}
