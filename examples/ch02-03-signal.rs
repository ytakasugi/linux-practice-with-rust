use std::io::Error;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

use signal_hook::consts::signal::*;

fn main() -> Result<(), Error> {
    const SIGINT_U: usize = SIGINT as usize;
    let term = Arc::new(AtomicUsize::new(0));
    signal_hook::flag::register_usize(SIGINT, Arc::clone(&term), SIGINT_U)?;
    loop {
        match term.load(Ordering::Relaxed) {
            0 => (),
            SIGINT_U => (),
            _ => unreachable!(),
        }
    }
}
