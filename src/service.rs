use hyper::server::{Request, Response, Service};
use hyper::Error;
use futures::{future, Future};

use context::Context;
use router::Router;
use std::sync::Arc;
use std::cell::RefCell;
use std::ops::Deref;

pub struct GinService(pub Arc<RefCell<Router>>);

impl Service for GinService {
    type Request = Request;
    type Response = Response;
    type Error = Error;
    type Future = Box<Future<Item = Self::Response, Error = Self::Error>>;

    fn call(&self, req: Self::Request) -> Self::Future {
//        #[derive(Serialize)]
//        struct Hello {
//            name: String,
//            password: String,
//        }
//        let h = Hello{name: "dong".to_string(), password: "12345".to_string()};
//        let mut context = Context::new(&req);
//        context.json(StatusCode::Ok, &h);
//        Box::new(future::ok(context.response().unwrap()))
        let path = req.uri().path();
        println!("{}", path);
        if let Some(handler) = self.0.as_ref().borrow().deref().get(path) {
            let mut c = Context::new(&req);
            handler.handle(&mut c);
            Box::new(future::ok(c.response().unwrap()))
        } else {
            Box::new(future::err(Error::Method))
        }

    }
}

