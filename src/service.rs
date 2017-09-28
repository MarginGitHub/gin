use hyper::server::{Request, Response, Service};
use hyper::Error;
use futures::{future, Future};

use context::Context;

pub struct GinService;

impl Service for GinService {
    type Request = Request;
    type Response = Response;
    type Error = Error;
    type Future = Box<Future<Item = Self::Response, Error = Self::Error>>;

    fn call(&self, req: Self::Request) -> Self::Future {
        let mut c = Context::new(&req);
        c.init_query();
        if let Some(_router) = unsafe{::ROUTER.as_ref()} {
            match _router.dispatch(&mut c) {
                Ok(_) => Box::new(future::ok(c.response().unwrap())),
                Err(err) => {
                    eprintln!("{}", err);
                    Box::new(future::err(Error::Method))
                }
            }
        } else {
            Box::new(future::err(Error::Method))
        }

    }
}

