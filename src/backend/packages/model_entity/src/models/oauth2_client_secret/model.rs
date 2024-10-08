use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

use crate::models::admin_user;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "oauth2_client_secret")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub client_id: i32,
    pub admin_user_id: i32,
    pub client_secret: String,
    #[sea_orm(nullable)]
    pub deleted_at: Option<DateTime>,
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
        Relation::AdminUser.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
