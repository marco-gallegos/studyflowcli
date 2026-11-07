//! Lectura y escritura del archivo local de datos.
use std::fs::File;
use std::io::{self, Write, Read};

use crate::config::DEFAULT_TODO_FILE;

pub fn file_exists(file_name: &str) -> bool {
    if File::open(file_name).is_ok() {
        return true;
    }
    return false;
}

pub fn create_new_todo(file_name: &str) -> bool {
    if file_exists(file_name) {
        println!("El archivo '{}' ya existe. No se puede crear un nuevo proyecto.", file_name);
        return false;
    }
    let mut file = File::create(file_name).unwrap();
    writeln!(file, "{{}}").unwrap();
    file.flush().unwrap();
    return true;
}

pub fn read_todo_file(file_name: &str) -> io::Result<String> {
    if !file_exists(file_name) {
        println!("El archivo '{}' no existe. Por favor, inicializa un proyecto primero.", file_name);
        return Err(io::Error::new(io::ErrorKind::NotFound, "Archivo no encontrado"));
    }
    let mut file = File::open(file_name)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}


pub fn list_todos() {
    match read_todo_file(DEFAULT_TODO_FILE) {
        Ok(contents) => {
            println!("Contenido del archivo '{}':", DEFAULT_TODO_FILE);
            println!("{}", contents);
        }
        Err(e) => {
            println!("Error al leer el archivo '{}': {}", DEFAULT_TODO_FILE, e);
        }
    }
}

