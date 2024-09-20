use model_entity::models::oauth2_client_secret;
use sea_orm::Database;
use std::env;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL is not set");
    let db = Database::connect(db_url)
        .await
        .expect("database connection failed.");

    let args: Vec<String> = env::args().collect();
    let admin_user_id = &args[1];
    if admin_user_id.is_empty() {
        panic!("please pass uid.");
    }
    oauth2_client_secret::mutation::logic_delete(&db, admin_user_id.parse::<i32>().unwrap())
        .await
        .expect("failed to legic delete all oauth2 client secrets");

    oauth2_client_secret::mutation::create_oauth2_client_secret(
        &db,
        admin_user_id.parse::<i32>().unwrap(),
    )
    .await
    .expect("failed to create oauth2 client secret");

    let result = oauth2_client_secret::mutation::get_by_id(&db)
        .await
        .expect("failed to create oauth2 client secret");

    match result {
        Some(created_secret) => {
            println!("generated new client secret. Below is the secret code.");
            println!("client_id: {}", created_secret.client_id);
            println!("client_secret: {}", created_secret.client_secret);
        }
        None => unimplemented!("Something is wrong for creating client secret"),
    }

    Ok(())
}
