use nix::{
    sys::wait::waitpid,
    unistd::{
        fork,
        ForkResult,
        getppid,
        getpid, 
        execve
    }
};
use std::ffi::CString;

fn main() {
    match unsafe { fork() } {
        // 親プロセス
        Ok(ForkResult::Parent { child }) => {
            println!("親プロセス: pid={}, 子プロセス: pid={}", getpid(), child);
            waitpid(child, None).unwrap();
        }
        // 子プロセス
        Ok(ForkResult::Child) => {
            let cmd = CString::new("/bin/echo").expect("CString::new failed");
            let args = [
                CString::new("echo").expect("CString::new failed"),
                CString::new(format!("pid={} からこんにちは", getpid()))
                    .expect("CString::new failed"),
            ];
            let env = CString::new("").expect("CString::new failed");
            println!(
                "子プロセス: pid={}, 親プロセス: pid={}",
                getpid(),
                getppid()
            );
            // 自プロセスを新しいプログラムで上書きする。
            // 子プロセス内で処理を実行するので、子プロセス内でexecve(2)を呼び出している
            execve(&cmd, &args, &[env]).expect("execve failed");
        }
        Err(_) => unsafe { libc::_exit(1) },
    }
}