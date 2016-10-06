#![feature(custom_derive, custom_attribute, plugin)]
#![plugin(diesel_codegen, dotenv_macros)]

// Related to Iron itself.
extern crate iron;
extern crate router;
extern crate staticfile;
extern crate mount;
extern crate logger;

// Related to database
#[macro_use] extern crate diesel;
extern crate r2d2;
extern crate r2d2_diesel;

// Easy handling of `.env`
extern crate dotenv;

mod api;
mod models;
mod schema;

use api::Api;

use iron::{Iron, Chain};
use logger::Logger;

use dotenv::dotenv;
use std::env;

fn main() {
    // Loads the `.env` file.
    dotenv().ok();

    // Get the database URL from `.env` or fail.
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    // Initialize the chain with the API.
    let api = Api::new(database_url);
    let mut chain = Chain::new(api);

    // Initialize logging with default output.
    let (logger_before, logger_after) = Logger::new(None);
    chain.link_before(logger_before);
    chain.link_after(logger_after);

    // Begin listening and output something useful for the user.
    match Iron::new(chain).http("localhost:3000") {
        Result::Ok(listening) => println!("{:?}", listening),
        Result::Err(err) => panic!("{:?}", err),
    };
}
