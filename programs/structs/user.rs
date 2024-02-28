struct AlwaysEqual;
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);


struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}

fn main() {
    let user = User {
        active: true,
        username: String::from("Ahsan"),
        email: "aj@gmail.com".to_string(),
        sign_in_count: 12
    };

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    let subject = AlwaysEqual;
}