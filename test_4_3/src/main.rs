fn main() {
    let mut s = String::from("hello world");
    let word = firstword(&s);
    s.clear();
    println!("word = {word}");

// The slice type la tham chieu den 1 phan cua string.
    let str = String::from("hello world");
    let slice = &str[0..2];
    let words = &str[..5];
    let words2 = &str[5]
    println!("slice = {slice}");
    println!("word = {words}");

    
}

fn firstword (s: &String) -> usize {
    let bytes = s.as_bytes();

    for(i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
