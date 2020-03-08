extern crate actix_rt;
extern crate actix_web;
extern crate diesel;
// extern crate dotenv;
extern crate actix_graphql_react_apollo;
extern crate env_logger;
extern crate juniper;
extern crate r2d2;

use dotenv::dotenv;
use std::{env, io};

use actix_web::{middleware, App, HttpServer};

use actix_graphql_react_apollo::db::get_pool;
use actix_graphql_react_apollo::endpoints::graphql_endpoints;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv().ok();
    logging_setup();

    // Instantiate a new connection pool
    let pool = get_pool();

    let graphql_port: i16 = env::var("GRAPHQL_PORT")
        .unwrap_or_else(|_| String::from("80"))
        .parse()
        .expect("GRAPHQL_PORT must be a number");

    // Start up the server, passing in (a) the connection pool
    // to make it available to all endpoints and (b) the configuration
    // function that adds the /graphql logic.
    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(middleware::Logger::default())
            .configure(graphql_endpoints)
    })
    .bind(format!("0.0.0.0:{}", graphql_port))?
    .run()
    .await
}

// TODO: more fine-grained logging setup
fn logging_setup() {
    env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
}
