use std::io;
use libc::{c_void, sighandler_t, SIG_DFL, SIG_IGN, SIG_ERR};
pub use libc::{SIGUSR1, SIGUSR2};

pub enum SigHandler {
    SigDfl,
    SigIgn,
    SigHnd(extern "C" fn(i32)),
}

pub fn signal(signum: i32, handler: SigHandler) -> Result<(), io::Error> {
    let e; 

    unsafe {
        e = match handler {
            SigHandler::SigDfl => libc::signal(signum, SIG_DFL),
            SigHandler::SigIgn => libc::signal(signum, SIG_IGN),
            SigHandler::SigHnd(h) => libc::signal(signum, h as *const () as sighandler_t),
        }
    }
    if e == SIG_ERR {
        return Err(io::Error::last_os_error());
    }
    Ok(())
}

pub fn kill(pid: i32, signum: i32) -> Result<(), io::Error> {
    let e;

    unsafe {
        e = libc::kill(pid, signum);
    }
    if e < 0 {
        return Err(io::Error::last_os_error());
    }
    Ok(())
}

pub fn pause() -> Result<(), io::Error> {
    let e;

    unsafe {
        e = libc::pause();
    }
    if e < 0 {
        return Err(io::Error::last_os_error());
    }
    Ok(())
}

pub fn putchar(c: char) {
    unsafe {
        libc::write(0, &raw const c as *const c_void, 1);
    }
}

pub fn sigtobit(sig: i32) -> char {
    if sig == SIGUSR1 {
        return '1';
    }
    '0'
}
