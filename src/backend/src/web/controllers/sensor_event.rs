use crate::web::{middleware::AppState, SimpleRes};
use axum::{extract::State, Json};
use model_entity::models::sensor_event::{self, query::SensorEventQuery};
use serde::Serialize;

#[derive(Serialize)]
pub struct List {
    sensor_events: Vec<sensor_event::model::Model>,
}

pub async fn list(state: State<AppState>) -> Result<Json<List>, Json<SimpleRes>> {
    let sensor_events = SensorEventQuery::find(&state.conn).await.map_err(|_| {
        Json(SimpleRes {
            message: "Cannot find sensor events in page".to_string(),
        })
    })?;

    Ok(Json(List { sensor_events }))
}
