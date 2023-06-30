use nix::sys::mman::{mmap, MapFlags, ProtFlags};
use std::{error::Error, fs::OpenOptions, io::Write, os::raw::c_void, ptr, time};

const CACHE_LINE_SIZE: usize = 64;
const NACCESS: usize = 128 * 1024 * 1024;

fn main() -> Result<(), Box<dyn Error>> {
    let mut file = OpenOptions::new()
        .create(true)
        .read(true)
        .write(true)
        .open("out.txt")?;

    let mut i = 2.0;
    while i <= 16.0 {
        let bufsize: usize = 2_f64.powf(i) as usize * 1024;
        let data: *mut c_void = unsafe {
            mmap(
                ptr::null_mut(),
                bufsize,
                ProtFlags::PROT_READ | ProtFlags::PROT_WRITE,
                MapFlags::MAP_SHARED | MapFlags::MAP_ANON,
                -1,
                0,
            )?
        };

        println!(
            "バッファサイズ 2^{:.2}({}) KBについてのデータを収集中...",
            i,
            bufsize / 1024
        );
        let start = time::Instant::now();
        let data = data as *mut u8;
        for _ in 0..(NACCESS / (bufsize / CACHE_LINE_SIZE)) {
            for j in (0..bufsize).step_by(CACHE_LINE_SIZE) {
                unsafe { data.add(j).write(0) };
            }
        }
        let end = time::Instant::now().duration_since(start).as_nanos();
        writeln!(&mut file, "{}\t{}", i, (NACCESS as f64 / end as f64))?;
        i += 0.25;
    }
    Ok(())
}