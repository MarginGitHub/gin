mod segment;
mod pattern_segment;
mod priority;

use std::ops::Deref;

pub use router::segment::priority::*;
pub use router::segment::segment::*;
pub use router::segment::pattern_segment::*;



#[derive(Debug)]
pub struct Segments {
    _inner: Vec<Segment>,
    _priority: Priority,
}

impl Segments {
    pub fn prority(&self) -> Priority {
        self._priority
    }
}

impl From<&'static str> for Segments {
    fn from(s: &'static str) -> Self {
        let mut _inner: Vec<Segment> = s.split('/')
            .filter(|s| s.len() != 0 )
            .map(|s| Segment::from(s))
            .collect();
        if _inner.len() == 0 && s.contains('/') {
            _inner.push(Normal("/"));
        }
        let mut _priority = P0;
        for s in _inner.iter() {
            let pri = s.prority();
            if pri > _priority {
                _priority = pri;
            }
        }
        let s = Segments{_inner, _priority};
        println!("{:#?}", s);
        s
    }
}

impl Deref for Segments {
    type Target = [Segment];

    fn deref(&self) -> &Self::Target {
        self._inner.as_ref()
    }
}