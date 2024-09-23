use sea_orm::*;

use super::model::{self, Entity as SensorEvent};

pub async fn create(
    db: &DbConn,
    description: String,
    image: String,
) -> Result<model::ActiveModel, DbErr> {
    model::ActiveModel {
        description: Set(description),
        image: Set(image),
        ..Default::default()
    }
    .save(db)
    .await
}

pub async fn get_by_id(db: &DbConn, id: i32) -> Result<Option<model::Model>, DbErr> {
    SensorEvent::find_by_id(id).one(db).await
}

pub async fn delete_all(db: &DbConn) -> Result<DeleteResult, DbErr> {
    SensorEvent::delete_many().exec(db).await
}
