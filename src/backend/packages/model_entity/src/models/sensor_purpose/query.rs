use crate::models::{capture, sensor, sensor_event};

use super::{model, model::Entity as SensorPurpose};
use sea_orm::*;

pub struct SensorPurposeQuery;

impl SensorPurposeQuery {
    pub async fn find_in_page(
        db: &DbConn,
        uid: i32,
        page: u64,
        models_per_page: u64,
    ) -> Result<(Vec<(model::Model, Option<sensor_event::model::Model>)>, u64), DbErr> {
        let paginator = SensorPurpose::find()
            .filter(model::Column::AdminUserId.eq(uid))
            .find_also_related(sensor_event::model::Entity)
            .order_by_asc(model::Column::Id)
            .paginate(db, models_per_page);
        let num_pages = paginator.num_pages().await?;

        paginator
            .fetch_page(page - 1)
            .await
            .map(|sensor_purposes| (sensor_purposes, num_pages))
    }

    pub async fn find_with_related_sensor_and_capture(
        db: &DbConn,
        sensor_purpose_id: i32,
    ) -> Result<
        (
            super::model::Model,
            Vec<(sensor::model::Model, Vec<capture::model::Model>)>,
        ),
        DbErr,
    > {
        let sensors_purpose_with_sensor = super::model::Entity::find()
            .filter(super::model::Column::Id.eq(sensor_purpose_id))
            .find_with_related(sensor::model::Entity)
            .all(db)
            .await
            .unwrap();
        let sensor_purpose_with_sensor = sensors_purpose_with_sensor.first().unwrap();
        let mut sensors_with_captures: Vec<(sensor::model::Model, Vec<capture::model::Model>)> =
            Vec::new();
        for sensor in &sensor_purpose_with_sensor.1 {
            let captures = sensor
                .find_related(capture::model::Entity)
                .all(db)
                .await
                .unwrap();
            sensors_with_captures.push((sensor.to_owned(), captures));
        }

        Ok((
            sensor_purpose_with_sensor.0.to_owned(),
            sensors_with_captures,
        ))
    }
}
