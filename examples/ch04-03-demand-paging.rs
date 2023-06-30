use nix::libc::size_t;
use nix::sys::mman::{mmap, MapFlags, ProtFlags};
use std::io::{self, Read};
use std::os::raw::c_int;
use std::ptr;

fn main() {
    const ALLOC_SIZE: size_t = 100 * 1024 * 1024;
    const ACCESS_UNIT: size_t = 10 * 1024 * 1024;
    const PAGE_SIZE: size_t = 4096;

    println!("新規メモリ領域獲得前。Enterキーを押すと100MiBの新規メモリ領域を獲得します:");

    let _ = io::stdin().lines();

    let mut memregion = unsafe {
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

    println!("新規メモリ領域を獲得しました。Enterキーを押すと1秒に10MiBづつ、合計100MiBの新規メモリ領域にアクセスします:");

    let _ = io::stdin().read(&mut [0u8]).expect("waiting enter failed");

    for i in (0..ALLOC_SIZE).step_by(PAGE_SIZE) {
        unsafe {
            let target_address = memregion as *mut c_int;
            *(target_address) = 0;
            memregion = memregion.add(PAGE_SIZE);
        }

        if i % ACCESS_UNIT == 0 && i != 0 {
            println!(
                "{}: {} MiBアクセスしました",
                chrono::Utc::now().format("%H:%M:%S"),
                i / 1024 / 1024
            );
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    }

    println!(
        "{}: 新規獲得したメモリ領域のすべてにアクセスしました。Enterキーを押すと終了します:",
        chrono::Utc::now().format("%H:%M:%S")
    );
    let _ = io::stdin().read(&mut [0u8]).expect("waiting enter failed");
}
