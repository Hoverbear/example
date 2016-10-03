use iron::{Handler, Request, Response, IronResult, status};

pub struct Api {}

impl Api {
    pub fn new() -> Api {
        Api {}
    }
}

impl Handler for Api {
    fn handle(&self, _: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok, "Hello from API")))
    }
}
