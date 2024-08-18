use crate::utils::{generate_1day_after_date_time, generate_1day_before_date_time};

use super::{model, model::Entity as Session};
use chrono::Utc;
use rand::{distributions::Alphanumeric, Rng};
use sea_orm::*;

pub async fn seed_unexpired_with_admin_user_id(
    db: &DbConn,
    admin_user_id: i32,
) -> Result<model::ActiveModel, DbErr> {
    let naive_date_time = generate_1day_after_date_time();
    let rand_str: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(64)
        .map(char::from)
        .collect();
    let session = model::ActiveModel {
        admin_user_id: ActiveValue::Set(admin_user_id),
        cookie_id: ActiveValue::set(rand_str),
        expire_at: ActiveValue::set(naive_date_time),
        ..Default::default()
    }
    .save(db)
    .await
    .expect("failed to seed session");

    Ok(session)
}

pub async fn seed_expired_with_admin_user_id(
    db: &DbConn,
    admin_user_id: i32,
) -> Result<model::ActiveModel, DbErr> {
    let naive_date_time = generate_1day_before_date_time();
    let rand_str: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(64)
        .map(char::from)
        .collect();
    let session = model::ActiveModel {
        admin_user_id: ActiveValue::Set(admin_user_id),
        cookie_id: ActiveValue::set(rand_str),
        expire_at: ActiveValue::set(naive_date_time),
        ..Default::default()
    }
    .save(db)
    .await
    .expect("failed to seed session");

    Ok(session)
}

pub async fn find_unexpired_by_admin_user_id(
    db: &DbConn,
    admin_user_id: i32,
) -> Result<Vec<model::Model>, DbErr> {
    Session::find()
        .filter(model::Column::AdminUserId.eq(admin_user_id))
        .filter(model::Column::ExpireAt.gte(Utc::now()))
        .all(db)
        .await
}

pub async fn find_expired_by_admin_user_id(
    db: &DbConn,
    admin_user_id: i32,
) -> Result<Vec<model::Model>, DbErr> {
    Session::find()
        .filter(model::Column::AdminUserId.eq(admin_user_id))
        .filter(model::Column::ExpireAt.lt(Utc::now()))
        .all(db)
        .await
}

pub async fn delete_all(db: &DbConn) -> Result<DeleteResult, DbErr> {
    Session::delete_many()
        .filter(model::Column::Id.is_not_null())
        .exec(db)
        .await
}
