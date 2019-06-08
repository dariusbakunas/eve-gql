#![recursion_limit="128"]

#[macro_use]
extern crate juniper;

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate error_chain;

#[macro_use]
extern crate log;

use std::io;
use actix_web::{middleware, App, HttpServer, web};
use r2d2;
use r2d2_diesel::ConnectionManager;
use diesel::mysql::MysqlConnection;
use juniper::{Context as JuniperContext};
use std::sync::Arc;

mod gql;
mod dao;
mod esi;
mod errors;

use gql::routes::graphiql;
use gql::routes::graphql;
use gql::schema::create_schema;
use crate::gql::schema::Schema;

pub struct Context {
    pub pool: r2d2::Pool<r2d2_diesel::ConnectionManager<MysqlConnection>>,
}

impl JuniperContext for Context {}

pub struct AppState {
    schema: Arc<Schema>,
    context: Context
}

pub fn run(database_url: &str) -> io::Result<()> {
    // Create Juniper schema
    let schema = std::sync::Arc::new(create_schema());

    let manager = ConnectionManager::<MysqlConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .max_size(15)
        .build(manager)
        .unwrap();

    // Start http server
    HttpServer::new(move || {
        App::new()
            .data(AppState {
                schema: schema.clone(),
                context: Context {
                    pool: pool.clone(),
                },
            })
            .wrap(middleware::Logger::default())
            .service(web::resource("/graphql").route(web::post().to_async(graphql)))
            .service(web::resource("/graphiql").route(web::get().to(graphiql)))
    })
        .bind("127.0.0.1:8080")?
        .run()
}