use std::path;
use std::collections::HashMap;
use std::iter::{Skip};
use std::slice::Iter;

use url::UrlSegments;
use router::segment::*;
pub use self::Value::*;

#[derive(Debug)]
pub struct  ParamsPattern {
    _inner: HashMap<String, Value>,
}

impl ParamsPattern {
    pub fn content(&self, key: &str) -> Option<&str> {
        match self._inner.get(key) {
            Some(&Content(ref _s)) => {
                Some(_s)
            },
            _ => None,
        }
    }

    pub fn path(&self, key: &str) -> Option<&path::Path> {
        match self._inner.get(key) {
            Some(&Path(ref _path)) => {
                Some(_path.as_ref())
            },
            _ => None,
        }
    }
}

impl<'r> From<(&'r UrlSegments<'r>, &'r Segments)> for ParamsPattern {
    fn from((usegments, segments): (&'r UrlSegments<'r>, &'r Segments)) -> Self {
        let mut anonymous = -1;
        let tmp: Vec<(String, Value)> = usegments.iter()
            .zip(segments.iter().as_ref())
            .enumerate()
            .filter(|&(_, (_, _segment))| {
                _segment.is_pattern()
            })
            .map(|(_idx, (_s, _segment))| {
                (_idx, *_s, _segment.pattern())
            })
            .map(|(_idx, _s, _pattern)| {
                match _pattern {
                    &Empty | &SimplePattern(_) => {
                        anonymous += 1;
                        (format!("${}", anonymous), Value::from(_s))
                    },
                    &Named(name) | &NamedPattern(name, _) => (name.to_string(), Value::from(_s)),
                    &SimplePath => {
                        anonymous += 1;
                        (format!("${}", anonymous), Value::from(usegments.iter().skip(_idx)))
                    },
                    &NamedPath(name) => (name.to_string(), Value::from(usegments.iter().skip(_idx))),
                }
            })
            .collect();
        let mut _inner = HashMap::new();
        for (key, value) in tmp.into_iter() {
            _inner.insert(key, value);
        }
        ParamsPattern{_inner}

    }
}

#[derive(Debug)]
pub enum Value {
    Content(String),
    Path(path::PathBuf),
}

impl<'s> From<&'s str> for Value {
    fn from(s: &'s str) -> Self {
        Content(s.to_string())
    }
}

impl<'s> From<Skip<Iter<'s, &'s str>>> for Value {
    fn from(skip: Skip<Iter<'s, &'s str>>) -> Self {
        let mut path = path::PathBuf::new();
        for s in skip {
            path = path.join(s);
        }
        println!("{:#?}", path);
        Path(path)
    }
}