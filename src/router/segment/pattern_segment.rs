pub use self::PatternSegment::*;
use router::segment::*;

#[derive(Debug)]
pub enum PatternSegment {
    Named(&'static str),
    SimplePattern(&'static str),
    NamedPattern(&'static str, &'static str),
    SimplePath,
    NamedPath(&'static str),
    Empty,
}

impl PatternSegment {
    pub fn is_empty(&self) -> bool {
        match self {
            &Empty => true,
            _ => false,
        }
    }
    pub fn is_equal(&self, s: &str) -> bool {
        match self {
            &Empty => true,
            &Named(_) => true,
            &SimplePattern(_pattern) => s.contains(_pattern),
            &NamedPattern(_, _pattern) => s.contains(_pattern),
            &SimplePath | &NamedPath(_) => true,
        }
    }

    pub fn prority(&self) -> Priority {
        match self {
            &SimplePattern(_) | &NamedPattern(_, _) => P1,
            &Empty | &Named(_) => P2,
            &SimplePath | &NamedPath(_) => P3,
        }
    }
}

impl From<&'static str> for PatternSegment {
    fn from(s: &'static str) -> Self {
        if s.contains(":") {
            let mut ss = s.splitn(2, ':');
            match (ss.next(), ss.next()) {
                (Some(_name), Some(_pattern)) => {
                    let name = _name.trim();
                    let pattern = _pattern.trim();
                    match (name.len(), pattern.len()) {
                        (0, 0) => Empty,
                        (_, 0) => Named(name),
                        (0, _) => SimplePattern(pattern),
                        (_, _) => NamedPattern(name, pattern),
                    }

                },
                (_, _) => {
                    unreachable!()
                }
            }
        } else if s.contains("..") {
            let ss: &str = s.splitn(2, "..").next().unwrap().trim();
            if ss.len() != 0 {
                NamedPath(ss)
            } else {
                SimplePath
            }
        } else {
            let ss = s.trim();
            if ss.len() != 0 {
                Named(ss)
            } else {
                Empty
            }
        }

    }
}