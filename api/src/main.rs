#![feature(decl_macro, proc_macro_hygiene)]
extern crate serde_derive;

#[macro_use]
extern crate diesel;

pub mod graphql;
pub mod models;
pub mod schema;
pub mod type_defs;

extern crate dotenv;
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

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be seta");

    let manager: ConnectionManager<PgConnection> =
        ConnectionManager::<PgConnection>::new(database_url);

    let pool: DbPool = r2d2::Pool::builder()
        .max_size(15)
        .build(manager)
        .expect("failed to create pool");

    let server = HttpServer::new(move || {
        App::new()
            .data(schema.clone())
            .data(pool.clone())
            .configure(graphql::route)
    })
    .bind(("127.0.0.1", 8000))
    .unwrap()
    .run();

    eprintln!("Listening on localhost:8000");

    server.await
}
