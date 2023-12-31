use actix_web::{get, post, web::Json};
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize)]
pub struct ConfigurationResponse {
    configuration: String,
}

#[derive(Deserialize)]
pub struct ConfigurationPostRequest {
    configuration: String,
}

#[post("/api/v1/config")]
pub async fn post_configuration(
    input: Json<ConfigurationPostRequest>,
) -> Json<ConfigurationResponse> {
    Json(ConfigurationResponse {
        configuration: input.into_inner().configuration,
    })
}

#[get("/api/v1/config")]
pub async fn get_configuration() -> Json<ConfigurationResponse> {
    Json(ConfigurationResponse {
        configuration: "default configuration".to_string(),
    })
}
