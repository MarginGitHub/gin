pub use self::Method::*;

pub enum Method {
    Get,
    Post,
    Head,
    Put,
    Trace,
    Options,
    Delete,
}

impl<'s> From<&'s str> for Method {
    fn from(_method: &'s str) -> Self {
        match _method {
            "GET" => Get,
            "POST" => Post,
            "HEAD" => Head,
            "PUT" => Put,
            "TRACE" => Trace,
            "OPTIONS" => Options,
            "DELETE" => Delete,
            _ => unreachable!()
        }
    }
}