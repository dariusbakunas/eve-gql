use std::io;
use eve_gql::run;
use std::env;
use dotenv;

fn main() -> io::Result<()> {
    std::env::set_var("RUST_LOG", "info, actix_web=info");
    env_logger::init();

    dotenv::dotenv().expect("No .env file found");
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env");
    run(&database_url)
}