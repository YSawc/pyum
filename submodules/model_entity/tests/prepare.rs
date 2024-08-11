use model_entity::oauth2_client_secret::model::Model;
use rand::{distributions::Alphanumeric, Rng};
use sea_orm::*;
use sqlx::types::chrono::NaiveDate;

fn prepare_client_secret() -> String {
    let client_secret: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(64)
        .map(char::from)
        .collect();

    client_secret
}

pub fn prepare_mock_db() -> DatabaseConnection {
    MockDatabase::new(DatabaseBackend::MySql)
        .append_query_results([[
            Model {
                client_id: 1,
                client_secret: prepare_client_secret(),
                deleted_at: None,
            },
            Model {
                client_id: 2,
                client_secret: prepare_client_secret(),
                deleted_at: Some(
                    NaiveDate::from_ymd_opt(2000, 1, 1)
                        .unwrap()
                        .and_hms_opt(0, 0, 0)
                        .unwrap(),
                ),
            },
            Model {
                client_id: 3,
                client_secret: prepare_client_secret(),
                deleted_at: None,
            },
            Model {
                client_id: 4,
                client_secret: prepare_client_secret(),
                deleted_at: None,
            },
        ]])
        .into_connection()
}
