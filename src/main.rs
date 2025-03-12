mod db;
mod templates;

use crate::db::get_connection;
use actix_files::Files;
use actix_web::{App, HttpServer, web};
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let db = get_connection().await;
    let template_manager = templates::TemplateManager::new("view");
    let host = env::var("SEAMS_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port: u16 = env::var("SEAMS_PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .expect("Invalid port number");

    println!("Starting server at http://{}:{}", host, port);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db.clone()))
            .app_data(web::Data::new(template_manager.handlebars.clone()))
            .service(Files::new("/static", "./static").show_files_listing())

        // There is a place for routes
        // .service(
        //     web::scope("/")
        //         .route("/{url:.*}", web::get().to(page_handler))
        // )
    })
    .bind((host.as_str(), port))?
    .run()
    .await
}
