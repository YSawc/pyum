use crate::models::sensor_purpose;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "sensor_event")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: i32,
    pub description: String,
    #[sea_orm(nullable)]
    pub image: String,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    SensorPurpose,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::SensorPurpose => Entity::belongs_to(sensor_purpose::model::Entity)
                .from(Column::Id)
                .to(sensor_purpose::model::Column::SensorEventId)
                .into(),
        }
    }
}

impl Related<sensor_purpose::model::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::SensorPurpose.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
