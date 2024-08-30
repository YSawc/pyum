use axum::{
    body::Body,
    extract::Request,
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};
use tower_sessions::Session;

use super::{middleware::buffer_and_print, SimpleRes};

pub async fn check_session_id(
    session: Session,
    req: Request,
    next: Next,
) -> Result<impl IntoResponse, Json<SimpleRes>> {
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
        None => Err(Json(SimpleRes {
            message: "session is not valid".to_string(),
        })),
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
        Router,
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
    async fn request_hello() {
        let conn = prepare_db_connection().await;
        let req = Request::builder()
            .uri("/hello")
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
            StatusCode::SEE_OTHER
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
        assert_eq!(res.status(), StatusCode::INTERNAL_SERVER_ERROR);
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
        assert_eq!(res.status(), StatusCode::SEE_OTHER);
    }

    #[tokio::test]
    async fn passing_exist_unexpired_cookie_id() {
        let conn = prepare_db_connection().await;
        delete_related_models(&conn).await;
        admin_user::mutation::seed_with_unexpired_session(&conn)
            .await
            .expect("failed to seed admin_user");
        let all_users = admin_user::mutation::find_all(&conn)
            .await
            .expect("failed to find all admin_user");
        let first_user = all_users.first().unwrap();
        let all_sessions = session::mutation::find_unexpired_by_admin_user_id(&conn, first_user.id)
            .await
            .expect("failed to find admin_user sessions");
        let first_session = all_sessions.first().unwrap();
        let req = Request::builder()
            .uri("/device")
            .header("uid", first_user.id)
            .header("cookie_id", first_session.cookie_id.clone())
            .body(Body::empty())
            .unwrap();
        let res = prepare_router(conn).await.oneshot(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn passing_exist_expired_cookie_id() {
        let conn = prepare_db_connection().await;
        delete_related_models(&conn).await;
        admin_user::mutation::seed_with_expired_session(&conn)
            .await
            .expect("failed to seed admin_user");
        let all_users = admin_user::mutation::find_all(&conn)
            .await
            .expect("failed to find all admin_user");
        let first_user = all_users.first().unwrap();
        let all_sessions = session::mutation::find_expired_by_admin_user_id(&conn, first_user.id)
            .await
            .expect("failed to find admin_user sessions");
        let first_session = all_sessions.first().unwrap();
        let req = Request::builder()
            .uri("/device")
            .header("uid", first_user.id)
            .header("cookie_id", first_session.cookie_id.clone())
            .body(Body::empty())
            .unwrap();
        let res = prepare_router(conn).await.oneshot(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::SEE_OTHER);
    }
}
