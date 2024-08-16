use bcrypt::DEFAULT_COST;
use sea_orm::*;

use crate::session;

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
