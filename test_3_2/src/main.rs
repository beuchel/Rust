use std::io;
fn main() {
    let guess: i8 = 18;
    let tup: (i8,u16,f64,i32) = (56,23,17.7,32);
    println!("a = {}",tup.0);
    println!("tup3 = {}",tup.3);
    let (x,y,z,e) = tup;
    println!("x = {x}, y = {y}, z = {z},e = {e}");
    println!{"guess = {guess}"};


    let mut arr1: [i32;5];
    let arr = [1,2,3,4,5];

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = arr[index];
    println!("The value of element at index[{index}] is: {element}");
}
