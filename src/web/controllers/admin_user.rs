use axum::extract::State;

use axum::http::StatusCode;
use axum::response::{Html, Redirect};
use axum::Form;
use model_entity::admin_user;

use crate::web::middleware::AppState;

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
    Form(admin_user): Form<admin_user::model::Model>,
) -> Result<Redirect, (StatusCode, &'static str)> {
    admin_user::mutation::create(&state.conn, admin_user)
        .await
        .expect("failed to create admin user.");

    Ok(Redirect::to("/device/"))
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

pub async fn post_login_admin_user(
    state: State<AppState>,
    Form(admin_user): Form<admin_user::model::Model>,
) -> Result<Redirect, (StatusCode, &'static str)> {
    admin_user::mutation::find_by_name(&state.conn, admin_user)
        .await
        .unwrap();

    Ok(Redirect::to("/device/"))
}
