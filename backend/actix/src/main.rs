use actix_web::{web, App, HttpServer, http};
use actix_cors::Cors;
use std::sync::Arc;
use tokio::sync::Mutex;
mod db_control;
use db_control::{Database, AppState, load_connection_string};

mod generate_token;
use generate_token::generate_token;
mod validate_token;
use validate_token::validate_token;

//로깅

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let connection_string = load_connection_string();
    let db = Database::new(&connection_string).await?;
    let app_data = web::Data::new(AppState {
        db: Arc::new(Mutex::new(db)),
    });

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("https://feedback.jinwoolee.info")
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);

        App::new()
            .wrap(cors)
            .app_data(app_data.clone())
            .service(
                web::resource("/api/check/{date}/{type}")
                    .route(web::get().to(generate_token))
            )
            .service(
                web::resource("/api/validate/{token}")
                    .route(web::get().to(validate_token))
            )
            // 다른 API 라우트들...
    })
    .bind("0.0.0.0:5095")?
    .run()
    .await?;

    Ok(())
}
