use actix_web::{Error, HttpResponse, web};
use futures::future::Future;
use std::sync::Arc;
use juniper::http::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;
use juniper::{Context as JuniperContext};
use r2d2;
use r2d2_diesel;
use diesel::mysql::MysqlConnection;

use crate::gql::schema::{Schema};
use crate::AppState;

struct Context {
    pub db: r2d2::Pool<r2d2_diesel::ConnectionManager<MysqlConnection>>,
}

impl JuniperContext for Context {}

pub fn graphiql() -> HttpResponse {
    let html = graphiql_source("http://127.0.0.1:8080/graphql");
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

pub fn graphql(
    st: web::Data<AppState>,
    data: web::Json<GraphQLRequest>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    web::block(move || {
        let res = data.execute(&st.schema, &());
        Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)
    })
        .map_err(Error::from)
        .and_then(|user| {
            Ok(HttpResponse::Ok()
                .content_type("application/json")
                .body(user))
        })
}