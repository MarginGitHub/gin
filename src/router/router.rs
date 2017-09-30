use std::collections::HashMap;

use hyper::{Method, StatusCode, Response, Error, Result};

use context::Context;
use router::priority_router::PriorityRouter;
use router::route::Route;
use router::segment::*;
use router::Handler;

pub struct Router {
    _inner: HashMap<Method, PriorityRouter>,
    _errors: HashMap<StatusCode, Box<Handler>>,
}

impl Router {
    pub fn new() -> Self {
        Router {
            _inner: HashMap::new(),
            _errors: HashMap::new(),
        }
    }

    pub fn inner(&self) -> &HashMap<Method, PriorityRouter> {
        &self._inner
    }
}

impl Router {
    pub fn get(&mut self, path: &'static str, handler: Box<Handler>) {
        let v = self._inner.entry(Method::Get).or_insert(PriorityRouter::new());
        v.set(Route::new(Segments::from(path), handler));
    }

    pub fn post(&mut self, path: &'static str, handler: Box<Handler>) {
        let v = self._inner.entry(Method::Post).or_insert(PriorityRouter::new());
        v.set(Route::new(Segments::from(path), handler));
    }

    pub fn errors(&mut self, code: StatusCode, handler: Box<Handler>) {
        self._errors.insert(code, handler);
    }
}

impl Router {
    fn handle(&self,mut c: Context, router:  &PriorityRouter) -> Result<Response> {
        for route in router.iter() {
            let priority = route.segments().prority();
            if priority < P3 && route.segments().len() != c.url().segments().len(){
                continue;
            }
            let count = c.url()
                .segments()
                .iter()
                .zip(route.segments().iter())
                .filter(|&(&ctx_segment, route_segment)| {
                    !route_segment.is_equal(ctx_segment)
                })
                .count();

            if count == 0 {
                println!("url: {:#?}", c.url().segments());
                println!("pattern: {:#?}", route.segments());
                c.parse_patterns(c.url().segments(), route.segments());
                (route.handler())(&mut c);
                return c.response().ok_or(Error::Method);
            }
        }
        match self._errors.get(&StatusCode::NotFound) {
            Some(_error_handler) => {
                _error_handler(&mut c);
                c.response().ok_or(Error::Method)
            },
            None => {
                c.error(StatusCode::NoContent, "未实现的方法");
                c.response().ok_or(Error::Method)
            }
        }

    }

    pub fn dispatch(&self, mut c: Context) -> Result<Response> {
        if let Some(pri_router) = self._inner.get(c.req.method()) {
            self.handle(c, pri_router)
        } else {
            match self._errors.get(&StatusCode::NotFound) {
                Some(_error_handler) => {
                    _error_handler(&mut c);
                    c.response().ok_or(Error::Method)
                },
                None => {
                    c.error(StatusCode::NotImplemented, "未实现的方法");
                    c.response().ok_or(Error::Method)
                }
            }
        }
    }
}