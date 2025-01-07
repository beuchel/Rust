fn main() {
// The slice type
    let mut s = String::from("hello world");

    let word = firstword(&s);
    s.clear();
    println!("word = {word}");
    
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
