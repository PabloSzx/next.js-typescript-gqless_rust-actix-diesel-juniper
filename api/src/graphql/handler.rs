use crate::graphql::schema::{Auth, Context, Schema};
use crate::DbPoolData;
use actix_session::Session;
use actix_web::{dev, error, web, Error, HttpResponse};
use juniper::http::{playground::playground_source, GraphQLRequest};
use std::sync::Arc;

pub async fn graphql(
    session: Session,
    pool: DbPoolData,
    st: web::Data<Arc<Schema>>,
    data: web::Json<GraphQLRequest>,
) -> Result<HttpResponse, Error> {
    let mut rt = futures::executor::LocalPool::new();
    // Context setup
    let email = session.get("authorization").unwrap();

    let ctx = Context {
        pool,
        auth: Auth { email },
    };

    // Execute
    let future_execute = data.execute_async(&st, &ctx);
    let res = rt.run_until(future_execute);
    let json = serde_json::to_string(&res).map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(json))
}

pub fn playground() -> HttpResponse {
    let html = playground_source("/api/graphql");
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}
