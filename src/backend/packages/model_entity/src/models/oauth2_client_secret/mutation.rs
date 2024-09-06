use rand::{distributions::Alphanumeric, Rng};
use sea_orm::*;
use sqlx::types::chrono::Utc;

use super::{model, model::Entity as Oauth2ClientSecret};

pub async fn get_by_id(db: &DbConn) -> Result<Option<model::Model>, DbErr> {
    Oauth2ClientSecret::find()
        .order_by_desc(model::Column::ClientId)
        .one(db)
        .await
}

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

pub async fn logic_delete_all(db: &DbConn) -> Result<UpdateResult, DbErr> {
    Oauth2ClientSecret::update_many()
        .col_expr(model::Column::DeletedAt, Utc::now().naive_utc().into())
        .exec(db)
        .await
}
