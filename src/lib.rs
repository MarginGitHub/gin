
pub extern crate futures;
pub extern crate futures_cpupool;
extern crate tokio_service;
pub extern crate hyper;

extern crate serde;
extern crate serde_json;


mod service;
mod param;
mod router;
mod context;
mod html;
mod url;

use hyper::server::Http;
use hyper::{Result, Server, StatusCode, Body};
use std::net::SocketAddr;
use std::sync::Arc;
use std::marker::{Sync, Send};
use std::path::Path;

use router::{Router};
use service::GinNewService;
pub use context::Context;


pub struct Gin {
    router: Arc<Router>,
}

impl Gin {
    pub fn new() -> Self {
        Gin {
            router: Arc::new(Router::new()),
        }
    }
}

impl Gin {
    pub fn get<H>(&mut self, path: &'static str, handler: H) -> &mut Self
        where H: Fn(&mut Context) + Send + Sync +'static {
        if let Some(_router) = Arc::get_mut(&mut self.router) {
            _router.get(path, Box::new(handler));
        }
        self
    }

    pub fn post<H>(&mut self, path: &'static str, handler: H) -> &mut Self
        where H: Fn(&mut Context) + Send + Sync +'static {
        if let Some(_router) = Arc::get_mut(&mut self.router) {
            _router.post(path, Box::new(handler));
        }
        self
    }

    pub fn errors<H>(&mut self, code: StatusCode, handler: H) -> &mut Self
        where H: Fn(&mut Context) + Send + Sync +'static {
        if let Some(_router) = Arc::get_mut(&mut self.router) {
            _router.errors(code, Box::new(handler));
        }
        self
    }

    pub fn assets<P: AsRef<Path>>(&mut self, _path: P) {

    }
}

impl Gin {
    pub fn bind(&self, addr: SocketAddr) -> Result<Server<GinNewService, Body>> {
        Http::new().bind(&addr, GinNewService(self.router.clone()))
    }
}
