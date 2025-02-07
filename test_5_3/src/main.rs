fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    println!("the area of rectangle is {} square pixels",rect.area());
    println!("The rectangle has a nonzero width; it is {}",rect.width());
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }
}
