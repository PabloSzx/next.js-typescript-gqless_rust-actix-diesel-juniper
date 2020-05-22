use actix_session::Session;
use actix_web::{Error, HttpResponse};

pub async fn handler(session: Session) -> Result<HttpResponse, Error> {
    match session.get::<String>("authorization").unwrap() {
        Some(_) => {
            session.clear();

            Ok(HttpResponse::Ok().body("OK"))
        }
        None => Ok(HttpResponse::Ok().body("No logout needed")),
    }
}
