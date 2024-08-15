use axum::{
    body::Body,
    extract::Request,
    http::{Response, StatusCode},
    middleware::Next,
    response::IntoResponse,
};

use crate::web::middleware::buffer_and_print;

pub async fn check_session_id(
    req: Request,
    next: Next,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let (parts, body) = req.into_parts();
    let maybe_uid = parts.headers.get("uid");
    let maybe_cookie_id = parts.headers.get("cookie_id");
    if maybe_uid.is_none() || maybe_cookie_id.is_none() {
        return Err((
            StatusCode::EXPECTATION_FAILED,
            "uid or cookie_id is not set.".to_string(),
        ));
    }
    let bytes = buffer_and_print("request", body).await?;
    let req = Request::from_parts(parts, Body::from(bytes));
    let res = next.run(req).await;
    let (parts, body) = res.into_parts();
    let bytes = buffer_and_print("response", body).await?;
    let res = Response::from_parts(parts, Body::from(bytes));

    Ok(res)
}

#[cfg(test)]
mod tests {
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use tower::ServiceExt;

    use crate::web::routes;
    use std::env;

    #[tokio::test]
    async fn request_hello() {
        let db_url = std::env::var("TEST_DATABASE_URL").expect("TEST_DATABASE_URL is not set");
        env::set_var("DATABASE_URL", db_url);
        let req = Request::builder()
            .uri("/hello")
            .body(Body::empty())
            .unwrap();
        let res = routes::router().await.oneshot(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn request_nonprepared_route() {
        let db_url = std::env::var("TEST_DATABASE_URL").expect("TEST_DATABASE_URL is not set");
        env::set_var("DATABASE_URL", db_url);
        let req = Request::builder()
            .uri("/not_exists_route")
            .body(Body::empty())
            .unwrap();
        let res = routes::router().await.oneshot(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn missing_header_uid_for_protected_route() {
        let db_url = std::env::var("TEST_DATABASE_URL").expect("TEST_DATABASE_URL is not set");
        env::set_var("DATABASE_URL", db_url);
        let req = Request::builder()
            .uri("/device/")
            .header("cookie_id", "Bar")
            .body(Body::empty())
            .unwrap();
        let res = routes::router().await.oneshot(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::EXPECTATION_FAILED);
    }

    #[tokio::test]
    async fn missing_header_cookie_id_for_protected_route() {
        let db_url = std::env::var("TEST_DATABASE_URL").expect("TEST_DATABASE_URL is not set");
        env::set_var("DATABASE_URL", db_url);
        let req = Request::builder()
            .uri("/device/")
            .header("uid", "Bar")
            .body(Body::empty())
            .unwrap();
        let res = routes::router().await.oneshot(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::EXPECTATION_FAILED);
    }

    #[tokio::test]
    async fn meet_header_for_protected_route() {
        let db_url = std::env::var("TEST_DATABASE_URL").expect("TEST_DATABASE_URL is not set");
        env::set_var("DATABASE_URL", db_url);
        let req = Request::builder()
            .uri("/device/")
            .header("uid", "Bar")
            .header("cookie_id", "Bar")
            .body(Body::empty())
            .unwrap();
        let res = routes::router().await.oneshot(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::OK);
    }
}
