use actix_web::{get, post, web::Json};
use serde::Serialize;

#[derive(Serialize)]
pub struct ConfigurationResponse {
    configuration: String,
}

#[post("/api/v1/config")]
pub async fn post_configuration() -> Json<ConfigurationResponse> {
    Json(ConfigurationResponse {
        configuration: "default configuration".to_string(),
    })
}

#[get("/api/v1/config")]
pub async fn get_configuration() -> Json<ConfigurationResponse> {
    Json(ConfigurationResponse {
        configuration: "default configuration".to_string(),
    })
}
