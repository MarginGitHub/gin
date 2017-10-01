use super::*;


pub struct RequestHead<'r> {
    _line: RequestLine<'r>,
    _headers: Headers<'r>,
}

pub struct RequestLine<'u>(Method, Url<'u>, HttpVersion);

impl<'u, 's: 'u> From<&'s str> for RequestLine<'u> {
    fn from(_line: &'s str) -> Self {
        let mut line = _line.split(|b| b == ' ' || b == '\t')
            .filter(|_s| _s.len() > 0);
        match (line.next(), line.next(), line.next()) {
            (Some(_method), Some(_url), Some(_version)) => {
                let method = Method::from(_method);
                let url = Url::from(_url);
                let version = HttpVersion::from(_version);
                RequestLine(method, url, version)
            },
            _ => unreachable!()
        }
    }
}