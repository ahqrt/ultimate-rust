// struct User {
//     active: bool,
//     username: String,
//     email: String,
//     sign_in_count: u64,
// }

// struct Color(i32, i32, i32);
// struct Point(i32, i32, i32);

// fn main() {
//     let mut user1 = User {
//         active: true,
//         username: String::from("someusername123"),
//         email: String::from("someone@example.com"),
//         sign_in_count: 1,
//     };

//     user1.email = String::from("anotheremail@example.com");

//     let user2 = User {
//         email: String::from("anotheremail@example.com"),
//         // active: user1.active,
//         // username: user1.username,
//         // sign_in_count: user1.sign_in_count,
//         ..user1
//     };

//     let black = Color(0, 0, 0);
//     let origin = Color(0, 0, 0);
// }

// fn build_user(email: String, username: String) -> User {
//     User {
//         active: true,
//         username,
//         email,
//         sign_in_count: 1,
//     }
// }
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn new(width: u32, height: u32) -> Self {
        Rectangle { width, height }
    }

    fn square(size: u32) -> Self {
        Rectangle::new(size, size)
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle::square(50);
    let rect2 = Rectangle::new(20, 20);
    println!("{:?}", rect1.area());
    println!("Can rect1 hold rect2 ? {}", rect1.can_hold(&rect2));
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
