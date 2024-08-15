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
        println!("res: {:?}", res);
        assert_eq!(res.status(), StatusCode::OK);
    }
}
