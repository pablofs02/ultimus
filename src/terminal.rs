use libc::{ioctl, winsize, STDOUT_FILENO, TIOCGWINSZ, geteuid};
use std::{mem, os::fd::AsRawFd};

pub fn es_output_de_terminal() -> bool {
    unsafe { libc::isatty(std::io::stdout().as_raw_fd()) != 0 }
}

pub fn dimensiones() -> Option<(u16, u16)> {
    unsafe {
        let mut dimensiones: winsize = mem::zeroed();
        if ioctl(STDOUT_FILENO, TIOCGWINSZ, &mut dimensiones) == 0 {
            Some((dimensiones.ws_row, dimensiones.ws_col))
        } else {
            None
        }
    }
}

pub fn panel_nuevo() {
    print!("\x1b[?1049h\x1b[0;0H\x1b[?25l");
}

pub fn panel_principal() {
    print!("\x1b[?25h\x1b[?1049l");
}

pub fn hay_privilegios() -> bool {
    let euid = unsafe { geteuid() };
    if euid == 0 {
        true
    } else {
        false
    }
}

