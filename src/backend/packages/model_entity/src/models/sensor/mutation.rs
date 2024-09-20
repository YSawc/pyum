use super::model::{self, Entity as Sensor};
use crate::models::sensor_purpose;
use sea_orm::*;

pub async fn create(db: &DbConn, form_data: model::Model) -> Result<model::ActiveModel, DbErr> {
    model::ActiveModel {
        device_id: Set(form_data.device_id),
        sensor_purpose_id: Set(form_data.sensor_purpose_id),
        trigger_limit_val: Set(form_data.trigger_limit_val),
        trigger_limit_sequence_count: Set(form_data.trigger_limit_sequence_count),
        ..Default::default()
    }
    .save(db)
    .await
}

pub async fn get_by_id(
    db: &DbConn,
    id: i32,
) -> Result<(model::Model, sensor_purpose::model::Model), DbErr> {
    let models = Sensor::find_by_id(id)
        .find_also_related(sensor_purpose::model::Entity)
        .one(db)
        .await?
        .unwrap();
    Ok((models.0, models.1.unwrap()))
}

pub async fn update(
    db: &DbConn,
    form_data: model::Model,
    sensor_id: i32,
) -> Result<model::Model, DbErr> {
    let sensor: model::ActiveModel = Sensor::find_by_id(sensor_id)
        .one(db)
        .await?
        .ok_or(DbErr::Custom("Cannot find sensor.".to_owned()))
        .map(Into::into)?;

    model::ActiveModel {
        id: sensor.id,
        sensor_purpose_id: Set(form_data.sensor_purpose_id),
        trigger_limit_val: Set(form_data.trigger_limit_val),
        trigger_limit_sequence_count: Set(form_data.trigger_limit_sequence_count),
        ..Default::default()
    }
    .update(db)
    .await
}

pub async fn delete_by_id(db: &DbConn, id: i32) -> Result<DeleteResult, DbErr> {
    let sensor: model::ActiveModel = Sensor::find_by_id(id)
        .one(db)
        .await?
        .ok_or(DbErr::Custom("Cannot find sensor.".to_owned()))
        .map(Into::into)?;

    sensor.delete(db).await
}
