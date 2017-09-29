mod segment;

use std::collections::HashMap;
use std::io::{Result};
use std::ops::Deref;

use hyper::StatusCode;
use hyper::Method;

use context::Context;
use self::segment::{Segments};
use self::segment::Segment::{Normal, Pattern};
use self::segment::SegmentPattern::{Named, NamedPattern, SimplePattern, Empty};

pub type Handler = for<'r> Fn(&'r mut Context<'r>) + Send + Sync +'static;

#[derive(Debug)]
pub struct Router {
    _inner: HashMap<Method, Vec<Route>>,
    _errors: HashMap<StatusCode, Box<Handler>>,
}

#[derive(Debug)]
struct Route {
    segments: Segments,
    handler: Box<Handler>,
}

impl Deref for Route {
    type Target = Segments;

    fn deref(&self) -> &Self::Target {
        &self.segments
    }
}

impl Router {
    pub fn new() -> Self {
        Router {
            _inner: HashMap::new(),
            _errors: HashMap::new(),
        }
    }
}

impl Router {
    pub fn get(&mut self, path: &'static str, handler: Box<Handler>) {
        let v = self._inner.entry(Method::Get).or_insert(vec![]);
        v.push(Route{
            segments: Segments::from(path),
            handler,
        });
    }

    pub fn post(&mut self, path: &'static str, handler: Box<Handler>) {
        let v = self._inner.entry(Method::Post).or_insert(vec![]);
        v.push(Route{
            segments: Segments::from(path),
            handler,
        });
    }

    pub fn errors(&mut self, code: StatusCode, handler: Box<Handler>) {
        self._errors.insert(code, handler);
    }
}

impl Router {
    fn handle(&self, c: & mut Context, routes:  &[Route]) -> Result<()> {
        for route in routes {
            let ret: Vec<_> = route.iter()
                .zip(c.url().segments().iter())
                .filter(|&(&_segment, &_s)| {
                    match _segment {
                        Normal(segment) => !(segment == _s),
                        Pattern(pattern) => {
                            match pattern {
                                Empty => false,
                                Named(name) => {
                                    c.patterns.insert(name, _s);
                                    false
                                },
                                SimplePattern(p) => {
                                    c.patterns.insert(_s, p);
                                    false
                                },
                                NamedPattern(name, p) => {
                                    c.patterns.insert(name, _s);
                                    false
                                }
                            }
                        }
                    }
                })
                .collect();
            if ret.len() == 0 {
                (route.handler.as_ref())(c);
                return Ok(());
            }
        }

        match self._errors.get(&StatusCode::NotFound) {
            Some(_error_handler) => {
                _error_handler(c);
                Ok(())
            },
            None => {
                c.error(StatusCode::NotImplemented, "未实现的方法");
                Ok(())
            }
        }

    }

    pub fn dispatch(&self, c: &mut Context) -> Result<()> {
        if let Some(_routes) = self._inner.get(c.req.method()) {
            self.handle(c, _routes.as_ref())
        } else {
            match self._errors.get(&StatusCode::NotFound) {
                Some(_error_handler) => {
                    _error_handler(c);
                    Ok(())
                },
                None => {
                    c.error(StatusCode::NotImplemented, "未实现的方法");
                    Ok(())
                }
            }
        }
    }
}
