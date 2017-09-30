use std::ops::Deref;

use router::segment::*;
use router::Handler;

pub struct Route {
    segments: Segments,
    handler: Box<Handler>,
}

impl Route {
    pub fn new(segments:Segments, handler: Box<Handler>) -> Self{
        Route {
            segments,
            handler,
        }
    }
}

impl Deref for Route {
    type Target = Segments;

    fn deref(&self) -> &Self::Target {
        &self.segments
    }
}

impl Route {
    pub fn segments(&self) -> &Segments {
        &self.segments
    }
    pub fn handler(&self) -> &Handler {
        self.handler.as_ref()
    }
}