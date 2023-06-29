use std::io::{self, Write};
use std::process;

fn main() {
    let size = 200_000_000;

    println!("メモリ獲得前");
    let output = process::Command::new("free").output().expect("free failed");
    io::stdout().write_all(&output.stdout).unwrap();

    // メモリ確保
    let _array = vec![1_000; size];

    println!("メモリ獲得後");
    let output = process::Command::new("free").output().expect("free failed");
    io::stdout().write_all(&output.stdout).unwrap();
}
