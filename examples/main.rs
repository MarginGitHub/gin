#[macro_use]
extern crate serde_derive;
extern crate gin;

use gin::context::Context;

fn main() {
    gin::init();
    gin::get("/hello", Box::new(index));
    gin::get("/abc", Box::new(|c| {
        c.string("Hello");
    }));
    gin::run("127.0.0.1:3333");
}

fn index(c: &mut Context) {
    #[derive(Serialize)]
    struct Info<'a> {
        name: Option<&'a str>,
        pwd: Option<&'a str>,
    }
    let mut info = Info{name: None, pwd: None};
    if let (Some(_name), Some(_pwd)) = (c.get_query("name"), c.get_query("pwd")) {
        info.name = Some(_name);
        info.pwd = Some(_pwd);
        c.json(&info)
    } else {
        c.string("无请求参数")
    }
}