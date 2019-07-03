use actix_web::{Error, HttpResponse, HttpRequest, web};
use futures::future::Future;
use juniper::http::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;

use crate::{AppState, Context};

pub fn graphiql() -> HttpResponse {
    let html = graphiql_source("http://127.0.0.1:8080/graphql");
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

pub fn graphql(
    req: HttpRequest,
    st: web::Data<AppState>,
    data: web::Json<GraphQLRequest>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    info!("{:?}", req.headers());

    let esi_token = req.headers()
        .get("x-esi-token")
        .map(|value| value.to_str().ok())
        .unwrap_or(None)
        .and_then(|str| Some(String::from(str)));

    info!("{:?}", esi_token);

    web::block(move || {
        let context = Context {
            pool: st.context.pool.clone(),
            esi_token: esi_token,
        };

        let res = data.execute(&st.schema, &context);
        Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)
    })
        .map_err(Error::from)
        .and_then(|user| {
            Ok(HttpResponse::Ok()
                .content_type("application/json")
                .body(user))
        })
}