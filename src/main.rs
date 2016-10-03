extern crate iron;
extern crate router;
extern crate staticfile;
extern crate mount;
extern crate logger;

mod api;

use api::Api;

use iron::{Iron, Chain};
use router::Router;
use staticfile::Static;
use mount::Mount;
use logger::Logger;

use std::path::Path;


fn main() {
    //
    let api_mount = {
        let mut mount = Mount::new();
        mount.mount("/api/", Api::new());

        mount
    };

    let router = {
        let mut router = Router::new();
        // Serve static assets from `site/` on bare routes.
        router.get("/", Static::new(Path::new("site/")), "site");
        // Serve from the API on anything on the `/api` routes.
        router.any("/api", api_mount, "api");

        router
    };

    // Initialize logging.
    let mut chain = Chain::new(router);
    let (logger_before, logger_after) = Logger::new(None);
    chain.link_before(logger_before);
    chain.link_after(logger_after);

    match Iron::new(chain).http("localhost:3000") {
        Result::Ok(listening) => println!("{:?}", listening),
        Result::Err(err) => panic!("{:?}", err),
    };
}
