use std::mem::replace;

use hyper::server::{Request, Response};
use hyper::StatusCode;
use hyper::Headers;

use serde::Serialize;
use serde_json::to_string;

use param::Params;

#[derive(Debug)]
pub struct Context<'r> {
    pub req: &'r Request,
    resp: Option<Response>,
    params: Option<Params<'r>>,
}

impl<'r> Context<'r> {
    #[inline]
    pub fn new(req: &'r Request) -> Self {
        Context { req, resp: None, params: None }
    }

    pub fn response(self) -> Option<Response> {
        self.resp
    }
}


impl<'r> Context<'r> {
    pub fn init_query(&mut self) {
        let _ = self.req.uri().query().map(|_query| {
            let params = Params::from(_query);
            self.params = Some(params);
            ()
        });
    }

    pub fn get_all_query(&self) -> Option<&Params> {
        self.params.as_ref()
    }

    pub fn get_query(&self, key: &'r str) -> Option<&str> {
        self.get_query_array(key).map(|_array| {
            match _array.get(0) {
                Some(s) => s,
                None => "",
            }
        })
    }

    pub fn get_query_array(&self, key: &'r str) -> Option<&[&str]> {
        match self.params {
            Some(ref _params) => {
                _params.get_array(key)
            },
            None => None
        }
    }
}

impl<'r> Context<'r> {
    pub fn string<C: Into<String>>(&self, content: C) {
        self.string_with_code(StatusCode::Ok, content);
    }

    pub fn string_with_code<C: Into<String>>(&self, code: StatusCode, content: C) {
        let mut headers = Headers::new();
        headers.set_raw("content_type", "text/plain");
        let resp = Response::new()
            .with_status(code)
            .with_headers(headers)
            .with_body(content.into());
        replace(unsafe {&mut *(&(self.resp) as *const _ as *mut Option<Response>)}, Some(resp));
    }

    pub fn json<T: Serialize>(&self, obj: &T) {
        self.json_with_code(StatusCode::Ok, obj);
    }

    pub fn json_with_code<T: Serialize>(&self, code: StatusCode, obj: &T) {
        let mut headers = Headers::new();
        let mut resp = Response::new();
        match to_string(obj) {
            Ok(s) => {
                headers.set_raw("content_type", "application/json");
                resp = resp.with_status(code)
                    .with_headers(headers)
                    .with_body(s);
            }
            Err(err) => {
                headers.set_raw("content_type", "text/plain");
                resp = resp
                    .with_status(StatusCode::Ok)
                    .with_headers(headers)
                    .with_body(format!("{}", err));
            }
        }
        replace(unsafe {&mut *(&(self.resp) as *const _ as *mut Option<Response>)}, Some(resp));
    }

    pub fn error<E: Into<String>>(&self, code: StatusCode, extra: E) {
        let mut headers = Headers::new();
        headers.set_raw("content_type", "text/html");
        let resp = Response::new()
            .with_status(code)
            .with_headers(headers)
            .with_body(extra.into());
        replace(unsafe {&mut *(&(self.resp) as *const _ as *mut Option<Response>)}, Some(resp));
    }
}