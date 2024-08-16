pub mod prepare;
use model_entity::models::oauth2_client_secret::{
    model::{Entity, Model},
    query::Query,
};
use sea_orm::{prelude::Expr, EntityOrSelect, EntityTrait, QueryFilter, QueryTrait};

// cargo test --features mock
#[cfg(feature = "mock")]
#[tokio::test]
async fn main() {
    use model_entity::models::{
        admin_user::model::Model,
        oauth2_client_secret::{self, model::Entity},
    };
    use sea_orm::DatabaseBackend;

    {
        let db = &prepare::prepare_mock_db();
        // let oauth2_client_secret = Entity::find().find_by_id(db, 1).await.unwrap().unwrap();

        // assert_eq!(oauth2_client_secret.client_id, 1);
    }
    // {
    //     let db = &prepare::prepare_mock_db();
    //     let oauth2_client_secrets = Query::is_not_deleted(db).await.unwrap();
    //     let client_ids: Vec<i32> = oauth2_client_secrets
    //         .into_iter()
    //         .map(|e| e.client_id)
    //         .collect();

    //     assert_eq!(client_ids, [1, 3, 4]);
    // }
}
