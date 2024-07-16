use actix_web::{web, App, HttpServer, http};
use actix_cors::Cors;
use std::sync::Arc;
use tokio::sync::Mutex;
mod db_control;
use db_control::{Database, AppState, load_connection_string};

mod generate_token;
use generate_token::generate_token;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let connection_string = load_connection_string();
    let db = Database::new(&connection_string).await?;
    let app_data = web::Data::new(AppState {
        db: Arc::new(Mutex::new(db)),
    });

    HttpServer::new(move || {
        let cors = Cors::default() // 배포하면 다 막기 사실 지금도 배포야 할 수 있지만
            .allow_any_origin() // 모든 출처 허용
            .allow_any_method() // 모든 HTTP 메서드 허용
            .allow_any_header() // 모든 헤더 허용
            .max_age(3600);

        App::new()
            .wrap(cors)
            .app_data(app_data.clone())
            .service(
                web::resource("/api/check/{date}/{type}")
                    .route(web::get().to(generate_token))
            )
            // 다른 API 라우트들...
    })
    .bind("0.0.0.0:5095")?
    .run()
    .await?;

    Ok(())
}
