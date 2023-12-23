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

#[get("/api/v1/status/{status_type}")]
pub async fn get_status(type_identifier: Path<StatusTypeIdentifier>) -> Json<String> {
    return Json(type_identifier.into_inner().status_type);
}
