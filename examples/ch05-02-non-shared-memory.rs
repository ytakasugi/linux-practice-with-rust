use nix::{
    libc::_exit,
    sys::wait::wait,
    unistd::{fork, ForkResult},
};

fn main() {
    let mut data = 1000;
    println!("子プロセス生成前のデータの値：{}", data);
    match unsafe { fork() } {
        Ok(ForkResult::Parent { .. }) => {
            wait().expect("wait failed");
        }
        Ok(ForkResult::Child) => {
            data *= 2;
            dbg!(data);
            unsafe {
                _exit(0);
            };
        }
        Err(e) => println!("{}", e),
    }
    println!("子プロセス終了後のデータの値：{}", data);
}
