use hyper::server::{Request, Response, Service};
use hyper::Error;
use hyper::StatusCode;
use futures::{future, Future};

use context::Context;

#[derive(Debug)]
pub struct GinService;

impl Service for GinService {
    type Request = Request;
    type Response = Response;
    type Error = Error;
    type Future = Box<Future<Item = Self::Response, Error = Self::Error>>;

    fn call(&self, req: Self::Request) -> Self::Future {
        let mut context = Context::new(&req);
        context.string(StatusCode::Ok, "Hello".to_string());
        Box::new(future::ok(context.response().unwrap()))
    }
}