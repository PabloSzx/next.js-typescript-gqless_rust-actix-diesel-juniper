use crate::graphql::schema::{Context, Schema};
use crate::DbPoolData;
use actix_web::{error, web, Error, HttpResponse};
use juniper::http::{playground::playground_source, GraphQLRequest};
use std::sync::Arc;

pub async fn graphql(
    pool: DbPoolData,
    st: web::Data<Arc<Schema>>,
    data: web::Json<GraphQLRequest>,
) -> Result<HttpResponse, Error> {
    let mut rt = futures::executor::LocalPool::new();

    // Context setup
    let ctx = Context { pool };

    // Execute
    let future_execute = data.execute_async(&st, &ctx);
    let res = rt.run_until(future_execute);
    let json = serde_json::to_string(&res).map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(json))
}

pub fn playground() -> HttpResponse {
    // I prefer playground but you can use graphiql as well
    let html = playground_source("http://localhost:8000/api/graphql");
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}
