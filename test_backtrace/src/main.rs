use std::backtrace::Backtrace;

fn faulty_function() {
    println!("Inside faulty_function");
    // Gây ra một lỗi panic để kiểm tra backtrace
    panic!("Something went wrong!");
}

fn main() {
    println!("Program started");

    // Sử dụng closure để bắt lỗi panic và in backtrace
    std::panic::set_hook(Box::new(|info| {
        println!("Panic occurred: {:?}", info);
        let bt = Backtrace::capture();
        println!("Backtrace:\n{:?}", bt);
    }));

    // Gọi hàm có lỗi
    faulty_function();

    println!("Program ended");
}