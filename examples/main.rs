extern crate gin;

use gin::context::Context;

fn main() {
    gin::init();
    gin::get("/", Box::new(index));
    gin::run("127.0.0.1:3333");
}

fn index(c: &mut Context) {
    match c.get_query("name") {
        Some(val) => {
            println!("value: {}", val)
        },
        None => {
            println!("no search")
        }
    }
    c.string("Hello");
}