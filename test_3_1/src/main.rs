fn main() {
    let mut y = 5;
    println!("x = {y}");
    y= 6;
    println!("x = {y}");

    let x = 5;
    let x = 10;
    println!("x = {x}");

    {
        let x = x + 1;
        println!("x= {x}");
    }
    println!("x = {x}");
    
    const AGE_YOU: u32 = 26;
    println!("my age = {AGE_YOU}");


    let space = "";
    let space = space.len(); // not error because space be defined again
    
    let mut str = "hihihaha";
    // str = str.len(); // error because str be converted frome str to int 
    println!("str == {str}");
}
