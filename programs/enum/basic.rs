#[derive(Debug)]
enum IPAddrType {
    v4(String),
    v6,
}
fn main() {
    let t = IPAddrType::v4("helo".to_string());
    println!("type: {:?}", t);
}
