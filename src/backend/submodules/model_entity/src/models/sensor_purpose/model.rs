use crate::models::{admin_user, sensor};
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "sensor_purpose")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: i32,
    #[serde(skip_serializing)]
    #[serde(skip_deserializing)]
    pub admin_user_id: i32,
    pub description: String,
    #[serde(skip_deserializing)]
    pub created_at: DateTime,
    #[serde(skip_deserializing)]
    pub updated_at: DateTime,
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
            Self::Sensor => Entity::has_many(sensor::model::Entity)
                .from(Column::Id)
                .to(sensor::model::Column::SensorPurposeId)
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
