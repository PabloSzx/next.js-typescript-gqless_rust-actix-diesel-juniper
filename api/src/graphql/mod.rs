use actix_web::http;
use actix_web::web::{self, HttpResponse};
mod handler;
pub mod mutation;
pub mod query;
pub mod schema;

async fn redirect() -> HttpResponse {
    HttpResponse::Ok()
        .status(http::StatusCode::from_u16(307).unwrap())
        .set_header("Location", "/api/graphql")
        .body("Redirecting")
}

pub fn route(cfg: &mut web::ServiceConfig) {
    cfg.route("/", web::get().to(redirect));
    cfg.service(
        web::resource("/api/graphql")
            .route(web::post().to(handler::graphql))
            .route(web::get().to(handler::playground)),
    );
}
