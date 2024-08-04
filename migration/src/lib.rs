pub use sea_orm_migration::prelude::*;

mod m20240801_000001_create_devices_table;
mod m20240803_063030_create_admin_users_table;
mod m20240803_090516_add_created_at_and_updated_at_column_to_devices;
mod m20240803_221344_create_oauth2_client_secret;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240801_000001_create_devices_table::Migration),
            Box::new(m20240803_063030_create_admin_users_table::Migration),
            Box::new(m20240803_090516_add_created_at_and_updated_at_column_to_devices::Migration),
            Box::new(m20240803_221344_create_oauth2_client_secret::Migration),
        ]
    }
}
