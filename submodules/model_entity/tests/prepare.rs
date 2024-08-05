use model_entity::oauth2_client_secret::model::Model;
use rand::{distributions::Alphanumeric, Rng};
use sea_orm::*;

// cargo test --features mock

#[cfg(feature = "mock")]
pub fn prepare_mock_db() -> DatabaseConnection {
    let client_secret: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(64)
        .map(char::from)
        .collect();

    MockDatabase::new(DatabaseBackend::MySql)
        .append_query_results([[Model {
            client_id: 1,
            client_secret,
            is_deleted: false,
        }]])
        .into_connection()
}
