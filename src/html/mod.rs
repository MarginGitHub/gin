use hyper::Body;
use std::path::{Path};
use std::fs::OpenOptions;
use std::io::{BufReader, Read};

pub struct HTML{
    content: String,
}

impl Into<Body> for HTML {
    fn into(self) -> Body {
        self.content.into()
    }
}
//
//impl<'s> From<&'s str> for HTML {
//    fn from(s: &'s str) -> Self {
//        HTML{content: s.to_string()}
//    }
//}
//
//impl From<String> for HTML {
//    fn from(s: String) -> Self {
//        HTML{content: s}
//    }
//}
//
//impl From<Vec<u8>> for HTML {
//    fn from(v: Vec<u8>) -> Self {
//        match String::from_utf8(v) {
//            Ok(s) => HTML{content: s},
//            Err(err) => {
//                HTML{content: format!("{}", err)}
//            }
//        }
//    }
//}

impl<T> From<T> for HTML
    where T: AsRef<Path> {
    fn from(path: T) -> Self {
        if path.as_ref().exists() && path.as_ref().is_file() {
            match OpenOptions::new().read(true).open(path) {
                Ok(_file) => {
                    let mut content = String::new();
                    match BufReader::new(_file).read_to_string(&mut content) {
                        Ok(_) => {
                            HTML {content}
                        },
                        Err(err) => {
                            HTML{content: format!("{}", err)}
                        }
                    }
                },
                Err(err) => {
                    HTML{content: format!("{}", err)}
                }
            }
        } else {
            HTML{content: format!("html file path is error!")}
        }
    }
}

impl HTML {
    pub fn default() -> Self {
        HTML{content: String::from("")}
    }
}


