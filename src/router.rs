use std::collections::HashMap;
use method::Handler;
use std::ops::{Deref, DerefMut};

pub struct Router {
    pub get: HashMap<String, Box<Handler>>,
}

impl Router {
    pub fn new() -> Self {
        Router{get: HashMap::new()}
    }
}

impl Deref for Router {
    type Target = HashMap<String, Box<Handler>>;
    fn deref(&self) -> &Self::Target {
        &(self.get)
    }
}

impl DerefMut for Router {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut(self.get)
    }
}