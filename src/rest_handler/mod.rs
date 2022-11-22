use axum::{
    response::{IntoResponse, Response},
    Json,
};
use http::StatusCode;

use crate::pb;

pub async fn greeter_handler(
    Json(data): Json<pb::helloworld::HelloRequest>,
) -> Result<impl IntoResponse, Response> {
    let resp = pb::helloworld::HelloResponse {
        message: format!("Hello {}", data.name),
        test_array: Vec::new(),
    };

    Ok((StatusCode::OK, Json(resp)))
}
