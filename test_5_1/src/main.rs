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
    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("hiep222@gmail.com"),
        sign_in_count: user1.sign_in_count,
    };
// after use update syntax.
    let mut user3 = User {
        email: String::from("abc@gmail.com"),
        ..user2
    };
// Giong nhu phep gan,nen tru email ra thi cac thuoc tinh cua user2 da chuyen qua user3
// va user 2 khong con so huu cac gia tri tren heap nua.
//     println!("username of user1 = {}",user1.username); //error
//     println!("username of user1 = {}",user2.username); //error
    println!("email of user2 = {}",user2.email); // valid vi khong chuyen quyen so huu
    println!("sign in count of user2 = {}",user2.sign_in_count); // la copy tren stack
// nen van hop le
    
//// tuple struct khong co truong.
    
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

// tuple struct khong co truong.
struct Color (i32, i32, i16);


