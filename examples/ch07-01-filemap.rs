use std::{
    error::Error,
    io::{self, Write},
    os::raw::c_void,
    process::Command,
    ptr,
};

use nix::{
    self,
    fcntl::{open, OFlag},
    sys::{
        mman::{mmap, MapFlags, ProtFlags},
        stat::Mode,
    },
    unistd,
};

fn main() -> Result<(), Box<dyn Error>> {
    let pid = unistd::getpid();
    println!("*** testfileのメモリマップ前のプロセスの仮想アドレス空間");
    let output = Command::new("cat")
        .arg(format!("/proc/{}/maps", pid))
        .output()?;
    io::stdout().write_all(&output.stdout)?;

    // File::open()を使うとmmap呼び出しがEACCESでエラー
    // let file = File::open("./testfile")?;
    // let fd = file.as_raw_fd();
    let fd = open("testfile", OFlag::O_RDWR, Mode::empty())?;
    let data: *mut c_void = unsafe {
        mmap(
            ptr::null_mut(),
            5,
            ProtFlags::PROT_READ | ProtFlags::PROT_WRITE,
            MapFlags::MAP_SHARED,
            fd,
            0,
        )?
    };
    println!("\ntestfileをマップしたアドレス:{:p}\n", data);

    println!("*** testfileのメモリマップ前のプロセスの仮想アドレス空間");
    let output = Command::new("cat")
        .arg(format!("/proc/{}/maps", pid))
        .output()?;
    io::stdout().write_all(&output.stdout)?;
    for (i, b) in b"HELLO".iter().enumerate() {
        unsafe {
            (data.add(i) as *mut u8).write(*b);
        }
    }
    Ok(())
}