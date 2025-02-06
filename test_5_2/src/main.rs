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
// debug in ra rect2
    println!("Rectangle is {rect2:?}");

// su dung dbg! de tra ve gia tri cua bieu thuc va in ra ca line,ham debug
    let rect3 = Rectangle {
        width: dbg!(2*20),
        height: 60,
    };
// !dbg se nam quyen so huu 1 bieu thuc,con println! se nam tham chieu.
// muon dbg! nam tham chieu thi nhu duoi day.
    dbg!(&rect3);

}

//debug de in ra ca struct
#[derive(Debug)]
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
