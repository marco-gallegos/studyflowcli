//! Lectura y escritura del archivo local de datos.
use std::fs::File;
use std::io::{self, Write, Read};

use crate::config::DEFAULT_TODO_FILE;
use crate::errors::AppError;
use crate::models::{Tarea};

pub fn file_exists(file_name: &str) -> bool {
    if File::open(file_name).is_ok() {
        return true;
    }
    return false;
}

pub fn create_new_todo_file(file_name: &str) -> bool {
    if file_exists(file_name) {
        println!("El archivo '{}' ya existe. No se puede crear un nuevo proyecto.", file_name);
        return false;
    }
    let mut file = File::create(file_name).unwrap();
    writeln!(file, "[]").unwrap();
    file.flush().unwrap();
    return true;
}

pub fn read_todo_file(file_name: &str) -> io::Result<String> {
    if !file_exists(file_name) {
        return Err(io::Error::new(io::ErrorKind::NotFound, "Archivo no encontrado"));
    }
    let mut file = File::open(file_name)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}


pub fn list_get_todo_content() ->  Result<String, AppError> {
    match read_todo_file(DEFAULT_TODO_FILE) {
        Ok(content) => Ok(content),
        Err(_) => {
            Err(AppError::InvalidJsonFile)
        }
    }
}

/// Deserializa una cadena JSON en un objeto `serde_json::Value`.
pub fn deserialize_str_json_data(json_str: &str) -> Result<Vec<Tarea>, serde_json::Error> {
    let todos: Vec<Tarea> = serde_json::from_str(json_str)?;
    Ok(todos)
}

