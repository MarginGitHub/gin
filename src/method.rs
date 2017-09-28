use context::Context;

pub type Handler = Fn(&mut Context) + 'static;


pub fn get(_path: &str, _handler: Box<Handler>) {
    unsafe {
        if let Some(_router) = ::ROUTER.as_mut() {
            _router.insert_get(_path, _handler);
        }
    }
}