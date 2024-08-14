use crate::admin_user;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "session")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: i32,
    pub admin_user_id: i32,
    pub cookieid: String,
    pub expire_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    AdminUser,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::AdminUser => Entity::belongs_to(admin_user::model::Entity)
                .from(Column::AdminUserId)
                .to(admin_user::model::Column::Id)
                .into(),
        }
    }
}

impl Related<admin_user::model::Entity> for Entity {
    fn to() -> RelationDef {
        admin_user::model::Relation::Session.def().rev()
    }
}

impl ActiveModelBehavior for ActiveModel {}
