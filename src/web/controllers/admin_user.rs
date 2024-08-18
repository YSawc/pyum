use axum::extract::State;

use axum::http::{HeaderMap, HeaderValue, StatusCode};
use axum::response::{Html, IntoResponse, Redirect};
use axum::Form;
use model_entity::models::{admin_user, session};

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
) -> Result<impl IntoResponse, (StatusCode, &'static str)> {
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
        Ok((headers, Redirect::to("/device/")).into_response())
    } else {
        Ok(Redirect::to("/admin_user/login").into_response())
    }
}
