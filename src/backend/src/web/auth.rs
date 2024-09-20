use super::middleware::{buffer_and_print, AppState};
use axum::{
    body::Body,
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};
use model_entity::models::oauth2_client_secret;
use tower_sessions::Session;

pub async fn check_session_id(
    session: Session,
    req: Request,
    next: Next,
) -> Result<impl IntoResponse, StatusCode> {
    match session.get_value("uid").await.unwrap() {
        Some(_uid) => {
            let (parts, body) = req.into_parts();
            let bytes = buffer_and_print("request", body).await?;
            let req = Request::from_parts(parts, Body::from(bytes));
            let res = next.run(req).await;
            let (parts, body) = res.into_parts();
            let bytes = buffer_and_print("response", body).await?;
            let res = Response::from_parts(parts, Body::from(bytes));

            Ok(res.into_response())
        }
        None => Err(StatusCode::UNAUTHORIZED),
    }
}

pub async fn check_oauth_secret(
    state: State<AppState>,
    session: Session,
    req: Request,
    next: Next,
) -> Result<impl IntoResponse, StatusCode> {
    let (parts, body) = req.into_parts();
    match parts.headers.get("client-secret") {
        Some(client_secret) => {
            let client_secret_str = client_secret.to_str().unwrap().to_string();
            let oauth2_client_secret = oauth2_client_secret::mutation::find_by_oauth_secret(
                &state.conn,
                client_secret_str,
            )
            .await
            .unwrap()
            .unwrap();
            session
                .insert("uid", oauth2_client_secret.admin_user_id)
                .await
                .unwrap();
            let bytes = buffer_and_print("request", body).await?;
            let req = Request::from_parts(parts, Body::from(bytes));
            let res = next.run(req).await;
            let (parts, body) = res.into_parts();
            let bytes = buffer_and_print("response", body).await?;
            let res = Response::from_parts(parts, Body::from(bytes));

            Ok(res.into_response())
        }
        None => Err(StatusCode::UNAUTHORIZED),
    }
}

// In executing with muti thread, model deletion is not works expected, so thread should be only one.
// run below test: cargo test -- --nocapture --test-threads=1
#[cfg(test)]
mod tests {
    use crate::web::routes;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
        Json, Router,
    };
    use model_entity::models::{admin_user, session};
    use rstest::rstest;
    use sea_orm::{Database, DatabaseConnection};
    use std::env;
    use tower::ServiceExt;

    async fn prepare_db_connection() -> DatabaseConnection {
        let test_db_url = std::env::var("TEST_DATABASE_URL").expect("TEST_DATABASE_URL is not set");
        env::set_var("DATABASE_URL", test_db_url);
        let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL is not set");
        Database::connect(db_url)
            .await
            .expect("database connection failed.")
    }

    async fn prepare_router(conn: DatabaseConnection) -> Router {
        routes::router(conn).await
    }

    async fn delete_related_models(conn: &DatabaseConnection) {
        admin_user::mutation::delete_all(conn)
            .await
            .expect("failed to delete all admin user");
        session::mutation::delete_all(conn)
            .await
            .expect("failed to delete all session");
    }

    #[tokio::test]
    async fn api_health_check() {
        let conn = prepare_db_connection().await;
        let req = Request::builder()
            .uri("/api/health_check")
            .body(Body::empty())
            .unwrap();
        let res = prepare_router(conn).await.oneshot(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn request_nonprepared_route() {
        let conn = prepare_db_connection().await;
        let req = Request::builder()
            .uri("/not_exists_route")
            .body(Body::empty())
            .unwrap();
        let res = prepare_router(conn).await.oneshot(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    #[rstest]
    #[case(false, true)]
    #[case(true, false)]
    async fn miss_passing_header_uid_for_protected_route(
        #[case] missing_uid: bool,
        #[case] missing_session_id: bool,
    ) {
        let conn = prepare_db_connection().await;
        let mut custom_req = Request::builder().uri("/device");
        if !missing_uid {
            delete_related_models(&conn).await;
            admin_user::mutation::seed(&conn)
                .await
                .expect("failed to seed admin_user");
            let all_users = admin_user::mutation::find_all(&conn)
                .await
                .expect("failed to find all admin_user");
            let first_user = all_users.first().unwrap();
            custom_req = custom_req.header("uid", first_user.id);
        }
        if !missing_session_id {
            custom_req = custom_req.header("cookie_id", "Bar");
        }
        let req = custom_req.body(Body::empty()).unwrap();
        let res = prepare_router(conn).await.oneshot(req).await.unwrap();
        let expect_code = if missing_uid || missing_session_id {
            StatusCode::UNAUTHORIZED
        } else {
            StatusCode::OK
        };
        assert_eq!(res.status(), expect_code);
    }

    #[tokio::test]
    async fn passing_not_exist_uid() {
        let conn = prepare_db_connection().await;
        delete_related_models(&conn).await;
        admin_user::mutation::seed(&conn)
            .await
            .expect("failed to seed admin_user");
        let req = Request::builder()
            .uri("/device")
            .header("uid", 0)
            .header("cookie_id", "Bar")
            .body(Body::empty())
            .unwrap();
        let res = prepare_router(conn).await.oneshot(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn passing_not_exist_cookie_id() {
        let conn = prepare_db_connection().await;
        delete_related_models(&conn).await;
        admin_user::mutation::seed_with_unexpired_session(&conn)
            .await
            .expect("failed to seed admin_user");
        let all_users = admin_user::mutation::find_all(&conn)
            .await
            .expect("failed to find all admin_user");
        let first_user = all_users.first().unwrap();
        let req = Request::builder()
            .uri("/device")
            .header("uid", first_user.id)
            .header("cookie_id", "Bar")
            .body(Body::empty())
            .unwrap();
        let res = prepare_router(conn).await.oneshot(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::UNAUTHORIZED);
    }
}
