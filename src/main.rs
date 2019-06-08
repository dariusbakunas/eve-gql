use std::io;
use eve_gql::run;
use std::env;
use dotenv;
use std::str;

fn main() -> io::Result<()> {
    std::env::set_var("RUST_LOG", "info, actix_web=info");
    env_logger::init();

    let encoded_database_url = match env::var("DATABASE_URL") {
        Ok(url) => {url},
        Err(_) => {
            dotenv::dotenv().expect("No .env file found");
            let url = env::var("DATABASE_URL")
                .expect("DATABASE_URL must be set in .env");
            url
        },
    };

    let database_url = base64::decode(&encoded_database_url)
        .expect("Unable to decode DATABASE_URL");

    run(str::from_utf8(&database_url[..])
        .expect("Unable to decode DATABASE_URL"))
}