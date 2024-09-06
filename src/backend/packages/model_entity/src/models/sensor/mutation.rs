use sea_orm::*;

use super::model::{self, Entity as SensorPurpose};

pub async fn create(
    db: &DbConn,
    form_data: model::Model,
    uid: i32,
) -> Result<model::ActiveModel, DbErr> {
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
