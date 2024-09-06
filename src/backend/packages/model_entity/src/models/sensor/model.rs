use crate::models::{device, sensor_purpose};
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "sensor")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: i32,
    pub device_id: i32,
    pub sensor_purpose_id: i32,
    pub trigger_limit_val: i32,
    pub trigger_limit_sequence_count: Option<i32>,
    #[serde(skip_deserializing)]
    pub created_at: DateTime,
    #[serde(skip_deserializing)]
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Device,
    SensorPurpose,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Device => Entity::belongs_to(device::model::Entity)
                .from(Column::DeviceId)
                .to(device::model::Column::Id)
                .into(),
            Self::SensorPurpose => Entity::belongs_to(sensor_purpose::model::Entity)
                .from(Column::SensorPurposeId)
                .to(sensor_purpose::model::Column::Id)
                .into(),
        }
    }
}

impl Related<device::model::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Device.def()
    }
}

impl Related<sensor_purpose::model::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::SensorPurpose.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
