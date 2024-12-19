fn main() {
    let number = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4,3 or 2");
    }


    // co the dung cau lenh de gan voi if
    let condition = true;
    let number = if condition {2} else {1};
    println!("number = {number}");   


    // loop
    let mut counter = 0;
    let result = loop {
        counter + = 1;
        if counter == 10 {
            break counter * 3;
        }
     }
     println!("counter = {counter}");
}
