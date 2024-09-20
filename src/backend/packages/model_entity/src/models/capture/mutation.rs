use super::model::{self};
use sea_orm::*;

pub async fn create(db: &DbConn, form_data: model::Model) -> Result<model::ActiveModel, DbErr> {
    model::ActiveModel {
        sensor_id: Set(form_data.sensor_id),
        capture_val: Set(form_data.capture_val),
        shift_digit: Set(form_data.shift_digit),
        ..Default::default()
    }
    .save(db)
    .await
}
