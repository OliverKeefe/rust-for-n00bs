use std::thread;

fn main() {
    let value = 20;
    let handle = thread::spawn(move || -> i32 {
        value * 2
    });

    let result = handle.join();

    match result {
        Ok(r) => println!("Thread finished successfully, Result: {}", r),
        Err(_) => println!("Thread did not finish successfully")
    }
}