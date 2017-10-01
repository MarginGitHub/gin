use std::mem::replace;
use std::path::Path;

use hyper::server::{Request, Response};
use hyper::StatusCode;
use hyper::Headers;

use serde::Serialize;
use json::to_string;

use param::Params;
use html::HTML;
use url::{Url, UrlSegments};
use router::segment::*;
use context::*;

#[derive(Debug)]
pub struct Context<'r> {
    pub req: &'r Request,
    resp: Option<Response>,
    url: Url<'r>,
    patterns: Option<ParamsPattern>,
}

impl<'r> Context<'r> {
    #[inline]
    pub fn new(req: &'r Request) -> Self {
        Context {
            req,
            resp: None,
            url: Url::from(req.uri()),
            patterns: None,
        }
    }

    pub fn url(&self) -> &Url {
        &self.url
    }

    pub fn response(self) -> Option<Response> {
        self.resp
    }

    pub fn patterns(&self) -> Option<&ParamsPattern> {
        self.patterns.as_ref()
    }

}

impl<'r> Context<'r> {
    pub fn parse_patterns(&self, url_segments: &UrlSegments, segments: &Segments) {
        let patterns = ParamsPattern::from((url_segments, segments));
        replace(unsafe {&mut *(&(self.patterns) as *const _ as *mut Option<ParamsPattern>)}, Some(patterns));
    }

    pub fn content(&self, key: &str) -> Option<&str> {
        match self.patterns {
            Some(ref _pattern) => {
                _pattern.content(key)
            },
            None => None,
        }
    }

    pub fn path(&self, key: &str) -> Option<&Path> {
        match self.patterns {
            Some(ref _pattern) => {
                _pattern.path(key)
            },
            None => None,
        }
    }
}


impl<'r> Context<'r> {

    pub fn get_all_query(&self) -> Option<&Params> {
        self.url.querys()
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
        match self.get_all_query() {
            Some(_params) => {
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

    pub fn html<P: AsRef<Path>>(&self, html: P) {
        self.html_with_code(StatusCode::Ok, html);
    }

    pub fn html_with_code<P: AsRef<Path>>(&self, code: StatusCode, html: P) {
        let mut headers = Headers::new();
        headers.set_raw("content_type", "text/html");
        let resp = Response::new()
            .with_status(code)
            .with_headers(headers)
            .with_body(HTML::from(html));
        replace(unsafe {&mut *(&(self.resp) as *const _ as *mut Option<Response>)}, Some(resp));
    }

    pub fn html_with_content<C: Into<String>>(&self, content: C) {
        let mut headers = Headers::new();
        headers.set_raw("content_type", "text/html");
        let resp = Response::new()
            .with_status(StatusCode::Ok)
            .with_headers(headers)
            .with_body(content.into());
        replace(unsafe {&mut *(&(self.resp) as *const _ as *mut Option<Response>)}, Some(resp));
    }
}