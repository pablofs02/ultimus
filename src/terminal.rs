use libc::{ioctl, winsize, STDOUT_FILENO, TIOCGWINSZ};
use std::mem;

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
