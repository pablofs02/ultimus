//! Funciones para manejar y reconocer los elementos de una terminal.
use libc::{geteuid, ioctl, winsize, STDOUT_FILENO, TIOCGWINSZ};
use std::{io::Write, mem, os::fd::AsRawFd};

/// Mira si la salida estándar actual es una terminal.
/// Útil para dar texto con o sin colores.
pub fn es_output_de_terminal() -> bool {
    unsafe { libc::isatty(std::io::stdout().as_raw_fd()) != 0 }
}

/// Devuelve las (filas y columnas) de la terminal actual.
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

/// Crea un buffer vacío y esconde el cursor en la esquina superior izquierda.
pub fn panel_nuevo() {
    print!("\x1b[?1049h\x1b[0;0H\x1b[?25l");
    std::io::stdout().flush().unwrap();
}

/// Retorna al buffer principal.
pub fn panel_principal() {
    print!("\x1b[?25h\x1b[?1049l");
    std::io::stdout().flush().unwrap();
}

/// Usuario que ejecuta el comando.
pub fn usuario_id() -> u32 {
    unsafe { geteuid() }
}

/// Mira si ha sido ejecutado con privilegios.
pub fn hay_privilegios() -> bool {
    let euid = unsafe { geteuid() };
    if euid == 0 {
        true
    } else {
        false
    }
}
