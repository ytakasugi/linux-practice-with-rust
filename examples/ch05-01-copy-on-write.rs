use nix::libc::{_exit, size_t};
use nix::sys::{
    mman::{mmap, MapFlags, ProtFlags},
    wait::waitpid,
};
use nix::unistd::{fork, getpid, ForkResult};
use std::ffi::c_void;
use std::io::Write;
use std::os::raw::c_int;
use std::ptr;

const ALLOC_SIZE: size_t = 100 * 1024 * 1024;
const PAGE_SIZE: size_t = 4096;

fn access(mut data: *mut c_void) {
    for _ in (0..ALLOC_SIZE).step_by(PAGE_SIZE) {
        unsafe {
            let target_address = data as *mut c_int;
            *(target_address) = 0;
            data = data.add(PAGE_SIZE);
        }
    }
}

fn show_meminfo(msg: &str, process: &str) {
    println!("{}", msg);
    println!("freeコマンドの実行結果:");
    let commmand = std::process::Command::new("free")
        .output()
        .expect("free failed");

    std::io::stdout()
        .write_all(&commmand.stdout)
        .expect("write failed");

    println!("{}のメモリ関連情報：", process);

    let command = std::process::Command::new("ps")
        .arg("-orss,maj_flt,min_flt")
        .arg(getpid().to_string())
        .output()
        .expect("ps failed");

    std::io::stdout()
        .write_all(&command.stdout)
        .expect("write failed");
}

fn main() {
    // メモリ領域確保
    let data = unsafe {
        mmap(
            ptr::null_mut(),
            ALLOC_SIZE,
            ProtFlags::PROT_READ | ProtFlags::PROT_WRITE,
            MapFlags::MAP_ANON | MapFlags::MAP_PRIVATE,
            -1,
            0,
        )
        .expect("mmap failed")
    };

    // メモリアクセス
    access(data);
    show_meminfo("*** 子プロセス生成前 ***", "親プロセス");

    // 子プロセス作成
    match unsafe { fork() } {
        Ok(ForkResult::Child) => {
            show_meminfo("*** 子プロセス生成直後 ***", "子プロセス");
            // 子プロセス内でメモリアクセス
            access(data);
            show_meminfo("*** 子プロセスによるメモリアクセス後 ***", "子プロセス");
            unsafe { _exit(0) };
        }
        Ok(ForkResult::Parent { child, .. }) => {
            waitpid(child, None).expect("waitpid failed");
        }
        Err(_) => unsafe { _exit(1) },
    }
}
