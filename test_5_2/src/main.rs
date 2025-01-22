fn main() {
// Bai toan tinh dien tich hcn.

// use Tuples.
let rect1 = (30,50);
println!("The area of the rectangle is {} square pixels.",area(rect1));
}

fn area(dimension: (u32, u32)) -> u32 {
    dimension.0 * dimension.1
}
