fn main() {
    //borrow
    let s1 = String::from("hello");

    let len = calculate_length(&s1);
    println!("The length of '{s1}' is {len}");
    // s la reference nen sau khi thoai khoi pham vi ham thi s bi huy,
    //nhung s1 khong bi huy do ko chuyen onership sang s
    
    //not_change(s1);
    // error do reference khong duoc phep thay doi.

    let mut s2 = String::from("hello");
    mutable_change(&mut s2);
    println!("s2= {s2}");

    let mut s3 = String::from("hello");
    let r2 = &mut s3;
    let r3 = &mut s3; // chua error vi tham chieu chua duoc su dung
    // println!("r2 = {r2}"); 
    // println!("r3 = {r3}");
    // Error co 2 tham chieu co the thay doi gia tri(&mut)r2,r3 den s3 ton tai 
    // dong thoi dan den data race

    let mut s4 = String::from("hello");
    let r4 = &mut s4;
    println!("r4 = {r4}");
    let r5 = &mut s4;
    println!("r5 = {r5}");
    // Khong bi error vi pham vi su dung cua r4 da ket thuc sau println.


    let mut s5 = String::from("hello");
    {
        let r6 = &mut s5;
    } // r6 goes out of scope here, so we can make a new reference with no problems.
    let r7 = &mut s5;
    println!("r7 = {r7}");


    let mut s6 = String::from("hello");
    let r8 = &s6; //no problem
    let r9 = &s6; // no problem
    // let r10 = &mut s6; // BIG PROBLEM vi khong the tham chieu mut trong khi
    // dang tham chieu notmut toi 1 gia tri
    // println("r10 = {r10}");

    let mut s7 = String::from("hello");
    let r11 = &s7; // no problem.
    let r12 = &s7; // no problem.
    println!(" {r11} and {r12}");
    let r13 = &mut s7; // no problem do pham vi cua r11 va r12 da ket thuc o line 50
    println!("r13 = {r13}");
   
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

// fn not_change(some_string: &String) {
//     some_string.push_str(",world");

// }

fn mutable_change(some_string: &mut String) {
    some_string.push_str(",world");
}