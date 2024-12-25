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
        counter += 1;
        if counter == 10 {
            break counter * 3;
        }
     };
     println!("result = {result}");

     //while
     counter = 2;
     while counter != 0 {
        println!("hello");
        counter -= 1;
     }

     //for
    let arr = [10, 20];
    for element in arr {
        println!("element = {element}");
    }

    for number in (1..=4).rev() {
        println!("number = {number}");
    }
    println!("LIFTOFF!!!");
}
