struct AlwaysEqual;
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct User<'a> {
    active: bool,
    username: &'a str,
    email: &'a str,
    sign_in_count: u64,
}

fn main() {
    let user = User {
        active: true,
        username: "hello",
        email: "aj@gmail.com",
        sign_in_count: 12,
    };

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    let subject = AlwaysEqual;
}
