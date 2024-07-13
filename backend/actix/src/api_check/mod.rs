// src/api_check/mod.rs
use actix_web::{web, HttpResponse, Responder};
use crate::db_control::AppState;


// check api는 쿠키에 저장된 토큰값과 요청한 UID를 비교해 유저가 맞는지 auth하는 api

// 아래는 그냥 db 작동하는지 확인만 하는 예시 코드
pub async fn api_check(data: web::Data<AppState>, path: web::Path<(String, String)>) -> impl Responder {
    let db = data.db.lock().await;
    let query = format!("SELECT 1");
    match db.execute_query(&query).await {
        Ok(_) => HttpResponse::Ok().body(format!("Database connection is healthy. Query: {}, {}", path.0, path.1)),
        Err(err) => HttpResponse::InternalServerError().body(format!("Database connection error: {}. Query: {}", err, query)),
    }
}