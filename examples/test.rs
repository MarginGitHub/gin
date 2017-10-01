#[macro_use]
extern crate serde_derive;
extern crate gin;
use gin::http::Url;

#[derive(Debug, Deserialize)]
struct User {
    name: String,
    pwd: String,
}

fn main() {
    let _url = "/a/b/c/d?name=dong&pwd=123&1 = 123";
    let mut url = Url::from(_url);
//    println!("{:#?}", url);
    let u: User = url.params().unwrap().json().unwrap();
//    println!("{:#?}", u);
    println!("{},\n {:?}", url.params().unwrap().get_param("name").unwrap(), u);
}