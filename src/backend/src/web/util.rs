use axum::{extract::State, Json};
use model_entity::models::session::{self};

use super::{middleware::AppState, SimpleRes};

pub async fn get_uid(state: &State<AppState>) -> Result<i32, Json<SimpleRes>> {
    let cid = "cid";
    let vec = session::mutation::find_unexpired_by_cid(&state.conn, cid.to_string())
        .await
        .expect("failed to checking session");
    let target = vec.first();
    match target {
        Some((_sessions, maybe_user)) => match maybe_user {
            Some(user) => Ok(user.id),
            None => Err(Json(SimpleRes {
                message: "user related user not found.".to_string(),
            })),
        },
        None => Err(Json(SimpleRes {
            message: "session is not valid".to_string(),
        })),
    }
}
