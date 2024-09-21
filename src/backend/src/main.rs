use backend::web::routes;
use dotenvy::dotenv;
use sea_orm::Database;
use std::net::SocketAddr;
use tracing::error;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().expect(".env is not found");
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL is not set");
    let split_socket_addr = std::env::var("SOCKET_ADDR").expect("SOCKET_ADDR is not set");
    let socket_port = std::env::var("SOCKET_PORT").expect("SOCKET_PORT is not set");

    let split_socket_addr: Vec<&str> = split_socket_addr.split(".").collect();
    if split_socket_addr.to_owned().len() != 4 {
        error!("socket_addr must be like x.x.x.x");
    }
    let mut socket_addr: [u8; 4] = [0, 0, 0, 0];
    for (idx, n) in split_socket_addr.iter().enumerate() {
        socket_addr[idx] = n.parse::<u8>().unwrap();
    }
    let socket_port = socket_port.parse::<u16>().unwrap();

    let conn = Database::connect(db_url)
        .await
        .expect("database connection failed.");
    let app = routes::router(conn).await;

    // run it with hyper
    let addr = SocketAddr::from((socket_addr, socket_port));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app)
        .await
        .expect("something is go wrong when axum server is launching");

    Ok(())
}
