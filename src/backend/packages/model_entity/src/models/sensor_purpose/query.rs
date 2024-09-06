use super::{model, model::Entity as SensorPurpose};
use sea_orm::*;

pub struct SensorPurposeQuery;

impl SensorPurposeQuery {
    pub async fn find_in_page(
        db: &DbConn,
        uid: i32,
        page: u64,
        devices_per_page: u64,
    ) -> Result<(Vec<model::Model>, u64), DbErr> {
        let paginator = SensorPurpose::find()
            .filter(model::Column::AdminUserId.eq(uid))
            .order_by_asc(model::Column::Id)
            .paginate(db, devices_per_page);
        let num_pages = paginator.num_pages().await?;

        paginator
            .fetch_page(page - 1)
            .await
            .map(|sensor_purposes| (sensor_purposes, num_pages))
    }
}
