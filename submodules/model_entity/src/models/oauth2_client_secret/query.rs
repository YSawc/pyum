use super::model::{self, Entity, Model};
use sea_orm::*;

pub struct Query;

impl Query {
    pub async fn is_not_deleted(db: &DbConn) -> Result<Vec<Model>, DbErr> {
        let q = Entity::find().filter(model::Column::DeletedAt.is_null());
        println!("{:?}", q.build(DatabaseBackend::MySql).to_string());
        q.all(db).await
    }
}
