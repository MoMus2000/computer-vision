use crate::gui::index;
use crate::api::filter;

use actix_web::{web, App, HttpServer};


mod gui;
mod cv;
mod api;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index::index))
            .route("/filter", web::post().to(filter::apply_filter))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
