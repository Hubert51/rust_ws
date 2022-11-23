use std::arch::x86_64::CpuidResult;

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct AlwaysEqual;

// attribute of debug to print everything in the struct for developer
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    user1.email = String::from("anotheremail@example.com");

    let mut user2 = build_user(String::from("ruijiegeng@gmail.com"), String::from("Hubert"));
    println!("User2.email: {}", user2.email);

    // copy struct:
    // here we already borrow the user2.username to user3. Cannot access user2.username anymore
    let user3 = User{
        email: String::from("gengr@umich.edu"),
        active: false,
        ..user2
    };

    println!("user2.username: {}", user2.email);

    user2.username = String::from("gengr");
    println!("user2.username: {}", user2.username);

    // Tuple struct without named field
    let black = Color(0,0,0);
    println!("first pixel of black: {}", black.0);

    // TODO: probably need more time to be familar
    let subject = AlwaysEqual;


    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    // :?: debug format
    println!("rect1 is {:?}", rect1);

    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1);
    println!("rect1 is {:?}", rect1);

}

fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}