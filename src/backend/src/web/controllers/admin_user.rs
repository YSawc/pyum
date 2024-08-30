use axum::extract::State;

use axum::http::StatusCode;
use axum::response::Html;
use axum::Json;
use model_entity::models::admin_user::mutation::verify_password;
use model_entity::models::{admin_user, session};
use tower_sessions::Session;

use crate::web::middleware::AppState;
use crate::web::SimpleRes;

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

pub async fn post_login_admin_user(
    session: Session,
    state: State<AppState>,
    Json(body): Json<admin_user::model::Model>,
) -> Result<Json<SimpleRes>, (StatusCode, &'static str)> {
    if let Some(admin_user) = admin_user::mutation::find_by_name(&state.conn, body.clone())
        .await
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "failed to finding admin user by name.",
            )
        })
        .unwrap()
    {
        match verify_password(admin_user.encrypted_password, body.password) {
            Ok(res) => match res {
                true => {
                    session::mutation::seed_unexpired_with_admin_user_id(
                        &state.conn,
                        admin_user.id,
                    )
                    .await
                    .expect("failed to create session");
                    let all_session = session::mutation::find_unexpired_by_admin_user_id(
                        &state.conn,
                        admin_user.id,
                    )
                    .await
                    .expect("");
                    let last_session = all_session.last().unwrap();
                    session
                        .insert("uid", last_session.admin_user_id)
                        .await
                        .unwrap();
                    Ok(Json(SimpleRes {
                        message: "success to login admin user.".to_string(),
                    }))
                }
                false => Err((StatusCode::INTERNAL_SERVER_ERROR, "password is wrong")),
            },
            Err(_err) => Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "something is wrong in verify password",
            )),
        }
    } else {
        Err((StatusCode::INTERNAL_SERVER_ERROR, "user not found"))
    }
}
