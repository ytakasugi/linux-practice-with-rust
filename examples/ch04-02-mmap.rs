use nix::libc::size_t;
use nix::sys::mman::mmap;
use nix::sys::mman::MapFlags;
use nix::sys::mman::ProtFlags;
use nix::unistd::getpid;
use std::io::{self, Write};
use std::process;
use std::ptr;

fn main() {
    const ALLOC_SIZE: size_t = 1024 * 1024 * 1024;
    let pid = getpid();

    //dbg!(pid);

    println!("新規メモリ領域獲得前のメモリマップ");

    let command = process::Command::new("cat")
        .arg(format!("/proc/{}/maps", pid))
        .output()
        .expect("cat failed");
    io::stdout()
        .write_all(&command.stdout)
        .expect("write stdout failed");

    // mmap()の使い方：https://kazmax.zpp.jp/cmd/m/mmap.2.html
    let data = unsafe {
        mmap(
            // 大した意味なし、0でいい
            ptr::null_mut(),
            // メモリにマップするサイズ
            ALLOC_SIZE,
            // メモリ保護の指定
            ProtFlags::PROT_READ | ProtFlags::PROT_WRITE,
            // マップされたオブジェクトのタイプ、マップ時のオプション、
            // マップされたページコピーへの変更を そのプロセスだけが行えるのかを指定する
            MapFlags::MAP_ANON | MapFlags::MAP_PRIVATE,
            // MAP_ANONがセットされた場合は-1にするとだけまず覚えておく
            -1,
            // ページサイズの整数倍であること
            0,
        )
        .expect("mmap failed")
    };

    println!(
        "新規メモリ領域： アドレス = {:p}, サイズ = 0x{:x}",
        data, ALLOC_SIZE
    );

    println!("新規メモリ領域獲得後のメモリマップ");
    let command = process::Command::new("cat")
        .arg(format!("/proc/{}/maps", pid))
        .output()
        .expect("cat failed");
    io::stdout()
        .write_all(&command.stdout)
        .expect("write stdout failed");
}
