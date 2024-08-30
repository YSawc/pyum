use crate::web::SimpleRes;
use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};
use tower_sessions::Session;

#[derive(Debug, Serialize, Deserialize)]
pub struct CheckValidBody {
    cid: String,
}

pub async fn check_valid(session: Session) -> Result<Json<SimpleRes>, (StatusCode, &'static str)> {
    let uid = session.get_value("uid").await.unwrap();
    match uid {
        Some(_uid) => Ok(Json(SimpleRes {
            message: "session is valid".to_string(),
        })),
        None => Err((StatusCode::INTERNAL_SERVER_ERROR, "session is not valid")),
    }
}
