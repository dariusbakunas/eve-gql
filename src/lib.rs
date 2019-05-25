#[macro_use]
extern crate juniper;

use std::io;
use actix_web::{middleware, App, HttpServer, web};
use r2d2;
use r2d2_diesel::ConnectionManager;
use diesel::mysql::MysqlConnection;
use std::sync::Arc;

mod gql;

use gql::routes::graphiql;
use gql::routes::graphql;
use gql::schema::create_schema;
use crate::gql::schema::Schema;

pub struct AppState {
    schema: Arc<Schema>,
    pool: r2d2::Pool<r2d2_diesel::ConnectionManager<MysqlConnection>>,
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
                pool: pool.clone(),
            })
            .wrap(middleware::Logger::default())
            .service(web::resource("/graphql").route(web::post().to_async(graphql)))
            .service(web::resource("/graphiql").route(web::get().to(graphiql)))
    })
        .bind("127.0.0.1:8080")?
        .run()
}