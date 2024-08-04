use sea_orm::*;

// cargo test --features mock

#[cfg(feature = "mock")]
pub fn prepare_mock_db() -> DatabaseConnection {
    MockDatabase::new(DatabaseBackend::MySql).into_connection()
}
