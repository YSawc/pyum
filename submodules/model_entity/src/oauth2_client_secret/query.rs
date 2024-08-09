use super::model::{self, Entity, Model};
use sea_orm::*;

pub struct Query;

impl Query {
    pub async fn find_by_id(db: &DbConn, id: i32) -> Result<Option<Model>, DbErr> {
        Entity::find_by_id(id).one(db).await
    }

    pub async fn is_not_deleted(db: &DbConn) -> Result<Vec<Model>, DbErr> {
        let q = Entity::find().filter(model::Column::IsDeletedAt.is_null());
        println!("{:?}", q.build(DatabaseBackend::MySql).to_string());
        q.all(db).await
    }
}
