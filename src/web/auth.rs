use axum::{
    body::Body,
    extract::{Request, State},
    http::{Response, StatusCode},
    middleware::Next,
    response::IntoResponse,
};
use model_entity::admin_user;

use crate::web::middleware::buffer_and_print;

use super::middleware::AppState;

pub async fn check_session_id(
    state: State<AppState>,
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
    } else {
        match maybe_uid
            .unwrap()
            .to_str()
            .expect("something is wrong of header uid value")
            .parse::<i32>()
        {
            Ok(uid) => {
                match admin_user::mutation::find_by_id(&state.conn, uid)
                    .await
                    .expect("failed to find admin_user by id")
                {
                    Some(_admin_user) => (),
                    None => {
                        return Err((
                            StatusCode::INTERNAL_SERVER_ERROR,
                            "admin_user linked with specified uid is not exists".to_string(),
                        ));
                    }
                }
            }
            Err(e) => return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
        }
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
    use std::env;

    use crate::web::routes;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
        Router,
    };
    use model_entity::admin_user;
    use rstest::rstest;
    use sea_orm::{Database, DatabaseConnection};
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
    #[case(false, false)]
    async fn checking_header_uid_for_protected_route(
        #[case] missing_uid: bool,
        #[case] missing_session_id: bool,
    ) {
        let conn = prepare_db_connection().await;
        let mut custom_req = Request::builder().uri("/device/");
        if !missing_uid {
            admin_user::mutation::delete_all(&conn)
                .await
                .expect("failed to delete all admin_user");
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
            StatusCode::EXPECTATION_FAILED
        } else {
            StatusCode::OK
        };
        assert_eq!(res.status(), expect_code);
    }

    #[tokio::test]
    async fn passing_not_exist_uid() {
        let conn = prepare_db_connection().await;
        admin_user::mutation::delete_all(&conn)
            .await
            .expect("failed to delete all admin_user");
        admin_user::mutation::seed(&conn)
            .await
            .expect("failed to seed admin_user");
        let req = Request::builder()
            .uri("/device/")
            .header("uid", 0)
            .header("cookie_id", "Bar")
            .body(Body::empty())
            .unwrap();
        let res = prepare_router(conn).await.oneshot(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }
}
