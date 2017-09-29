use std::collections::HashMap;

#[derive(Debug)]
pub struct Params<'p> {
    inner: HashMap<&'p str, Vec<&'p str>>,
}

impl<'p> From<&'p str> for Params<'p> {
    fn from(s: &'p str) -> Self {
        let mut ret = Params::new();
        let ss = s.split("&");
        for _s in ss {
            let mut item = _s.split("=");
            if let (Some(_key), Some(_value)) = (item.next(), item.next()) {
                ret.put(_key, _value);
            }
        }
        ret
    }
}

impl<'p> Params<'p> {
    pub fn new() -> Self {
        Params {inner: HashMap::new()}
    }

    pub fn put(&mut self, key: &'p str , value: &'p str) {
        let values = self.inner.entry(key).or_insert(Vec::new());
        values.push(value);
    }
}

impl<'p> Params<'p> {
    pub fn get(&self, _key: &str) -> Option<&str> {
        match self.inner.get(_key) {
            Some(_array) => {
                _array.get(0).map(|s| *s)
            },
            None => None,
        }
    }

    pub fn get_array(&self, _key: &str) -> Option<&[&str]> {
        self.inner.get(_key).map(|v| v.as_ref())
    }
}
