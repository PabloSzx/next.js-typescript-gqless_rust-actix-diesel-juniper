use actix_web::web::{self};
pub mod login;
pub mod logout;
pub mod sign_up;

pub fn route(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/api/login").route(web::post().to(login::handler)));

    cfg.service(web::resource("/api/logout").route(web::post().to(logout::handler)));
}
