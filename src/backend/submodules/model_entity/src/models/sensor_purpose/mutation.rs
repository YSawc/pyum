use super::{model, model::Entity as SensorPurpose};
use sea_orm::*;

pub async fn find_by_id(db: &DbConn, id: i32) -> Result<Option<model::Model>, DbErr> {
    SensorPurpose::find()
        .filter(model::Column::Id.eq(id))
        .one(db)
        .await
}
