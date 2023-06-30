use std::path::Path;

use nix::fcntl::{flock, open, FlockArg, OFlag};
use nix::sys::stat::Mode;
use nix::unistd::{read, write};

fn main() -> Result<(), nix::Error> {
    let path = Path::new("count");
    let fd = open(path, OFlag::O_RDWR, Mode::empty())?;
    flock(fd, FlockArg::LockExclusive)?;

    let mut buf = [0u8; 8];
    read(fd, &mut buf)?;
    let content = buf
        .iter()
        .filter(|b| **b != 0)
        .map(|b| *b as char)
        .collect::<String>();
    let mut count: i64 = content
        .parse()
        .unwrap_or_else(|_| panic!("parse error: {}", content));
    count += 1;
    let content = count.to_string().into_bytes();

    let fd = open(path, OFlag::O_WRONLY | OFlag::O_TRUNC, Mode::empty())?;
    write(fd, &content)?;
    flock(fd, FlockArg::Unlock)?;
    Ok(())
}