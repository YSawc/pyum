use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

use crate::models::{admin_user, sensor};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "device")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: i32,
    #[serde(skip_serializing)]
    #[serde(skip_deserializing)]
    pub admin_user_id: i32,
    pub name: String,
    #[sea_orm(nullable)]
    pub image: String,
    #[sea_orm(nullable)]
    #[serde(skip_serializing)]
    #[serde(skip_deserializing)]
    pub deleted_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    AdminUser,
    Sensor,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::AdminUser => Entity::belongs_to(admin_user::model::Entity)
                .from(Column::AdminUserId)
                .to(admin_user::model::Column::Id)
                .into(),
            Self::Sensor => Entity::belongs_to(sensor::model::Entity)
                .from(Column::Id)
                .to(sensor::model::Column::DeviceId)
                .into(),
        }
    }
}

impl Related<admin_user::model::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AdminUser.def()
    }
}

impl Related<sensor::model::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Sensor.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
