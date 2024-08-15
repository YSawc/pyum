use pyum::web::routes;
use sea_orm::Database;
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // let file_appender = RollingFileAppender::new(Rotation::DAILY, "/var/log/pyum", "logfile.log");
    // let subscriber = tracing_subscriber::registry()
    //     .with(
    //         tracing_subscriber::EnvFilter::try_from_default_env()
    //             .unwrap_or_else(|_| "tokio_mysql=debug".into()),
    //     )
    //     .with(tracing_subscriber::fmt::layer())
    //     .init();

    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL is not set");
    let conn = Database::connect(db_url)
        .await
        .expect("database connection failed.");
    let app = routes::router(conn).await;

    // run it with hyper
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app)
        .await
        .expect("something is go wrong when axum server is launching");

    Ok(())
}
