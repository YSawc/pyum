use chrono::Utc;
use rand::{distributions::Alphanumeric, Rng};
use sea_orm::*;

use super::{model, model::Entity as Oauth2ClientSecret};

pub async fn get_by_id(db: &DbConn) -> Result<Option<model::Model>, DbErr> {
    Oauth2ClientSecret::find()
        .order_by_desc(model::Column::ClientId)
        .one(db)
        .await
}

pub async fn create_oauth2_client_secret(
    db: &DbConn,
    admin_user_id: i32,
) -> Result<model::ActiveModel, DbErr> {
    let client_secret: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(64)
        .map(char::from)
        .collect();

    model::ActiveModel {
        client_secret: Set(client_secret),
        admin_user_id: Set(admin_user_id),
        ..Default::default()
    }
    .save(db)
    .await
}

pub async fn logic_delete(db: &DbConn, admin_user_id: i32) -> Result<UpdateResult, DbErr> {
    Ok(Oauth2ClientSecret::update_many()
        .filter(model::Column::AdminUserId.eq(admin_user_id))
        .col_expr(model::Column::DeletedAt, Utc::now().naive_utc().into())
        .exec(db)
        .await?)
}

pub async fn find_by_oauth_secret(
    db: &DbConn,
    client_secret: String,
) -> Result<Option<model::Model>, DbErr> {
    Oauth2ClientSecret::find()
        .filter(model::Column::ClientSecret.eq(client_secret))
        .one(db)
        .await
}
