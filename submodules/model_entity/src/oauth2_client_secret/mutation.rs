use rand::{distributions::Alphanumeric, Rng};
use sea_orm::*;

use super::model;

pub async fn create_oauth2_client_secret(db: &DbConn) -> Result<model::ActiveModel, DbErr> {
    let client_secret: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(64)
        .map(char::from)
        .collect();

    model::ActiveModel {
        client_secret: Set(client_secret),
        ..Default::default()
    }
    .save(db)
    .await
}
