extern crate serde_derive;

#[macro_use]
extern crate diesel;

pub mod auth;
pub mod graphql;
pub mod schema;
pub mod type_defs;

extern crate dotenv;

use actix_session::CookieSession;
use actix_web::{web, App, HttpServer};
use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use dotenv::dotenv;
use std::env;

pub type DbPool = r2d2::Pool<diesel::r2d2::ConnectionManager<PgConnection>>;
pub type DbPoolData = web::Data<DbPool>;
#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let schema = std::sync::Arc::new(crate::graphql::schema::create_schema());

    let secret_token = env::var("SECRET_TOKEN").unwrap_or(String::from("secret_default"));
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be seta");

    let manager: ConnectionManager<PgConnection> =
        ConnectionManager::<PgConnection>::new(database_url);

    let pool: DbPool = r2d2::Pool::builder()
        .max_size(15)
        .build(manager)
        .expect("failed to create pool");

    let server = HttpServer::new(move || {
        App::new()
            .wrap(
                CookieSession::signed(secret_token.as_bytes())
                    .secure(false)
                    .max_age(30),
            )
            .data(schema.clone())
            .data(pool.clone())
            .configure(graphql::route)
            .configure(auth::route)
    })
    .bind(("127.0.0.1", 8000))
    .unwrap()
    .run();

    eprintln!("Listening on localhost:8000");

    server.await
}
