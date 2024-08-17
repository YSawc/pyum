use ::chrono::Days;
use bcrypt::DEFAULT_COST;
use rand::{distributions::Alphanumeric, Rng};
use sea_orm::*;
use sqlx::types::chrono::Utc;

use crate::models::session;

use super::{model, model::Entity as AdminUser};

pub async fn seed(db: &DbConn) -> Result<model::ActiveModel, DbErr> {
    let name = "test".to_string();
    let encrypted_password =
        bcrypt::hash("test", DEFAULT_COST).expect("error occured when encrypting password");
    model::ActiveModel {
        name: Set(name),
        encrypted_password: ActiveValue::Set(encrypted_password),
        ..Default::default()
    }
    .save(db)
    .await
}

pub async fn seed_with_session(
    db: &DbConn,
) -> Result<(model::ActiveModel, session::model::ActiveModel), DbErr> {
    let name = "test".to_string();
    let encrypted_password =
        bcrypt::hash("test", DEFAULT_COST).expect("error occured when encrypting password");

    let admin_user = model::ActiveModel {
        name: Set(name),
        encrypted_password: ActiveValue::Set(encrypted_password),
        ..Default::default()
    }
    .save(db)
    .await
    .expect("failed to seed admin_user");

    let naive_date_time = (Utc::now().naive_utc())
        .checked_add_days(Days::new(1))
        .expect("failed to construct datetime");

    let rand_str: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(64)
        .map(char::from)
        .collect();

    let session = session::model::ActiveModel {
        admin_user_id: admin_user.to_owned().id,
        cookie_id: ActiveValue::set(rand_str),
        expire_at: ActiveValue::set(naive_date_time),
        ..Default::default()
    }
    .save(db)
    .await
    .expect("failed to seed admin_user");

    Ok((admin_user, session))
}

pub async fn create(db: &DbConn, form_data: model::Model) -> Result<model::ActiveModel, DbErr> {
    let encrypted_password = bcrypt::hash(form_data.password, DEFAULT_COST)
        .expect("error occured when encrypting password");

    model::ActiveModel {
        name: Set(form_data.name.to_owned()),
        encrypted_password: ActiveValue::Set(encrypted_password),
        ..Default::default()
    }
    .save(db)
    .await
}

pub async fn find_by_name(
    db: &DbConn,
    form_data: model::Model,
) -> Result<Option<model::Model>, DbErr> {
    AdminUser::find()
        .filter(model::Column::Name.eq(form_data.name))
        .one(db)
        .await
}

pub async fn find_by_id(db: &DbConn, uid: i32) -> Result<Option<model::Model>, DbErr> {
    AdminUser::find()
        .filter(model::Column::Id.eq(uid))
        .one(db)
        .await
}

pub async fn find_by_id_with_session(
    db: &DbConn,
    uid: i32,
) -> Result<Vec<(model::Model, Vec<session::model::Model>)>, DbErr> {
    AdminUser::find()
        .find_with_related(session::model::Entity)
        .filter(model::Column::Id.eq(uid))
        .all(db)
        .await
}

pub async fn find_all(db: &DbConn) -> Result<Vec<model::Model>, DbErr> {
    AdminUser::find()
        .filter(model::Column::Id.is_not_null())
        .all(db)
        .await
}

pub async fn delete_all(db: &DbConn) -> Result<DeleteResult, DbErr> {
    AdminUser::delete_many()
        .filter(model::Column::Id.is_not_null())
        .exec(db)
        .await
}
