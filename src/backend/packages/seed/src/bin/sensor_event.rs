use model_entity::models::sensor_event;
use sea_orm::Database;

// cargo run --bin sensor_event
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL is not set");
    let db = Database::connect(db_url)
        .await
        .expect("database connection failed.");

    sensor_event::mutation::delete_all(&db).await?;
    sensor_event::mutation::create(&db,
        "none".to_string(),
        "https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcTqOitnmTV5hZY6F_B9EhrHiRSKVjU0FD30Ig&s".to_string(),
        ).await?;
    sensor_event::mutation::create(&db,
        "line notify".to_string(),
        "https://upload.wikimedia.org/wikipedia/commons/thumb/4/41/LINE_logo.svg/1200px-LINE_logo.svg.png".to_string(),
        ).await?;

    Ok(())
}
