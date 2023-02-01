use ferris_says::say;
use std::io::{stdout, BufWriter};

fn fibo(n : i32) -> i32 {
    if n < 2 {
        return 1;
    }
    else {
        return fibo(n-1) + fibo(n-2);
    }
}

fn main() {
    let stdout = stdout();
    let message = String::from("Hello fellow Rustaceans!");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();

    println!("Hello, world!");
    for i in 0..20 {
        println!("Fibo of {} is {}", i, fibo(i));
    }
}
