fn main() {
// Bai toan tinh dien tich hcn.

// use Tuples.
    let rect1 = (30,50);
    println!("The area of the rectangle is {} square pixels.",area(rect1));

// use struct.

    let rect2 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("The area of the rectangle is {} square pixels.",area1(&rect2));
}

struct Rectangle {
    width: u32,
    height: u32,
}

fn area(dimension: (u32, u32)) -> u32 {
    dimension.0 * dimension.1
}

fn area1(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
