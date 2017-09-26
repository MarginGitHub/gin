use hyper::server::{Request, Response};

use param::Params;

#[derive(Debug)]
pub struct Context<'r> {
    pub req: &'r Request,
    pub resp: &'r mut Response,
    params: Option<Params<'r>>,
}

impl<'r> Context<'r> {
    #[inline]
    pub fn new(req: &'r Request, resp: &'r mut Response) ->Self {
        Context{req, resp, params: None}
    }
}


impl<'r> Context<'r> {
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