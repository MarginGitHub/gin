pub extern crate futures;
pub extern crate futures_cpupool;
pub extern crate hyper;

extern crate serde;
extern crate serde_json;


pub mod service;
pub mod context;
pub mod method;
mod param;
mod router;

pub use method::get;
pub use hyper::StatusCode;

use router::Router;
use hyper::server::Http;
use std::sync::Arc;
use std::cell::RefCell;

pub static mut ROUTES: Option<Arc<RefCell<Router>>> = None;

pub fn init() {
    unsafe {
        ROUTES = Some(Arc::new(RefCell::new(Router::new())));
    }
}

pub fn run(addr: &str) {
    let addr = addr.parse().unwrap();
    let server = Http::new()
        .bind(&addr, || Ok(service::GinService(unsafe{ROUTES.as_ref().unwrap().clone()})))
        .unwrap();
    server.run().unwrap();
}


