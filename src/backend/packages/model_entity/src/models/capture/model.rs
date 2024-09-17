use crate::models::sensor;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "capture")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: i32,
    pub sensor_id: i32,
    pub capture_val: i32,
    pub shift_digit: i32,
    #[sea_orm(nullable)]
    #[serde(skip_deserializing)]
    pub created_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Sensor,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Sensor => Entity::belongs_to(sensor::model::Entity)
                .from(Column::SensorId)
                .to(sensor::model::Column::Id)
                .into(),
        }
    }
}

impl Related<sensor::model::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Sensor.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
