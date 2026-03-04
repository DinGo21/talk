use talk::{bits, sig::{self, SIGUSR1, SIGUSR2, SigHandler}};
use std::process;

static mut SIG: i32 = 0;

fn main() {
    if let Err(e) = sig::signal(SIGUSR1, SigHandler::SigHnd(sig_handler)) {
        eprintln!("Signal error: {e}");
        process::exit(1);
    }
    if let Err(e) = sig::signal(SIGUSR2, SigHandler::SigHnd(sig_handler)) {
        eprintln!("Signal error: {e}");
        process::exit(1);
    }
    println!("Process id: {}", process::id());
    run();
}

extern "C" fn sig_handler(signum: i32) {
    unsafe {
        SIG = signum;
    }
}

fn run() {
    loop {
        let mut bin = String::new();
        for _ in 0..8 {
            let sig;
            let _ = sig::pause();
            unsafe {sig = SIG}
            bin.push(sig::sigtobit(sig));
        }
        match bits::bintochar(bin) {
            Some(c) => print!("{c}"),
            None => println!(),
        }
    }
}
