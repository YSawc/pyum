use crate::session;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "admin_user")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: i32,
    pub name: String,
    #[sea_orm(ignore)]
    pub password: String,
    #[serde(skip_deserializing)]
    pub encrypted_password: String,
    #[serde(skip_deserializing)]
    pub created_at: DateTime,
    #[serde(skip_deserializing)]
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Session,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Session => Entity::has_many(session::model::Entity)
                .from(Column::Id)
                .to(session::model::Column::AdminUserId)
                .into(),
        }
    }
}

impl Related<session::model::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Session.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[cfg(test)]
mod tests {
    use crate::admin_user;
    use sea_orm::{DbBackend, EntityTrait, JoinType, QuerySelect, QueryTrait, RelationTrait};
    #[tokio::test]
    async fn query_string_for_relation_of_session_and_admin_user() {
        assert_eq!(
        admin_user::model::Entity::find()
            .join(JoinType::LeftJoin, admin_user::model::Relation::Session.def())
            .build(DbBackend::MySql)
            .to_string(),
        [
            "SELECT `admin_user`.`id`, `admin_user`.`name`, `admin_user`.`encrypted_password`, `admin_user`.`created_at`, `admin_user`.`updated_at`",
            "FROM `admin_user`",
            "LEFT JOIN `session` ON `admin_user`.`id` = `session`.`admin_user_id`"
        ]
        .join(" ")
        );
    }
}
