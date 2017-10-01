use std::ops::Deref;

pub use self::HttpVersion::*;

#[derive(Clone, Copy)]
pub enum HttpVersion {
    V09,
    V10,
    V11,
    V20,
}

impl Deref for HttpVersion {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        match *self {
            V09 => "HTTP/0.9",
            V10 => "HTTP/1.0",
            V11 => "HTTP/1.1",
            V20 => "HTTP/2.0",
        }
    }
}

impl Default for HttpVersion {
    fn default() -> Self {
        V11
    }
}

impl<'s> From<&'s str> for HttpVersion {
    fn from(s: &'s str) -> Self {
        match s {
            "HTTP/0.9" => V09,
            "HTTP/1.0" => V10,
            "HTTP/1.1" => V11,
            "HTTP/2.0" => V20,
            _ => unimplemented!(),
        }
    }
}
