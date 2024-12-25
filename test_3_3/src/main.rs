fn main() {
    let a = add();
    let b = sub(4,6);
    println!("Gia tri a = {a}");
    println!("Gia tri b = {b}");
}

fn add() ->i32 {
    let y = {
        let x = 2;
        x + 1
    };
    y
} // x+1 khong co ; thi tinh la mot gia tri tra ve, y cung vay.

// ham tra ve thi can co ->i32.
fn sub(x: i32, y:i32) -> i32 {
    let z = y - x;
    return z;
}