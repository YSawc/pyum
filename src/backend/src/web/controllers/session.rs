use crate::web::middleware::AppState;
use crate::web::SimpleRes;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use model_entity::models::session;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CheckValidBody {
    cid: String,
}

pub async fn check_valid(
    state: State<AppState>,
    Json(body): Json<CheckValidBody>,
) -> Result<Json<SimpleRes>, (StatusCode, &'static str)> {
    if let Some(_session) = session::mutation::find_by_cid(&state.conn, body.cid)
        .await
        .expect("failed to checking session")
    {
        Ok(Json(SimpleRes {
            message: "session is valid".to_string(),
        }))
    } else {
        Err((StatusCode::INTERNAL_SERVER_ERROR, "session is not valid"))
    }
}
