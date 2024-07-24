use actix_web::{web, HttpResponse};

pub async fn validate_token(token: web::Json<String>) -> HttpResponse {
    // Your token validation logic here
    
    // Example response
    HttpResponse::Ok().json("Token is valid")
}