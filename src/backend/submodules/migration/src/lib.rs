pub use sea_orm_migration::prelude::*;

mod m20240803_063030_create_admin_user_table;
mod m20240803_064001_create_device_table;
mod m20240803_090516_add_created_at_and_updated_at_column_to_device;
mod m20240803_221344_create_oauth2_client_secret;
mod m20240814_125552_create_session;
mod m20240903_212851_create_sensor_purpose;
mod m20240904_121407_create_sensor;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240803_063030_create_admin_user_table::Migration),
            Box::new(m20240803_064001_create_device_table::Migration),
            Box::new(m20240803_090516_add_created_at_and_updated_at_column_to_device::Migration),
            Box::new(m20240803_221344_create_oauth2_client_secret::Migration),
            Box::new(m20240814_125552_create_session::Migration),
            Box::new(m20240903_212851_create_sensor_purpose::Migration),
            Box::new(m20240904_121407_create_sensor::Migration),
        ]
    }
}
