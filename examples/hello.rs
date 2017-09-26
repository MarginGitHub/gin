extern crate gin;
use gin::bytes::BytesMut;

fn main() {
    let data = "Hello:   \t12356728\r";
    println!("{}", data.replace(|c| c == ' ' || c == '\t' || c == 'r', ""));
}