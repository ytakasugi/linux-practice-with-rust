use nix::{
    sys::wait::waitpid,
    unistd::{
        fork,
        ForkResult,
        getppid,
        getpid
    }
};

fn main() {
    // fork(2)をコールし、プロセスを複製させる
    match unsafe { fork() } {
        // 親プロセス
        Ok(ForkResult::Parent { child }) => {
            println!("親プロセス: pid={}, 子プロセスのpid={}", getpid(), child);
            // 子プロセスが完了するまで待つために、wait(2)をコールする
            waitpid(child, None).unwrap();
        }
        // 子プロセス
        Ok(ForkResult::Child) => {
            println!("子プロセス: pid={}, 親プロセスのpid={}", getpid(), getppid());
            unsafe { libc::_exit(0) };
        }
        Err(_) => unsafe { libc::_exit(1) }
    } 
}