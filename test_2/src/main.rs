use std::io;

fn main() {
    println!("Guess the number!");

    printfln!("Please input your guess");

    let mut guess = String::new();
    
    io::stdin()
        .read_line(&mut guess)
    let x = 5;
    let y = 10;
    
    println!("x = {x} and y + 2 = {}", y + 2);
}