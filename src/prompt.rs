//! Un prompt para crear aplicaciones de línea de consola personalizadas.
use crate::entrada::{self, Tecla};
use std::io::{stdout, Write};

pub struct Prompt {
    entradas: Vec<String>,
    prompt: String,
}

impl Prompt {
    pub fn iniciar(prompt: &str) -> Self {
        entrada::habilitar().unwrap();
        Self {
            entradas: vec![],
            prompt: prompt.to_owned(),
        }
    }

    pub fn preguntar(&mut self) -> String {
        let mut entrada = String::new();
        let mut actual = String::new();
        let mut n_entrada = 0;
        let mut pos = 0;
        print!("{}", self.prompt);
        stdout().flush().unwrap();
        loop {
            match entrada::pedir() {
                Tecla::Enter => {
                    println!();
                    break;
                }
                Tecla::Borrar => {
                    if pos != entrada.chars().count() {
                        entrada.remove(Self::pos_de_byte(&entrada, pos));
                        print!("\x1b[1D\x1b[1P");
                    }
                }
                Tecla::Suprimir => {
                    if pos != 0 {
                        pos -= 1;
                        entrada.remove(Self::pos_de_byte(&entrada, pos));
                        print!("\x1b[1P");
                    }
                }
                Tecla::Arriba => {
                    if n_entrada != self.entradas.len() {
                        n_entrada += 1;
                        if let Some(x) = self.entradas.get(self.entradas.len() - n_entrada) {
                            if n_entrada == 1 {
                                actual = entrada;
                            }
                            entrada = x.clone();
                            print!("\x1b[2K\r{}{entrada}", self.prompt);
                        }
                    }
                }
                Tecla::Abajo => {
                    if n_entrada == 1 {
                        n_entrada = 0;
                        entrada = actual.clone();
                        print!("\x1b[2K\r{}{entrada}", self.prompt);
                    } else if n_entrada != 0 {
                        n_entrada -= 1;
                        if let Some(x) = self.entradas.get(self.entradas.len() - n_entrada) {
                            entrada = x.clone();
                            print!("\x1b[2K\r{}{entrada}", self.prompt);
                        }
                    }
                }
                Tecla::Izquierda => {
                    if pos < entrada.chars().count() {
                        print!("\x1b[1D");
                        pos += 1;
                    }
                }
                Tecla::Derecha => {
                    if pos != 0 {
                        print!("\x1b[1C");
                        pos -= 1;
                    }
                }
                Tecla::Char(c) => {
                    if pos == 0 {
                        entrada.push(c);
                        print!("{c}");
                    } else {
                        let p = Self::pos_de_byte_sig(&entrada, pos - 1);
                        let x = &entrada[p..];
                        print!("\x1b[K{c}{x}{}", "\x1b[1D".repeat(x.chars().count()));
                        entrada.insert(p, c);
                    }
                }
                _ => (),
            }
            stdout().flush().unwrap();
        }
        if !entrada.is_empty() {
            self.entradas.push(entrada.clone());
        }
        entrada
    }

    fn pos_de_byte(entrada: &str, pos: usize) -> usize {
        if pos == 0 {
            return entrada.chars().count() - 1;
        }
        let pos = entrada.chars().count() - (1 + pos);
        let mut pb = 0;
        let mut pc = 0;
        for b in entrada.as_bytes() {
            if b & 0b1100_0000 != 0b1000_0000 {
                pc += 1;
            }
            pb += 1;
            if pc == pos {
                break;
            }
        }
        pb - 1
    }

    fn pos_de_byte_sig(entrada: &str, pos: usize) -> usize {
        if pos == 0 {
            return entrada.chars().count() - 1;
        }
        let pos = entrada.chars().count() - (1 + pos);
        let mut pb = 0;
        let mut pc = 0;
        for b in entrada.as_bytes() {
            if b & 0b1100_0000 != 0b1000_0000 {
                pc += 1;
            }
            pb += 1;
            if pc == pos + 1 {
                break;
            }
        }
        pb - 1
    }
}

impl Drop for Prompt {
    fn drop(&mut self) {
        entrada::deshabilitar().unwrap();
    }
}
