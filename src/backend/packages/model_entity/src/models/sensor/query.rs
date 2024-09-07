use super::{model, model::Entity as Sensor};
use sea_orm::*;

pub struct SensorQuery;

impl SensorQuery {
    pub async fn find_in_page(
        db: &DbConn,
        device_id: i32,
        page: u64,
        models_per_page: u64,
    ) -> Result<(Vec<model::Model>, u64), DbErr> {
        let paginator = Sensor::find()
            .filter(model::Column::DeviceId.eq(device_id))
            .order_by_asc(model::Column::Id)
            .paginate(db, models_per_page);
        let num_pages = paginator.num_pages().await?;

        paginator
            .fetch_page(page - 1)
            .await
            .map(|sensors| (sensors, num_pages))
    }
}
