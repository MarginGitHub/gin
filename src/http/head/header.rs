use std::collections::HashMap;

pub struct Headers<'h> {
    _inner: HashMap<&'h str, &'h str>,
}

impl<'h> Headers<'h> {
    pub fn new() -> Self {
        Headers{
            _inner: HashMap::new(),
        }
    }
}

impl<'h, 's: 'h> From<&'s str> for Headers<'h> {
    fn from(_headers: &'s str) -> Self {
        let mut map = HashMap::new();
            for header in _headers.split("\n").map(|s|{
                s.trim_matches(|c| {
                    c == ' ' || c == '\t' || c == '\r'
                }
            )}) {
                let mut h = header.split(":").map(|s| s.trim());
                match (h.next(), h.next()) {
                    (Some(_k), Some(_v)) => {
                        map.insert(_k.trim(), _v.trim());
                    },
                    _ => unreachable!(),
                }
            }
        Headers{
            _inner: map,
        }
    }
}