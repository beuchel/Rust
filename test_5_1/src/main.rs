fn main() {
    println!("Hello, world!");
    let mut user1 = User {
        active: true,
        username: String::from("hiep"),
        email: String::from("hiep0218@gmail.com"),
        sign_in_count: 1,
    };
    user1.email = String::from("hiepnd0218@gmail.com");
    println!("mail of username1 is {}",user1.email);

//struct update syntax.
//not update syntax.
    let mut user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("hiep222@gmail.com"),
        sign_in_count: user1.sign_in_count,
    };
// after use update syntax.
    let mut user3 = user {
        email: String::from("abc@gmail.com"),
        ..user1
    };
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User{
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}

// using the field init shorland. Chi can dat bien tham so trung voi bien cua struct.
fn build_user1(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

