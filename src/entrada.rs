use libc::{c_int, tcgetattr, tcsetattr, termios, ECHO, ICANON, TCSANOW};
use std::io;
use std::io::Read;
use std::os::unix::io::AsRawFd;

/// Permite la entrada directa a la aplicación.
pub fn habilitar() -> io::Result<()> {
    let stdin_fd = io::stdin().as_raw_fd();
    let mut termios = tomar_config(stdin_fd)?;
    termios.c_lflag &= !(ICANON | ECHO);
    aplicar_config(stdin_fd, &termios)?;
    Ok(())
}

/// Retorna la entrada a la consola.
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

/// Espera a recibir una entrada (tecla).
pub fn pedir() -> Tecla {
    let mut buffer = [0; 8];
    if let Ok(n) = std::io::stdin().read(&mut buffer) {
        if n != 0 {
            return match buffer {
                [9, 0, 0, 0, 0, 0, 0, 0] => Tecla::Tabulador,
                [10, 0, 0, 0, 0, 0, 0, 0] => Tecla::Enter,
                [27, 0, 0, 0, 0, 0, 0, 0] => Tecla::Escape,
                [27, 79, 80, 0, 0, 0, 0, 0] => Tecla::F(1),
                [27, 79, 81, 0, 0, 0, 0, 0] => Tecla::F(2),
                [27, 79, 82, 0, 0, 0, 0, 0] => Tecla::F(3),
                [27, 79, 83, 0, 0, 0, 0, 0] => Tecla::F(4),
                [27, 91, 52, 126, 0, 0, 0, 0] => Tecla::Fin,
                [27, 91, 53, 126, 0, 0, 0, 0] => Tecla::Retroceder,
                [27, 91, 54, 126, 0, 0, 0, 0] => Tecla::Avanzar,
                [27, 91, 65, 0, 0, 0, 0, 0] => Tecla::Arriba,
                [27, 91, 66, 0, 0, 0, 0, 0] => Tecla::Abajo,
                [27, 91, 67, 0, 0, 0, 0, 0] => Tecla::Derecha,
                [27, 91, 68, 0, 0, 0, 0, 0] => Tecla::Izquierda,
                [27, 91, 72, 0, 0, 0, 0, 0] => Tecla::Inicio,
                [27, 91, 80, 0, 0, 0, 0, 0] => Tecla::Suprimir,
                [127, 0, 0, 0, 0, 0, 0, 0] => Tecla::Borrar,
                _ => Tecla::Char(letra(buffer)),
            };
        }
    }
    Tecla::Nada
}

fn letra(buf: [u8; 8]) -> char {
    std::str::from_utf8(&buf).unwrap().chars().next().unwrap()
}

pub enum Tecla {
    Char(char),
    Escape,
    Enter,
    Control,
    Tabulador,
    F(u8),
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
    Nada,
}
//BackTab
//Insert
//Null
//CapsLock
//ScrollLock
//NumLock
//PrintScreen
//Pause
//Menu
//KeypadBegin
//Media(MediaKeyCode)
//Modifier(ModifierKeyCode)
