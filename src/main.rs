//! Actix web juniper example
//!
//! A simple example integrating juniper in actix-web
use std::io;
use eve_gql::run;

fn main() -> io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    run()
}