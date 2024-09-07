use sea_orm::*;

use super::model::{self};

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
