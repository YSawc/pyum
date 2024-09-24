use super::{model, model::Entity as Sensor};
use crate::models::sensor_purpose;
use sea_orm::{Statement, *};
use serde::Serialize;

pub struct SensorQuery;

#[derive(Debug, FromQueryResult, Serialize)]
pub struct FindInPageResult {
    device_id: i32,
    device_name: String,
    device_image: String,
    sensor_ids: String,                    // GROUP_CONCAT
    sensor_purpose_ids: String,            // GROUP_CONCAT
    trigger_limit_vals: String,            // GROUP_CONCAT
    trigger_limit_sequence_counts: String, // GROUP_CONCAT
    sensor_event_ids: String,              // GROUP_CONCAT
    sensor_event_descriptions: String,     // GROUP_CONCAT
    sensor_event_images: String,           // GROUP_CONCAT
}

impl SensorQuery {
    pub async fn find_in_page(
        db: &DbConn,
        device_id: i32,
        page: u64,
        models_per_page: u64,
    ) -> Result<Vec<(model::Model, sensor_purpose::model::Model)>, DbErr> {
        let paginator = Sensor::find()
            .filter(model::Column::DeviceId.eq(device_id))
            .find_with_related(sensor_purpose::model::Entity)
            .order_by_asc(model::Column::Id)
            .limit(models_per_page)
            .offset(models_per_page * (page - 1));

        let res = paginator.all(db).await?;
        Ok(res
            .iter()
            .map(|elm| (elm.0.to_owned(), elm.1.first().unwrap().to_owned()))
            .collect::<Vec<_>>())
    }

    pub async fn find_devices_with_related_sensor_and_purpose(
        db: &DbConn,
        page: u64,
        models_per_page: u64,
    ) -> Result<Vec<FindInPageResult>, DbErr> {
        let offset_str = models_per_page * (page - 1);
        let custom_res = FindInPageResult::find_by_statement(Statement::from_sql_and_values(
            db.get_database_backend(),
            format!(
                r#"
SELECT
    device.id AS device_id,
    device.name AS device_name,
    device.image AS device_image,
    GROUP_CONCAT(sensor.id) AS sensor_ids,
    GROUP_CONCAT(sensor.sensor_purpose_id) AS sensor_purpose_ids,
    GROUP_CONCAT(sensor.trigger_limit_val) AS trigger_limit_vals,
    GROUP_CONCAT(sensor.trigger_limit_sequence_count) AS trigger_limit_sequence_counts,
    GROUP_CONCAT(sensor_purpose.sensor_event_id) AS sensor_event_ids,
    GROUP_CONCAT(sensor_event.description) AS sensor_event_descriptions,
    GROUP_CONCAT(sensor_event.image) AS sensor_event_images
FROM
    device
LEFT JOIN sensor
    ON sensor.device_id = device.id
LEFT JOIN sensor_purpose
    ON sensor_purpose.id = sensor.sensor_purpose_id
LEFT JOIN sensor_event
ON sensor_event.id = sensor_purpose.sensor_event_id
LIMIT {} OFFSET {}"#,
                models_per_page, offset_str
            ),
            [],
        ))
        .all(db)
        .await
        .unwrap();

        Ok(custom_res)
    }
}
