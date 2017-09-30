use std::sync::Arc;
use hyper::server::{Request, Response};
use tokio_service::{Service, NewService};
use hyper::Error;
use futures::{future, Future};

use context::Context;
use router::Router;
use std::io;

pub struct GinService(pub Arc<Router>);

impl Service for GinService {
    type Request = Request;
    type Response = Response;
    type Error = Error;
    type Future = Box<Future<Item=Self::Response, Error=Self::Error>>;

    fn call(&self, req: Self::Request) -> Self::Future {
        let c = Context::new(&req);
//        println!("{:#?}\n{:#?}", c.url(), c.patterns());
        match self.0.dispatch(c) {
            Ok(_resp) => Box::new(future::ok(_resp)),
            Err(err) => {
                eprintln!("{}", err);
                Box::new(future::err(Error::Method))
            }
        }
    }
}

pub struct GinNewService(pub Arc<Router>);

impl NewService for GinNewService {
    type Request = Request;
    type Response = Response;
    type Error = Error;
    type Instance = GinService;

    fn new_service(&self) -> io::Result<Self::Instance> {
        Ok(GinService(self.0.clone()))
    }
}



