use axum::{
    response::{IntoResponse, Response},
    Json,
};
use http::StatusCode;

use crate::pb;

pub async fn greeter_handler() -> Result<impl IntoResponse, Response> {
    let resp = pb::helloworld::HelloResponse { timestamp: 123 };
    Ok((StatusCode::OK, Json(resp)))
}
