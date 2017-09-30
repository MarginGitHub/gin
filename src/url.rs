use std::ops::Deref;

use param::Params;

#[derive(Debug)]
pub struct Url<'r> {
    raw: &'r str,
    segments: UrlSegments<'r>,
    querys: Option<Params<'r>>,
}

impl<'r> From<&'r ::hyper::Uri> for Url<'r> {
    fn from(url: &'r ::hyper::Uri) -> Self {
        let mut querys = None;
        if let Some(_query) = url.query() {
            querys = Some(Params::from(_query));
        }
        Url {
            raw: url.as_ref(),
            segments: UrlSegments::from(url.path()),
            querys: querys,
        }
    }
}

impl<'r> Url<'r> {
    pub fn raw(&self) -> &str {
        self.raw
    }

    pub fn segments(&self) -> &UrlSegments {
        &self.segments
    }

    pub fn querys(&self) -> Option<&Params> {
        self.querys.as_ref()
    }

}

#[derive(Debug)]
pub struct UrlSegments<'r> {
    _inner: Vec<&'r str>,
}

impl<'r> From<&'r str> for UrlSegments<'r> {
    fn from(s: &'r str) -> Self {
        let mut _inner: Vec<&'r str> = s.split('/')
            .filter(|s| s.len() != 0 )
            .collect();
        if _inner.len() == 0 && s.contains('/') {
            _inner.push("/");
        }
        UrlSegments{_inner}
    }
}

impl<'r> Deref for UrlSegments<'r> {
    type Target = [&'r str];

    fn deref(&self) -> &Self::Target {
        self._inner.as_ref()
    }
}