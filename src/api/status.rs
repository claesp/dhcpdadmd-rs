use chrono::prelude::*;

use actix_web::{
    error::ResponseError,
    get,
    http::{header::ContentType, StatusCode},
    post, put,
    web::Data,
    web::Json,
    web::Path,
    HttpResponse,
};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct StatusTypeIdentifier {
    status_type: String,
}

#[derive(Serialize)]
pub struct StatusTypeResponse {
    status_type_response: String,
    status_type_date: String
}

#[get("/api/v1/status/{status_type}")]
pub async fn get_status(type_identifier: Path<StatusTypeIdentifier>) -> Json<StatusTypeResponse> {
    let status_type = type_identifier.into_inner().status_type;
    let status_type_date = Local::now().to_string();
    let status_type_response: String;
    match status_type.as_str() {
        "ping" => {
            status_type_response = "pong".to_string();
        }
        _ => status_type_response = "unknown status type".to_string()
    }

    let resp = StatusTypeResponse{ status_type_response, status_type_date };
    Json(resp)
}
