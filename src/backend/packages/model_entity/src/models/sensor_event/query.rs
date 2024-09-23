use super::{model, model::Entity as SensorPurpose};
use sea_orm::*;

pub struct SensorEventQuery;

impl SensorEventQuery {
    pub async fn find(db: &DbConn) -> Result<Vec<model::Model>, DbErr> {
        SensorPurpose::find()
            .order_by_asc(model::Column::Id)
            .all(db)
            .await
    }
}
