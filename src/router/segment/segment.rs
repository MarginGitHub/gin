pub use self::Segment::*;
use router::segment::*;


#[derive(Debug)]
pub enum Segment {
    Normal(&'static str),
    Pattern(PatternSegment),
}

impl From<&'static str> for Segment {
    fn from(s: &'static str) -> Self {
        match (s.find('{'), s.find('}')) {
            (Some(_), Some(_)) => {
                let _s = s.trim_matches(|b| b == ' ' || b == '\t' || b == '{' || b == '}');
                Pattern(PatternSegment::from(_s))
            },
            (_, _) => {
                let _s = s.trim_matches(|b| b == ' ' || b == '\t' || b == '{' || b == '}');
                Normal(_s)
            },
        }
    }
}

impl Segment {
    pub fn is_normal(&self) -> bool {
        match self {
            &Normal(_) => true,
            _ => false,
        }
    }
    pub fn is_pattern(&self) -> bool {
        match self {
            &Pattern(_) => true,
            _ => false,
        }
    }

    pub fn pattern(&self) -> &PatternSegment {
        match self {
            &Pattern(ref _pattern) => _pattern,
            _ => unreachable!()
        }
    }

    pub fn is_equal(&self, s: &str) -> bool {
        match self {
            &Normal(_s) => _s == s,
            &Pattern(ref _pattern) => {
                _pattern.is_equal(s)
            }
        }
    }

    pub fn prority(&self) -> Priority {
        match self {
            &Normal(_) => P0,
            &Pattern(ref _pattern) => _pattern.prority(),
        }
    }
}

