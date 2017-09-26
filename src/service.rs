use hyper::server::{Request, Response, Service};
use hyper::Error;
use futures::Future;

#[derive(Debug)]
pub struct GinService;

impl Service for GinService {
    type Request = Request;
    type Response = Response;
    type Error = Error;
    type Future = Box<Future<Item = Self::Response, Error = Self::Error>>;

    fn call(&self, req: Self::Request) -> Self::Future {
        let (_method, _url, _version, _headers, _body ) = req.deconstruct();
        unimplemented!()
    }
}