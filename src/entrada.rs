//! Funciones para manejar y reconocer la entrada directa de la terminal.
//! Esté módulo se usa siguiendo este esquema:
//! 1º habilitar:    activar la entrada directa.
//! 2º pedir:        reconocer la 'Tecla' pulsada en la terminal.
//! 3º deshabilitar: desactivar la entrada directa cuando ya no se requiera más.
use libc::{c_int, tcgetattr, tcsetattr, termios, ECHO, ICANON, TCSANOW};
use std::io::{self, Read};
use std::os::unix::io::AsRawFd;

/// Permite la entrada directa a la aplicación.
pub fn habilitar() -> io::Result<()> {
    let stdin_fd = io::stdin().as_raw_fd();
    let mut termios = tomar_config(stdin_fd)?;
    termios.c_lflag &= !(ICANON | ECHO);
    aplicar_config(stdin_fd, &termios)?;
    Ok(())
}

/// Retorna la entrada directa a la consola.
pub fn deshabilitar() -> io::Result<()> {
    let stdin_fd = io::stdin().as_raw_fd();
    let mut termios = tomar_config(stdin_fd)?;
    termios.c_lflag |= ICANON | ECHO;
    aplicar_config(stdin_fd, &termios)?;
    Ok(())
}

fn tomar_config(fd: c_int) -> io::Result<termios> {
    let mut termios = termios {
        c_iflag: 0,
        c_oflag: 0,
        c_cflag: 0,
        c_lflag: 0,
        c_line: 0,
        c_cc: [0; 32],
        c_ispeed: 0,
        c_ospeed: 0,
    };
    if unsafe { tcgetattr(fd, &mut termios as *mut _) } == -1 {
        return Err(io::Error::last_os_error());
    }
    Ok(termios)
}

fn aplicar_config(fd: c_int, termios: &termios) -> io::Result<()> {
    if unsafe { tcsetattr(fd, TCSANOW, termios as *const _) } == -1 {
        return Err(io::Error::last_os_error());
    }
    Ok(())
}

/// Espera hasta recibir una entrada directa (tecla).
pub fn pedir() -> Tecla {
    let mut buffer = [0; 8];
    if let Ok(n) = std::io::stdin().read(&mut buffer) {
        if n != 0 {
            println!("{buffer:?}");
            return match buffer {
                [9, 0, 0, 0, 0, 0, 0, 0] => Tecla::Tab,
                [10, 0, 0, 0, 0, 0, 0, 0] => Tecla::Intro,
                [27, 0, 0, 0, 0, 0, 0, 0] => Tecla::Escape,
                [27, 79, 80, 0, 0, 0, 0, 0] => Tecla::F(1),
                [27, 79, 81, 0, 0, 0, 0, 0] => Tecla::F(2),
                [27, 79, 82, 0, 0, 0, 0, 0] => Tecla::F(3),
                [27, 79, 83, 0, 0, 0, 0, 0] => Tecla::F(4),
                [27, 91, 49, 53, 126, 0, 0, 0] => Tecla::F(5),
                [27, 91, 49, 55, 126, 0, 0, 0] => Tecla::F(6),
                [27, 91, 49, 56, 126, 0, 0, 0] => Tecla::F(7),
                [27, 91, 49, 57, 126, 0, 0, 0] => Tecla::F(8),
                [27, 91, 50, 48, 126, 0, 0, 0] => Tecla::F(9),
                [27, 91, 50, 49, 126, 0, 0, 0] => Tecla::F(10),
                [27, 91, 50, 51, 126, 0, 0, 0] => Tecla::F(11),
                [27, 91, 50, 52, 126, 0, 0, 0] => Tecla::F(12),
                [27, 91, 49, 59, 50, 80, 0, 0] => Tecla::FS(1),
                [27, 91, 49, 59, 50, 81, 0, 0] => Tecla::FS(2),
                [27, 91, 49, 59, 50, 82, 0, 0] => Tecla::FS(3),
                [27, 91, 49, 59, 50, 83, 0, 0] => Tecla::FS(4),
                [27, 91, 49, 53, 59, 50, 126, 0] => Tecla::FS(5),
                [27, 91, 49, 55, 59, 50, 126, 0] => Tecla::FS(6),
                [27, 91, 49, 56, 59, 50, 126, 0] => Tecla::FS(7),
                [27, 91, 49, 57, 59, 50, 126, 0] => Tecla::FS(8),
                [27, 91, 50, 48, 59, 50, 126, 0] => Tecla::FS(9),
                [27, 91, 50, 49, 59, 50, 126, 0] => Tecla::FS(10),
                [27, 91, 50, 51, 59, 50, 126, 0] => Tecla::FS(11),
                [27, 91, 50, 52, 59, 50, 126, 0] => Tecla::FS(12),
                [27, 91, 52, 126, 0, 0, 0, 0] => Tecla::Fin,
                [27, 91, 53, 126, 0, 0, 0, 0] => Tecla::Retroceder,
                [27, 91, 54, 126, 0, 0, 0, 0] => Tecla::Avanzar,
                [27, 91, 65, 0, 0, 0, 0, 0] => Tecla::Arriba,
                [27, 91, 66, 0, 0, 0, 0, 0] => Tecla::Abajo,
                [27, 91, 67, 0, 0, 0, 0, 0] => Tecla::Derecha,
                [27, 91, 68, 0, 0, 0, 0, 0] => Tecla::Izquierda,
                [27, 91, 72, 0, 0, 0, 0, 0] => Tecla::Inicio,
                [27, 91, 80, 0, 0, 0, 0, 0] => Tecla::Suprimir,
                [27, 91, 90, 0, 0, 0, 0, 0] => Tecla::TabInv,
                [127, 0, 0, 0, 0, 0, 0, 0] => Tecla::Borrar,
                _ => Tecla::Char(letra(buffer)),
            };
        }
    }
    Tecla::Char('\0')
}

fn letra(buf: [u8; 8]) -> char {
    std::str::from_utf8(&buf).unwrap().chars().next().unwrap()
}

pub enum Tecla {
    Char(char),
    Escape,
    Intro,
    Tab,
    TabInv,
    Suprimir,
    Borrar,
    Arriba,
    Abajo,
    Derecha,
    Izquierda,
    Inicio,
    Fin,
    Avanzar,
    Retroceder,
    F(u8),
    FS(u8),
}
