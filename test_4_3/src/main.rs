fn main() {
    // cach 1 de lay first word
    let mut s = String::from("hello world");
    let word = firstword(&s);
    s.clear();
    // khi nay ma su dung chi muc word de trich xuat first word trog s thi loi ve mat logic,ko gay loi bien dich
    // vi s va firstword() khong co lien he voi nhau. khi firstword ket thuc thi word
    // nhan ve gia tri 5,va tham chieu trong ham firstword cung da ket thuc
    println!("word = {word}");

// The slice type la tham chieu den 1 phan cua string.
    let str = String::from("hello world");
    let slice = &str[0..2];
    let words = &str[1..4];
    let words2 = &str[5..];
    let len = str.len();
    let words3 = &str[5..len]; // giong word2.
    let words4 = &str[..]; // lay toan bo chuoi.

    println!("slice = {slice}");
    println!("word = {words}; words2 = {words2}; words3 = {words3};words4 = {words4}");
    println!("len = {len}");
// cach 2 lay first word su dung slice.
    let mut s2 = String::from("hello world");
    let words1 = firstword2(&s2);
    // s2.clear(); // error loi mutable borrow vi khi goi firstword2() thi tao ra mot 
    // tham chieu bat bien den s2 va cung tra ve 1 tham chieu. Va string words1 
    // can string goc la s2 ton tai,va ko thay doi cho den khi viec su dung words1 
    //ket thuc. Vay nen goi ham s2.clear() se thay doi s2,nen co the lam data races.
    // Vay nen loi biet dich
    println!("words1 = {words1}");
    s2.clear();  // o day ko loi vi words1 da su dung xong.


// truyen lice string vao ham.
    let mut s3 = String::from("hello world");
    let s4 = "helloworld";
    let word = firstword3(&s3[0..7]);
    println!("word = {word}");
// string literal
    let word = firstword3(&s4[2..4]);
    println!("word = {word}");

// hop le vi s4 la literal string = slices
    let word = firstword3(s4);
    println!("word = {word}");

// Slice co the dung voi mang
    let arr = [5, 6, 7, 8];
    let slice = &arr[1..3];
    println!(" slice[1] = {}",slice[0]);

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

fn firstword2 (s: &String) -> &str {
    let bytes = s.as_bytes();

    for(i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
} // &String la tham chieu den 1 owned string, con &str la tham chieu toi moi string slice
 // String::from("abc") la kieu String,duoc luu tren heap
 // &str la slices cung chinh la string literals (chuoi ki tu) duoc luu o vung nho 
 // read only, chinh la "hello"

fn firstword3 (s: &str) ->&str {
    let bytes = s.as_bytes();
    
    for(i,&item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    
    &s[..]
}