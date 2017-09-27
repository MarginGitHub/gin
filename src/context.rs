use hyper::server::{Request, Response};
use hyper::StatusCode;
use hyper::Headers;

use serde::Serialize;
use serde_json::to_string;

use param::Params;

#[derive(Debug)]
pub struct Context<'r> {
    req: &'r Request,
    resp: Option<Response>,
    params: Option<Params<'r>>,
}

impl<'r> Context<'r> {
    #[inline]
    pub fn new(req: &'r Request) ->Self {
        Context{req, resp: None, params: None}
    }

    pub fn response(self) -> Option<Response> {
        self.resp
    }
}


impl<'r> Context<'r> {
    pub fn get_all_query(&mut self) -> Option<&Params> {
        match self.params {
            Some(ref _params) => Some(_params),
            None => {
                match self.req.uri().query() {
                    Some(_query) => {
                        let params = Params::from(_query);
                        self.params = Some(params);
                        self.params.as_ref()
                    },
                    None => {
                        None
                    }
                }
            }
        }
    }
    pub fn get_query(&mut self, key: &'r str) -> Option<&str> {
       self.get_query_array(key).map(|_array| {
           match _array.get(0) {
               Some(s) => s,
               None => "",
           }
       })
    }

    pub fn get_query_array(&mut self, key: &'r str) -> Option<&[&str]> {
        match self.params {
            Some(ref _params) => {
                _params.get_array(key)
            },
            None => {
                match self.req.uri().query() {
                    Some(_query) => {
                        let params = Params::from(_query);
                        self.params = Some(params);
                        self.params.as_ref().unwrap().get_array(key)
                    },
                    None => {
                        None
                    }
                }

            }
        }
    }
}

impl<'r> Context<'r> {
    pub fn string(&mut self, content: &str) {
        let mut headers = Headers::new();
        headers.set_raw("content_type", "text/plain");
        let resp =Response::new()
            .with_status(StatusCode::Ok)
            .with_headers(headers)
            .with_body(content.to_string());
        self.resp = Some(resp);
    }

    pub fn string_with_code(&mut self, code: StatusCode, content: &str) {
        let mut headers = Headers::new();
        headers.set_raw("content_type", "text/plain");
        let resp =Response::new()
            .with_status(code)
            .with_headers(headers)
            .with_body(content.to_string());
        self.resp = Some(resp);
    }

    pub fn json<T: Serialize>(&mut self, code: StatusCode, obj: &T) {
        let mut headers = Headers::new();
        let mut resp =Response::new();
        match to_string(obj) {
            Ok(s) => {
                headers.set_raw("content_type", "application/json");
                resp = resp.with_status(code)
                    .with_headers(headers)
                    .with_body(s);
            },
            Err(err) => {
                headers.set_raw("content_type", "text/plain");
                resp = resp
                    .with_status(StatusCode::Ok)
                    .with_headers(headers)
                    .with_body(format!("{}", err));

            }
        }
        self.resp = Some(resp);
    }
}