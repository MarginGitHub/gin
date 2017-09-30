use std::iter::{Iterator};
use std::slice::Iter;

use router::route::Route;
use router::segment::*;

pub struct PriorityRouter {
    _p0: Vec<Route>,
    _p1: Vec<Route>,
    _p2: Vec<Route>,
    _p3: Vec<Route>,
}


impl PriorityRouter {
    pub fn new() -> Self {
        PriorityRouter {
            _p0: vec![],
            _p1: vec![],
            _p2: vec![],
            _p3: vec![],
        }
    }

    pub fn set(&mut self, route: Route) {
        match route.segments().prority() {
            P0 => self._p0.push(route),
            P1 => self._p1.push(route),
            P2 => self._p2.push(route),
            P3 => self._p3.push(route),
        }
    }

    pub fn get(&self, priority: Priority) -> &[Route] {
        match priority {
            P0 => self._p0.as_ref(),
            P1 => self._p1.as_ref(),
            P2 => self._p2.as_ref(),
            P3 => self._p3.as_ref(),
        }
    }

    pub fn iter(&self) -> PriorityRouterIter {
        PriorityRouterIter {
            _priority: P0,
            _router: self,
            _iter: self.get(P0).iter(),
        }
    }
}


pub struct PriorityRouterIter<'p> {
    _priority: Priority,
    _router: &'p PriorityRouter,
    _iter: Iter<'p, Route>,
}

impl<'p> Iterator for PriorityRouterIter<'p> {
    type Item = &'p Route;

    fn next(&mut self) -> Option<Self::Item> {
        let mut next = self._iter.next();
        while next.is_none() && self._priority < P3 {
            self._priority.decrement();
            self._iter = self._router.get(self._priority).iter();
            next = self._iter.next();
        }
        next
    }
}