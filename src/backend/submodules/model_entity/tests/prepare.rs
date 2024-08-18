use model_entity::models::oauth2_client_secret;
use rand::{distributions::Alphanumeric, Rng};
use sea_orm::*;

fn prepare_client_secret() -> String {
    let client_secret: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(64)
        .map(char::from)
        .collect();

    client_secret
}

#[cfg(feature = "mock")]
pub fn prepare_mock_db() -> DatabaseConnection {
    use sea_orm::MockDatabase;

    MockDatabase::new(DatabaseBackend::MySql)
        .append_query_results([[
            oauth2_client_secret::model::Model {
                client_id: 1,
                client_secret: prepare_client_secret(),
                deleted_at: None,
            },
            oauth2_client_secret::model::Model {
                client_id: 2,
                client_secret: prepare_client_secret(),
                deleted_at: Some(
                    NaiveDate::from_ymd_opt(2000, 1, 1)
                        .unwrap()
                        .and_hms_opt(0, 0, 0)
                        .unwrap(),
                ),
            },
            oauth2_client_secret::model::Model {
                client_id: 3,
                client_secret: prepare_client_secret(),
                deleted_at: None,
            },
            oauth2_client_secret::model::Model {
                client_id: 4,
                client_secret: prepare_client_secret(),
                deleted_at: None,
            },
        ]])
        .into_connection()
}
