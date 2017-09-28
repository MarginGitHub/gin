use std::collections::HashMap;
use std::io::{Result, Error, ErrorKind};

use hyper::StatusCode;

use context::Context;
use method::Handler;

pub struct Router {
    get: HashMap<String, Box<Handler>>,
    post: HashMap<String, Box<Handler>>,
}

impl Router {
    pub fn new() -> Self {
        Router { get: HashMap::new(), post: HashMap::new() }
    }
}

impl Router {
    pub fn insert_get<P: Into<String>>(&mut self, path: P, handler: Box<Handler>) {
        self.get.insert(path.into(), handler);
    }

    pub fn insert_post<P: Into<String>>(&mut self, path: P, handler: Box<Handler>) {
        self.post.insert(path.into(), handler);
    }
}

impl Router {
    fn handle(&self, c: &mut Context, path: &str, r: &HashMap<String, Box<Handler>>) -> Result<()> {
        r.get(path)
            .map(|_handler| {
                _handler(c);
                ()
            })
            .or_else(|| {
                c.error(StatusCode::NotFound, "404");
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
            _ => {Ok(())}
        }
    }
}
