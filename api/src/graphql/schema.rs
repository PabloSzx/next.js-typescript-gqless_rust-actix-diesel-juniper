use crate::graphql::mutation::Mutation;
use crate::graphql::query::Query;
use actix_web::{cookie, dev, App, Error, FromRequest, HttpRequest};
use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use juniper::RootNode;
extern crate r2d2;
use crate::DbPoolData;

use actix_http::httpmessage::HttpMessage;
use actix_web::error::ErrorBadRequest;
use futures::future::{err, ok, Ready};
pub struct Context {
    pub pool: DbPoolData,
    pub auth: Auth,
}

pub struct Auth {
    pub email: Option<String>,
}

impl juniper::Context for Context {}

impl Context {
    pub fn db_connection<F, T>(&self, action: F) -> T
    where
        F: Fn(r2d2::PooledConnection<ConnectionManager<PgConnection>>) -> T,
    {
        let connection = self.pool.get().unwrap();

        action(connection)
    }
}

pub type Schema = RootNode<'static, Query, Mutation>;

pub fn create_schema() -> Schema {
    Schema::new(Query {}, Mutation {})
}
