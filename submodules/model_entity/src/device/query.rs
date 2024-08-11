use super::{model as device, model::Entity as Device};
use sea_orm::*;

pub struct DeviceQuery;

impl DeviceQuery {
    pub async fn find_in_page(
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
