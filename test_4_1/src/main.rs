//onership
fn main() {
// voi stack thi copy oke,co the dung x sau khi copy
    let x = 5;
    let y = x;
    println!("y = {y}");
    println!("x = {x}");

    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("str = {s}");
//move
    let s1 = String::from("hello");
    let s2 = s1; // chuyen quyen so huu vung nho heap cua s1 sang s2, s1 khong the su dung nua
    // println!("s1 = {s1}"); //error vi s1 khong con hop le s1 da move sang s2

    println!("s2 = {s2}");

// copy trong heap co the dung ham clone()
    let s3 = String::from("hello");
    let s4 = s3.clone();
    println!("s3 = {s3}");
    println!("s4 = {s4}");

//onership and function
    let s5 = String::from("hello");
    takes_onership(s5);
    // println!("s5 = {s5}");

    let number = 5;
    make_copy(5);
    println!("number = {number}");


}

fn takes_onership(some_string: String) {
    println!("some_string = {some_string}")
}

fn make_copy(some_integer: i32) {
    println!("some_integer = {some_integer}");
}