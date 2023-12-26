use actix_web::{get, web::Json};
use serde::Serialize;

#[derive(Serialize)]
pub struct ConfigurationResponse {
    configuration: String,
}

#[get("/api/v1/config")]
pub async fn get_configuration() -> Json<ConfigurationResponse> {
    Json(ConfigurationResponse {
        configuration: "default configuration".to_string(),
    })
}
