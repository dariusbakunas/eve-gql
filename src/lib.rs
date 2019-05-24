#[macro_use]
extern crate juniper;

use std::io;
use actix_web::{middleware, App, HttpServer, web};

mod gql;

use gql::routes::graphiql;
use gql::routes::graphql;
use gql::schema::create_schema;


pub fn run() -> io::Result<()> {
    // Create Juniper schema
    let schema = std::sync::Arc::new(create_schema());

    // Start http server
    HttpServer::new(move || {
        App::new()
            .data(schema.clone())
            .wrap(middleware::Logger::default())
            .service(web::resource("/graphql").route(web::post().to_async(graphql)))
            .service(web::resource("/graphiql").route(web::get().to(graphiql)))
    })
        .bind("127.0.0.1:8080")?
        .run()
}