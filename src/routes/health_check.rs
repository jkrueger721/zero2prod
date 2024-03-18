use actix_web::HttpResponse;
use serde_json::json;

pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json(json!({"message": "we are good to go"}))
}
