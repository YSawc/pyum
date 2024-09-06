use std::env;

use sea_orm_migration::prelude::*;

#[async_std::main]
async fn main() {
    if cfg!(feature = "for_test") {
        let db_url = std::env::var("TEST_DATABASE_URL").expect("TEST_DATABASE_URL is not set");
        env::set_var("DATABASE_URL", db_url);
    }
    cli::run_cli(migration::Migrator).await;
}
