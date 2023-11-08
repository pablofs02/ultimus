use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn nombre_usuario_por_id(id: u32) -> Result<String, String> {
    let arc_usuarios = File::open("/etc/passwd").unwrap();
    for l in BufReader::new(arc_usuarios).lines() {
        if let Ok(l) = l {
            let partes: Vec<&str> = l.split(':').collect();
            if partes.len() >= 3 {
                if let Ok(uid) = partes[2].parse::<u32>() {
                    if uid == id {
                        return Ok(partes[0].to_string());
                    }
                }
            }
        }
    }
    Err(format!("No hay un usuario con id: '{id}'"))
}
