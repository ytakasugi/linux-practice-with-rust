use nix::{
    libc::{_exit, size_t},
    sys::{
        mman::{mmap, MapFlags, ProtFlags},
        wait::wait,
    },
    unistd::{fork, ForkResult},
};
use std::ffi::c_void;
use std::i64;
use std::ptr;

fn main() {
    const PAGE_SIZE: size_t = 4096;
    let mut data: i64 = 1000;
    let shared_memory: *mut c_void = unsafe {
        mmap(
            ptr::null_mut(),
            PAGE_SIZE,
            ProtFlags::PROT_READ | ProtFlags::PROT_WRITE,
            MapFlags::MAP_SHARED | MapFlags::MAP_ANON,
            -1,
            0,
        )
        .expect("mmap failed")
    };
    println!("子プロセス生成前のデータの値：{}", data);
    for (i, byte) in data.to_le_bytes().into_iter().enumerate() {
        unsafe {
            let addr = shared_memory.add(i) as *mut u8;
            addr.write(byte);
        }
    }

    match unsafe { fork() } {
        Ok(ForkResult::Parent { .. }) => {
            wait().expect("wait failed");
            // 子プロセスと同じメモリアドレス
            let addr = shared_memory as *mut [u8; 8];
            dbg!(addr);
            data = unsafe { i64::from_le_bytes(*addr) };
        }
        Ok(ForkResult::Child) => {
            // 親プロセスと同じメモリアドレス
            let addr = shared_memory as *mut [u8; 8];
            dbg!(addr);
            data = unsafe { i64::from_le_bytes(*addr) };
            data *= 2;
            for (i, byte) in data.to_le_bytes().into_iter().enumerate() {
                unsafe {
                    let addr: *mut u8 = shared_memory.add(i) as *mut u8;
                    addr.write(byte);
                }
            }
            unsafe { _exit(0) };
        }
        Err(_) => {
            unsafe { _exit(1) };
        }
    }
    println!("子プロセス終了後のデータの値：{}", data);
}
