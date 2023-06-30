use std::{
    fs::{File, OpenOptions},
    io::{Read, Write},
    path::Path,
};

fn main() {
    let path = Path::new("count");
    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(e) => {
            println!("open failed: {}", e);
            return;
        }
    };
    let mut content = String::new();
    let mut count: i64 = match file.read_to_string(&mut content) {
        Ok(_) => content.parse().expect("parse failed"),
        Err(e) => {
            println!("parse failed: {}", e);
            return;
        }
    };
    count += 1;
    match OpenOptions::new().write(true).open(path) {
        Ok(mut file) => {
            file.write_all(count.to_string().as_bytes())
                .expect("write failed");
        }
        Err(e) => {
            println!("{}", e);
            #[allow(clippy::needless_return)]
            return;
        }
    }
}