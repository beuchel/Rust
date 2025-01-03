fn main() {
    let s = dangle();
    println!("s = {s}");
}

// fn dangle() -> &String {
//     let s = String::from("hello");
    
//     &s
// }
//error do tra ve con tro lung lo,da bi giai phong khi het pham vi ham.

fn dangle() -> String {
    let s = String::from("hello");
    s
}
