fn main() {
    let info = "name..";
    println!("{:#?}", info.splitn(2, "..").next());
}