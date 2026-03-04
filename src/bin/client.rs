use std::{env, io, process};
use talk::{sig, sig::{SIGUSR1, SIGUSR2}, bits};
use std::thread;
use std::time::Duration;

const TIME: u32 = 150000;

fn main() {
    let (pid, msg) = build_args(env::args()).unwrap_or_else(|e| {
        eprintln!("Arguments Error: {e}");
        process::exit(1);
    });

    if let Err(e) = run(pid, msg) {
        eprintln!("Signal Error: {e}");
        process::exit(1);
    }
}

fn build_args(
    mut args: impl Iterator<Item = String>,
) -> Result<(i32, String), &'static str> {
    args.next();

    let Some(pid) = args.next() else {
        return Err("Expected process id");
    };
    let pid: i32 = match pid.parse() {
        Ok(id) => id,
        Err(_) => return Err("Process id is not a number"),
    };
    let Some(msg) = args.next() else {
        return Err("Expected a message");
    };
    Ok((pid, msg))
}

fn run(pid: i32, msg: String) -> Result<(), io::Error> {
    let bin = bits::strtobin(msg);

    for bit in bin.chars() {
        thread::sleep(Duration::new(0, TIME));
        match bit {
            '1' => sig::kill(pid, SIGUSR1)?,
            '0' => sig::kill(pid, SIGUSR2)?,
            _ => (),
        }
    }
    for _ in 0..8 {
        thread::sleep(Duration::new(0, TIME));
        sig::kill(pid, SIGUSR2)?;
    }
    Ok(())
}
