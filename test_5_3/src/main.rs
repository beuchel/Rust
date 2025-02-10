fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    let rect1 = Rectangle {
        width: 40,
        height: 60,
    };

    let rect2 = Rectangle {
        width: 50,
        height: 70,
    };

    println!("the area of rectangle is {} square pixels",rect.area());
    println!("The rectangle has a nonzero width; it is {}",rect.width());

    println!("Can rect1 hold rect2? {}",rect1.can_hold(&rect2));
    println!("Can rect2 hold rect? {}",rect2.can_hold(&rect));
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

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}


