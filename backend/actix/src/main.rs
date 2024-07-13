use actix_web::{web, App, HttpServer};
use std::sync::Arc;
use tokio::sync::Mutex;

mod db_control;
use db_control::{Database, AppState, load_connection_string};

mod api_check;
use api_check::api_check;


#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    // 환경 변수에서 데이터베이스 연결 정보 로드
    let connection_string = load_connection_string();

    // Database 인스턴스 생성
    let db = Database::new(&connection_string).await?;

    // 웹 데이터 설정
    let app_data = web::Data::new(AppState {
        db: Arc::new(Mutex::new(db)),
    });

    // Actix 설정
    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .route("/api/check/{user_token}/{service_token}", web::get().to(api_check))
    })

    //추후 Origin Domain 거르는 미들웨어 추가 필요

    .bind("0.0.0.0:5095")?
    .run()
    .await?;

    Ok(())
}
