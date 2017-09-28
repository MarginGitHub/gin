use context::Context;

pub type Handler = Fn(&mut Context) + 'static;


pub fn get(_path: &str, _handler: Box<Handler>) {
    unsafe {
        if let Some(_router) = ::ROUTER.as_mut() {
            _router.insert_get(_path, _handler);
        }
    }
}
pub fn post(_path: &str, _handler: Box<Handler>) {
    unsafe {
        if let Some(_router) = ::ROUTER.as_mut() {
            _router.insert_post(_path, _handler);
        }
    }
}

pub fn error(_handler: Box<Handler>) {
    unsafe {
        if let Some(_router) = ::ROUTER.as_mut() {
            _router.set_error(_handler);
        }
    }
}
