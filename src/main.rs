use axum::{
    body::Body,
    extract::{MatchedPath, Path, Request, State},
    http::{header, HeaderMap, HeaderValue, StatusCode},
    middleware,
    response::{Html, IntoResponse, Json, Redirect},
    routing::{get, post},
    Form, Router,
};
use model_entity::admin_user;
use pyum::{
    middleware::{print_request_response, AppState},
    web::protected,
};
use std::net::SocketAddr;
use tower_cookies::CookieManagerLayer;
use tower_http::trace::TraceLayer;

use sea_orm::Database;

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

    let state = AppState::new(conn);
    let app = Router::new()
        .merge(protected::router())
        .route("/api/health_check", get(health_check_handler))
        .route("/hello", get(hello))
        .route("/assets/images/:path", get(get_image_asset))
        .route("/admin_user/new", get(get_create_admin_user))
        .route("/admin_user/new", post(post_create_admin_user))
        .route("/admin_user/login", get(get_login_admin_user))
        .route("/admin_user/login", post(post_login_admin_user))
        .layer(CookieManagerLayer::new())
        .layer(
            TraceLayer::new_for_http()
                // Create our own span for the request and include the matched path. The matched
                // path is useful for figuring out which handler the request was routed to.
                .make_span_with(|req: &Request| {
                    let method = req.method();
                    let uri = req.uri();

                    // axum automatically adds this extension.
                    let matched_path = req
                        .extensions()
                        .get::<MatchedPath>()
                        .map(|matched_path| matched_path.as_str());

                    tracing::debug_span!("request", %method, %uri, matched_path)
                })
                // By default `TraceLayer` will log 5xx responses but we're doing our specific
                // logging of errors so disable that
                .on_failure(()),
        )
        .layer(middleware::from_fn(print_request_response))
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

async fn hello(state: State<AppState>) -> Result<Html<String>, (StatusCode, &'static str)> {
    let ctx = tera::Context::new();
    let body = state
        .templates
        .render("pages/hello.html", &ctx)
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Template error"))?;

    Ok(Html(body))
}

pub async fn get_image_asset(Path(path): Path<String>) -> impl IntoResponse {
    let image_path = format!("src/assets/images/{}", path);
    let file = match tokio::fs::File::open(image_path.to_owned()).await {
        Ok(file) => file,
        Err(err) => return Err((StatusCode::NOT_FOUND, format!("File not found: {}", err))),
    };
    let content_type = match mime_guess::from_path(&image_path).first_raw() {
        Some(mime) => mime,
        None => {
            return Err((
                StatusCode::BAD_REQUEST,
                "MIME Type couldn't be determined".to_string(),
            ))
        }
    };
    let stream = tokio_util::io::ReaderStream::new(file);
    let body = Body::from_stream(stream);

    let mut headermap = HeaderMap::new();
    headermap.insert(
        header::CONTENT_TYPE,
        HeaderValue::from_str(content_type).expect("header content type must be string."),
    );
    headermap.insert(
        header::CONTENT_DISPOSITION,
        HeaderValue::from_str("inline").expect("header content of diposition must be string."),
    );

    Ok((headermap, body))
}

async fn get_create_admin_user(
    state: State<AppState>,
) -> Result<Html<String>, (StatusCode, &'static str)> {
    let ctx = tera::Context::new();
    let body = state
        .templates
        .render("pages/admin_user/new.html", &ctx)
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Template error"))?;

    Ok(Html(body))
}

async fn post_create_admin_user(
    state: State<AppState>,
    Form(admin_user): Form<admin_user::model::Model>,
) -> Result<Redirect, (StatusCode, &'static str)> {
    admin_user::mutation::create(&state.conn, admin_user)
        .await
        .expect("failed to create admin user.");

    Ok(Redirect::to("/device/"))
}

async fn get_login_admin_user(
    state: State<AppState>,
) -> Result<Html<String>, (StatusCode, &'static str)> {
    let ctx = tera::Context::new();
    let body = state
        .templates
        .render("pages/admin_user/login.html", &ctx)
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Template error"))?;

    Ok(Html(body))
}

async fn post_login_admin_user(
    state: State<AppState>,
    Form(admin_user): Form<admin_user::model::Model>,
) -> Result<Redirect, (StatusCode, &'static str)> {
    admin_user::mutation::find_by_name(&state.conn, admin_user)
        .await
        .unwrap();

    Ok(Redirect::to("/device/"))
}
