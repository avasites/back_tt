extern crate timetable;
use actix_cors::Cors;
use actix_web::{web, App, HttpServer, http};

use timetable::search;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(
                Cors::new() // <- Construct CORS middleware builder
                .supports_credentials() 
                .allowed_methods(vec!["GET", "POST"])
                .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
                .allowed_header(http::header::CONTENT_TYPE)
                .max_age(3600)
                .finish()
            )            
            .route("/search", web::post().to(search::get_teacher_by_name))
    })
    .bind("127.0.0.1:8232")?
    .run()
    .await
}
