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
}

pub async fn list_related_capture(
    state: State<AppState>,
    Query(params): Query<ListRelatedCaptureParams>,
) -> Result<Json<ListRelatedCapture>, Json<SimpleRes>> {
    let sensor_purpose_id = params.sensor_purpose_id;
    let models =
        SensorPurposeQuery::find_with_related_sensor_and_capture(&state.conn, sensor_purpose_id)
            .await
            .map_err(|_| {
                Json(SimpleRes {
                    message: "Cannot find sensors in page".to_string(),
                })
            })?;

    Ok(Json(ListRelatedCapture { models }))
}
