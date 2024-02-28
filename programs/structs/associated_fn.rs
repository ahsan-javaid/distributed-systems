// std::fmt::Display
#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }

    fn square(size: u32) -> Self {
        Self {
            height: size,
            width: size,
        }
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect = Rectangle {
        height: 10,
        width: 10,
    };

    println!("Here is rect: {:?}", rect);
    println!("Here is rect: {:?}", rect.area());
    println!("Here is rect: {:?}", Rectangle::square(12));

    dbg!(&rect);
}
