use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use juniper::RootNode;

use crate::graphql::mutation::Mutation;
use crate::graphql::query::Query;
extern crate r2d2;
use crate::DbPoolData;

pub struct Context {
    pub pool: DbPoolData,
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
