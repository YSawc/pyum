use sea_orm::*;

use crate::models::sensor_purpose;

use super::model::{self, Entity as Sensor};

pub async fn create(
    db: &DbConn,
    form_data: model::Model,
    device_id: i32,
) -> Result<model::ActiveModel, DbErr> {
    model::ActiveModel {
        device_id: Set(device_id),
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
    let res = Sensor::find_by_id(id)
        .find_with_related(sensor_purpose::model::Entity)
        .all(db)
        .await?;
    Ok(res
        .first()
        .map(|elm| (elm.0.to_owned(), elm.1.first().unwrap().to_owned()))
        .unwrap())
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
