use super::model::{Entity, Model};
use sea_orm::*;

pub struct Query;

impl Query {
    pub async fn find_post_by_id(db: &DbConn, id: i32) -> Result<Option<Model>, DbErr> {
        Entity::find_by_id(id).one(db).await
    }
}
