#[macro_use]
extern crate serde_derive;
extern crate gin;

use gin::Context;
use gin::hyper::StatusCode;
use gin::Gin;

fn main() {
    //    gin::init();
    //    gin::get("/hello", Box::new(index));
    //    gin::get("/abc", Box::new(|c| {
    //        c.string("Hello");
    //    }));
    //    gin::error(StatusCode::NotFound, Box::new(error));
    //    gin::error(StatusCode::NoContent, Box::new(|c| {
    //        c.string(format!("url: {}, No Content!", c.req.uri().as_ref()));
    //    }));
    //    gin::run("127.0.0.1:3333");
    let _ = Gin::new()
        .get("/", |c| {
            c.html("assets/index.html");
        })
        .get("/abc", |c| {
            c.string("Hello, abc");
        })
        .get("/params", index)
        .errors(StatusCode::NotFound, error)
        .bind("127.0.0.1:3333".parse().unwrap())
        .map_err(|err| {
            eprintln!("{}", err);
            ()
        })
        .map(|server| {
            let _ = server.run().map_err(|err| {
                eprintln!("{}", err);
                ()
            });
            ()
        });
}

fn index(c: &mut Context) {
    #[derive(Serialize)]
    struct Info<'a> {
        name: Option<&'a str>,
        pwd: Option<&'a str>,
    }
    let mut info = Info { name: None, pwd: None };
    if let (Some(_name), Some(_pwd)) = (c.get_query("name"), c.get_query("pwd")) {
        info.name = Some(_name);
        info.pwd = Some(_pwd);
        c.json(&info)
    } else {
        c.string("无请求参数")
    }
}

fn error(c: &mut Context) {
    c.string(format!("error: {}", c.req.uri().as_ref()));
}