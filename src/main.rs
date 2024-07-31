use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use sea_orm::{ConnectOptions, Database, DatabaseConnection};

#[derive(Clone)]
struct AppState {
    conn: DatabaseConnection,
}

impl AppState {
    fn new(conn: DatabaseConnection) -> Self {
        Self { conn }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "tokio_mysql=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL is not set");
    let conn = Database::connect(db_url)
        .await
        .expect("database connection failed.");

    let state = AppState::new(conn);
    let app = Router::new()
        .route("/api/health_check", get(health_check_handler))
        // .route("/device/list", get(list_devices))
        // .route("/device/list", post(create_device))
        .with_state(state);

    // run it with hyper
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app)
        .await
        .expect("something is go wrong when axum server is launching");

    Ok(())
}

pub async fn health_check_handler() -> impl IntoResponse {
    const MESSAGE: &str = "API Services";

    let json_response = serde_json::json!({
        "status": "ok",
        "message": MESSAGE
    });

    Json(json_response)
}

// async fn list_devices(
//     State(pool): State<deadpool_diesel::mysql::Pool>,
// ) -> Result<Json<Vec<Device>>, (StatusCode, String)> {
//     let conn = pool.get().await.map_err(internal_error)?;
//     let res = conn
//         .interact(|conn| devices::table.select(Device::as_select()).load(conn))
//         .await
//         .map_err(internal_error)?
//         .map_err(internal_error)?;
//     Ok(Json(res))
// }
//
// async fn create_device(
//     State(pool): State<deadpool_diesel::mysql::Pool>,
//     Json(new_devise): Json<NewDevice>,
// ) -> Result<impl IntoResponse, (StatusCode, String)> {
//     // ) -> Result<Json<Device>, (StatusCode, String)> {
//     let conn = pool.get().await.map_err(internal_error)?;
//     let res = conn
//         .interact(|conn| {
//             diesel::insert_into(devices::table)
//                 .values(new_devise)
//                 .execute(conn)
//                 .unwrap();
//             // .get_result(conn)
//         })
//         .await
//         .map_err(internal_error)?;
//
//     let json_response = serde_json::json!({
//         "status": "ok",
//         "message": "insert successed."
//     });
//
//     Ok(Json(json_response))
// }

// Utility function for mapping any error into a `500 Internal Server Error`
// response.
fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
