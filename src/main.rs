// Related to Iron itself.
extern crate iron;
extern crate router;
extern crate staticfile;
extern crate mount;
extern crate logger;

// Related to database
extern crate diesel;
extern crate r2d2;
extern crate r2d2_diesel;

mod api;

use api::Api;

use iron::{Iron, Chain};
use logger::Logger;

// Use memory for now, non-persistent.
const DATABASE_CONNECTION_STRING: &'static str = ":memory:";

fn main() {
    // Initialize the chain with the API.
    let api = Api::new(DATABASE_CONNECTION_STRING);
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
