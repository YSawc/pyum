use bcrypt::{verify, BcryptError, DEFAULT_COST};
use sea_orm::*;

use crate::models::session::{
    self,
    mutation::{seed_expired_with_admin_user_id, seed_unexpired_with_admin_user_id},
};

use super::{model, model::Entity as AdminUser};

fn get_hashed_password(password: String) -> Result<std::string::String, BcryptError> {
    Ok(bcrypt::hash(password, DEFAULT_COST)?)
}

pub async fn seed(db: &DbConn) -> Result<model::ActiveModel, DbErr> {
    let name = "test".to_string();
    let encrypted_password =
        get_hashed_password("test".to_string()).expect("error occured when encrypting password");
    model::ActiveModel {
        name: Set(name),
        encrypted_password: ActiveValue::Set(encrypted_password),
        ..Default::default()
    }
    .save(db)
    .await
}

pub async fn seed_with_unexpired_session(
    db: &DbConn,
) -> Result<(model::ActiveModel, session::model::ActiveModel), DbErr> {
    let admin_user = seed(db).await.expect("failed to seed admin_user");
    let session = seed_unexpired_with_admin_user_id(db, *admin_user.to_owned().id.as_ref())
        .await
        .expect("failed to seed session");

    Ok((admin_user, session))
}

pub async fn seed_with_expired_session(
    db: &DbConn,
) -> Result<(model::ActiveModel, session::model::ActiveModel), DbErr> {
    let admin_user = seed(db).await.expect("failed to seed admin_user");
    let session = seed_expired_with_admin_user_id(db, *admin_user.to_owned().id.as_ref())
        .await
        .expect("failed to seed session");

    Ok((admin_user, session))
}

pub fn verify_password(encrypted_password: String, password: String) -> Result<bool, BcryptError> {
    Ok(verify(password, &encrypted_password)?)
}

pub async fn create(db: &DbConn, form_data: model::Model) -> Result<model::ActiveModel, DbErr> {
    let encrypted_password =
        get_hashed_password(form_data.password).expect("error occured when encrypting password");

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
