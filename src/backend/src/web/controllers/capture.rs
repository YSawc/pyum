use crate::web::{middleware::AppState, SimpleRes};
use axum::{
    extract::{Query, State},
    Json,
};
use model_entity::models::{
    capture, sensor,
    sensor_purpose::{self, query::SensorPurposeQuery},
};
use serde::{Deserialize, Serialize};
use tower_sessions::Session;

#[derive(Serialize)]
pub struct ListRelatedCapture {
    models: (
        sensor_purpose::model::Model,
        Vec<(sensor::model::Model, Vec<capture::model::Model>)>,
    ),
}

#[derive(Deserialize)]
pub struct ListRelatedCaptureParams {
    sensor_purpose_id: i32,
    limit: Option<i32>,
    start_date: Option<String>,
    end_date: Option<String>,
}

pub async fn list_related_capture(
    state: State<AppState>,
    Query(params): Query<ListRelatedCaptureParams>,
) -> Result<Json<ListRelatedCapture>, Json<SimpleRes>> {
    let sensor_purpose_id = params.sensor_purpose_id;
    let limit = params.limit;
    let start_date = params.start_date;
    let end_date = params.end_date;
    let models = SensorPurposeQuery::find_with_related_sensor_and_capture(
        &state.conn,
        sensor_purpose_id,
        limit,
        start_date,
        end_date,
    )
    .await
    .map_err(|_| {
        Json(SimpleRes {
            message: "Cannot find sensors in page".to_string(),
        })
    })?;

    Ok(Json(ListRelatedCapture { models }))
}

pub async fn create_capture(
    session: Session,
    state: State<AppState>,
    Json(form_data): Json<capture::model::Model>,
) -> Result<(), Json<SimpleRes>> {
    let models = sensor::mutation::get_by_id(&state.conn, form_data.sensor_id)
        .await
        .unwrap();
    let uid: i32 = session.get("uid").await.unwrap().unwrap();
    if models.1.admin_user_id == uid {
        capture::mutation::create(&state.conn, form_data)
            .await
            .unwrap();
        Ok(())
    } else {
        Err(Json(SimpleRes {
            message: "user id is not match".to_string(),
        }))
    }
}
