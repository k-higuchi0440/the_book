fn main() {
    let user1 = build_user(
        String::from("someone@example.com"),
        String::from("some_username123"),
    );
    let user2 = User {
        email: String::from("anotheremail@example.com"),
        username: String::from("another_username567"),
        ..user1 // update syntax
    };

    // derived Debug
    println!("{:?}", user2);

    // pretty print
    println!("{:#?}", user2);

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1) // function call
    );

    let rect2 = Rectangle::square(50);
    println!(
        "The area of the rectangle is {} square pixels.",
        rect2.area() // method call
    );
}

#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    User {
        email,    // shorthand
        username, // shorthand
        active: true,
        sign_in_count: 1,
    }
}

// different tuple structs
#[allow(dead_code)]
struct Color(i32, i32, i32);
#[allow(dead_code)]
struct Point(i32, i32, i32);

struct Rectangle {
    width: u32,
    height: u32,
}

#[allow(dead_code)]
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // Associated Functions, its like Static Method
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
