use sea_orm::*;

use super::model::{self, Entity as SensorPurpose};

pub async fn create(
    db: &DbConn,
    form_data: model::Model,
    uid: i32,
) -> Result<model::ActiveModel, DbErr> {
    model::ActiveModel {
        admin_user_id: Set(uid),
        description: Set(form_data.description.to_owned()),
        ..Default::default()
    }
    .save(db)
    .await
}

pub async fn get_by_id(db: &DbConn, id: i32) -> Result<Option<model::Model>, DbErr> {
    SensorPurpose::find_by_id(id).one(db).await
}

pub async fn update_by_id(
    db: &DbConn,
    id: i32,
    form_data: model::Model,
) -> Result<model::Model, DbErr> {
    let sensor_purpose: model::ActiveModel = SensorPurpose::find_by_id(id)
        .one(db)
        .await?
        .ok_or(DbErr::Custom("Cannot find sensor purpose.".to_owned()))
        .map(Into::into)?;

    model::ActiveModel {
        id: sensor_purpose.id,
        description: Set(form_data.description.to_owned()),
        ..Default::default()
    }
    .update(db)
    .await
}

pub async fn delete_by_id(db: &DbConn, id: i32) -> Result<DeleteResult, DbErr> {
    let sensor_purpose: model::ActiveModel = SensorPurpose::find_by_id(id)
        .one(db)
        .await?
        .ok_or(DbErr::Custom("Cannot find sensor purpose.".to_owned()))
        .map(Into::into)?;

    sensor_purpose.delete(db).await
}
