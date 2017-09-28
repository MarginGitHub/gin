use context::Context;
use hyper::StatusCode;

pub type Handler = Fn(&mut Context) + 'static;


pub fn get(_path: &'static str, _handler: Box<Handler>) {
    unsafe {
        if let Some(_router) = ::ROUTER.as_mut() {
            _router.insert_get(_path, _handler);
        }
    }
}
pub fn post(_path: &'static str, _handler: Box<Handler>) {
    unsafe {
        if let Some(_router) = ::ROUTER.as_mut() {
            _router.insert_post(_path, _handler);
        }
    }
}

pub fn error(code: StatusCode, _handler: Box<Handler>) {
    unsafe {
        if let Some(_router) = ::ROUTER.as_mut() {
            _router.set_error(code, _handler);
        }
    }
}
