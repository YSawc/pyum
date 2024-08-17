use super::{model, model::Entity as Session};
use sea_orm::*;

pub async fn find_by_admin_user_id(
    db: &DbConn,
    admin_user_id: i32,
) -> Result<Vec<model::Model>, DbErr> {
    Session::find()
        .filter(model::Column::AdminUserId.eq(admin_user_id))
        .all(db)
        .await
}

pub async fn delete_all(db: &DbConn) -> Result<DeleteResult, DbErr> {
    Session::delete_many()
        .filter(model::Column::Id.is_not_null())
        .exec(db)
        .await
}
