use super::{model, model::Entity as Sensor};
use crate::models::{device, sensor_purpose};
use sea_orm::*;

pub struct SensorQuery;

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
    ) -> Result<
        Vec<(
            device::model::Model,
            Vec<(super::model::Model, sensor_purpose::model::Model)>,
        )>,
        DbErr,
    > {
        let devices_with_sensor = device::model::Entity::find()
            .find_with_related(super::model::Entity)
            .limit(models_per_page)
            .offset(models_per_page * (page - 1))
            .all(db)
            .await
            .unwrap();
        let mut devices_and_related_sensors_and_purposes = Vec::new();
        for device_with_sensor in devices_with_sensor {
            let sensors = device_with_sensor.1;
            let mut sensors_and_purposes = Vec::new();
            for sensor in sensors {
                let purpose = sensor
                    .find_related(sensor_purpose::model::Entity)
                    .one(db)
                    .await
                    .unwrap()
                    .unwrap();
                sensors_and_purposes.push((sensor, purpose));
            }
            devices_and_related_sensors_and_purposes
                .push((device_with_sensor.0, sensors_and_purposes))
        }

        Ok(devices_and_related_sensors_and_purposes)
    }
}
