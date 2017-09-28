use std::collections::HashMap;
use std::io::{Result, Error, ErrorKind};

use hyper::StatusCode;

use context::Context;
use method::Handler;

pub struct Router {
    get: HashMap<&'static str, Box<Handler>>,
    post: HashMap<&'static str, Box<Handler>>,
    error: HashMap<StatusCode, Box<Handler>>,
}

impl Router {
    pub fn new() -> Self {
        Router {
            get: HashMap::new(),
            post: HashMap::new(),
            error: HashMap::new(),
        }
    }
}

impl Router {
    pub fn insert_get(&mut self, path: &'static str, handler: Box<Handler>) {
        self.get.insert(path, handler);
    }

    pub fn insert_post(&mut self, path: &'static str, handler: Box<Handler>) {
        self.post.insert(path, handler);
    }

    pub fn set_error(&mut self, code: StatusCode, handler: Box<Handler>) {
        self.error.insert(code, handler);
    }
}

impl Router {
    fn handle(&self, c: &mut Context, path: &str, r: &HashMap<&'static str, Box<Handler>>) -> Result<()> {
        r.get(path)
            .map(|_handler| {
                _handler(c);
                ()
            })
            .or_else(|| {
                if let Some(ref _err_handler) = self.error.get(&StatusCode::NotFound) {
                    _err_handler(c);
                } else {
                    c.error(StatusCode::NotFound, "404");
                }
                Some(())
            })
            .ok_or(Error::new(ErrorKind::NotFound, "未知错误"))
    }

    pub fn dispatch(&self, c: &mut Context) -> Result<()> {
        let method = c.req.method().as_ref();
        let path = c.req.uri().path();
        match method {
            "GET" => {
                self.handle(c, path, &self.get)
            }
            "POST" => {
                self.handle(c, path, &self.post)
            }
            _ => { Ok(()) }
        }
    }
}
