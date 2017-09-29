use std::ops::{Deref};

#[derive(Debug)]
pub struct Segments {
    _inner: Vec<Segment>,
}

impl From<&'static str> for Segments {
    fn from(s: &'static str) -> Self {
        let inner = s.split('/')
            .filter(|s| s.len() != 0 )
            .map(|s| Segment::from(s))
            .collect();
        Segments{_inner: inner}
    }
}

impl Deref for Segments {
    type Target = [Segment];

    fn deref(&self) -> &Self::Target {
        self._inner.as_ref()
    }
}


#[derive(Debug)]
pub enum Segment {
    Normal(&'static str),
    Pattern(SegmentPattern),
}

impl From<&'static str> for Segment {
    fn from(s: &'static str) -> Self {
        match (s.find('{'), s.find('}')) {
            (Some(_), Some(_)) => Segment::Pattern(SegmentPattern::from(s)),
            (_, _) => Segment::Normal(s),
        }
    }
}

#[derive(Debug)]
pub enum SegmentPattern {
    Named(&'static str),
    SimplePattern(&'static str),
    NamedPattern(&'static str, &'static str),
    Empty,
}

impl From<&'static str> for SegmentPattern {
    fn from(s: &'static str) -> Self {
        let mut ss = s.splitn(2, ':');
        match (ss.next(), ss.next()) {
            (Some(_name), None) => {
                let name = _name.trim();
                if name.len() != 0 {
                    SegmentPattern::Named(name)
                } else {
                    SegmentPattern::Empty
                }
            },
            (Some(_name), Some(_pattern)) => {
                let name = _name.trim();
                let pattern = _pattern.trim();
                match (name.len(), pattern.len()) {
                    (0, 0) => SegmentPattern::Empty,
                    (_ , 0) => SegmentPattern::Named(name),
                    (0, _) => SegmentPattern::SimplePattern(pattern),
                    (_, _) => SegmentPattern::NamedPattern(name, pattern),
                }

            },
            (None, None) => SegmentPattern::Empty,
            (_, _) => {
                unreachable!()
            }
        }
    }
}