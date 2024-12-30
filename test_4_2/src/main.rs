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
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn not_change(some_string: &String) {
    some_string.push_str(",world");
}

fn mutable_change(some_string: &mut String) {
    some_string.push_str(",world");
}