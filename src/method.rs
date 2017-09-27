use context::Context;
use std::marker::{Send, Sync};
use std::ops::{Deref, DerefMut};

pub trait Handler: Send + Sync {
    fn handle(&self, c: &mut Context);
}

impl<F> Handler for F where F: Fn(&mut Context) + Send + Sync{
    fn handle(&self, c: &mut Context) {
        (*self)(c);
    }
}

//impl<F> Handler for F where F: FnMut(&mut Context) + 'static {
//    fn handle(&self, c: &mut Context) {
//        (*self)(c);
//    }
//}
//
//impl<F> Handler for F where F: FnOnce(&mut Context) + 'static {
//    fn handle(&self, c: &mut Context) {
//        (*self)(c);
//    }
//}

//impl Handler for fn(&mut Context) {
//    fn handle(&self, c: &mut Context) {
//        (*self)(c);
//    }
//}

pub fn get(_path: &str, _handler: Box<Handler>) {
//    unsafe {
//        ::ROUTES.get.insert(_path.to_string(), _handler);
//    }
    unsafe {
        if let Some(_arc) = ::ROUTES.as_mut() {
            _arc.deref().borrow_mut().deref_mut().insert(_path.to_string(), _handler);
        }
    }
}