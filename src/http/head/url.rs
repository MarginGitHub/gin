use serde::Deserialize;
use json::from_str;

#[derive(Debug)]
pub struct Url<'u> {
    _raw: &'u str,
    _segments: Vec<&'u str>,
    _params: Option<Params<'u>>,
}

impl<'u> Url<'u> {
    pub fn segments(&self) -> &[&str] {
        self._segments.as_ref()
    }

    pub fn params(&mut self) -> Option<&mut Params<'u>> {
        self._params.as_mut()
    }
}

impl<'u, 's: 'u> From<&'s str> for Url<'u> {
    fn from(url: &'s str) -> Self {
        let mut us = url.splitn(2, "?");
        match (us.next(), us.next()) {
            (Some(_path), Some(_params)) => {
                let _segments: Vec<&str> = _path.split("/")
                    .filter(|_segment: &&str| _segment.len() > 0)
                    .collect();
                let _params = Params::from(_params);
                Url{_raw: url, _segments, _params: Some(_params)}
            },
            (Some(_path), None) => {
                let _segments: Vec<&str> = _path.split("/")
                    .filter(|_segment: &&str| _segment.len() > 0)
                    .collect();
                Url{_raw: url, _segments, _params: None}
            },
            _ => unreachable!(),
        }
    }
}


#[derive(Debug)]
pub struct Params<'u> {
    _raw: &'u str,
    _inner: Vec<(&'u str, &'u str)>,
    _json: Option<String>,
}

impl<'u> Params<'u> {
    pub fn json<T>(&mut self) -> Option<T>
        where T: for<'de> Deserialize<'de>
    {
        let mut s = None;
        if self._json.is_none() {
            let mut json_str = String::from("{");
            for &(_k, _v) in self._inner.iter().as_ref() {
                json_str.push_str(&format!("\"{}\":\"{}\",", _k, _v));
            }
            let last = json_str.len() - 1;
            json_str.remove(last);
            json_str.push('}');
            s = Some(json_str);
        }
        self._json = s;
        match from_str(self._json.as_ref().unwrap()) {
            Ok(value) => {
                Some(value)
            },
            Err(err) => {
                eprintln!("{:#?}", err);
                None
            }
        }
    }

    pub fn get_param(&self, key: &str) -> Option<&str> {
        for &(_k, _v) in self._inner.iter().as_ref() {
            if _k == key {
                return Some(_v);
            }
        }
        None
    }
}

impl<'u, 's: 'u> From<&'s str> for Params<'u> {
    fn from(s: &'s str) -> Self {
        let _inner: Vec<(&str, &str)> = s.split("&")
            .map(|_s| _s.trim())
            .filter(|_s| {
                if !_s.contains("=") {
                    return false;
                }
                if _s.len() == "=".len() {
                    return false;
                }
                if _s.starts_with("=") {
                    return false;
                }
                true
            })
            .map(|_s| {
                let mut _ss = _s.splitn(2, "=");
                match (_ss.next(), _ss.next()) {
                    (Some(_key), Some(_value)) => (_key.trim(), _value.trim()),
                    _ => unreachable!()
                }
            })
            .collect();
        Params{_raw: s, _inner, _json: None}
    }
}
//
//#[derive(Debug)]
//enum Json<T> {
//    Empty,
//    Raw(String),
//    Value(T),
//}
//
//impl<T> Json<T>
//    where T: for<'de> Deserialize<'de>
//{
//    pub fn is_empty(&self) -> bool {
//        match self {
//            &Empty => true,
//            _ => false,
//        }
//    }
//
//    pub fn is_raw(&self) -> bool {
//        match self {
//            &Raw(_) => true,
//            _ => false,
//        }
//    }
//
//    pub fn is_value(&self) -> bool {
//        match self {
//            &Value(_) => true,
//            _ => false,
//        }
//    }
//
//    pub fn raw(&self) -> &str {
//        match self {
//            &Raw(ref _s) => _s,
//            _ => unreachable!()
//        }
//    }
//
//    pub fn value(&self) -> &T {
//        match self {
//            &Value(ref _v) => _v,
//            _ => unreachable!()
//        }
//    }
//
//    pub fn replace(&self, other: Self) {
//        unsafe {
//            replace(&mut *(self as *const _ as *mut Json<T>), other);
//        }
//    }
//}