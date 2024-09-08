use crate::models::sensor_purpose;

use super::{model, model::Entity as Sensor};
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
}
