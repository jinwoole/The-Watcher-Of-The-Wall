use actix_web::{web, HttpResponse};

pub async fn validate_token(token: web::Json<String>) -> HttpResponse {
    // Your token validation logic here
    let token = format!("{:x}", result)[..32].to_string();

    let json_result = format!("{{\"token\": \"{}\"}}", token);

    Ok(json_result)
    // Example response
    HttpResponse::Ok().json("Token is valid")
}