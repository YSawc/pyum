use crate::model_entity::device::{model as device, model::Entity as Device};
use sea_orm::*;

pub struct DeviceQuery;

impl DeviceQuery {
    pub async fn find_device_by_id(db: &DbConn, id: i32) -> Result<Option<device::Model>, DbErr> {
        Device::find_by_id(id).one(db).await
    }

    pub async fn find_devices_in_page(
        db: &DbConn,
        page: u64,
        devices_per_page: u64,
    ) -> Result<(Vec<device::Model>, u64), DbErr> {
        let paginator = Device::find()
            .order_by_asc(device::Column::Id)
            .paginate(db, devices_per_page);
        let num_pages = paginator.num_pages().await?;

        paginator
            .fetch_page(page - 1)
            .await
            .map(|devices| (devices, num_pages))
    }
}
