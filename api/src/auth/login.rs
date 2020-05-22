use crate::schema::users::dsl::*;
use crate::type_defs;
use crate::DbPoolData;
use actix_session::Session;
use actix_web::{web, Error, HttpResponse};
use diesel::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct AuthInfo {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct AuthResult {
    pub ok: bool,
}

pub async fn handler(
    session: Session,
    info: web::Json<AuthInfo>,
    pool: DbPoolData,
) -> Result<HttpResponse, Error> {
    let db = pool.get().unwrap();

    let user = users
        .filter(email.eq(&info.email))
        .filter(password.eq(&info.password))
        .first::<type_defs::users::UserSQL>(&*db);

    match user {
        Ok(v) => {
            session.set("authorization", v.email).unwrap();

            Ok(HttpResponse::Ok().json(AuthResult { ok: true }))
        }
        Err(e) => Ok(HttpResponse::Ok().json(AuthResult { ok: false })),
    }
}
