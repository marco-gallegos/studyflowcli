//! Implementación de la lógica de cada comando.
use std::io::{self, Error};
use serde_json;
use crate::storage::{create_new_todo_file, list_get_todo_content, file_exists};


/// Deserializa una cadena JSON en un objeto `serde_json::Value`.
pub fn deserialize_str_json_data(json_str: &str) -> Result<serde_json::Value, serde_json::Error> {
    serde_json::from_str(json_str)
}

pub fn print_help() {
    println!("StudyFlow CLI - Comandos disponibles:");
    println!("  help      - Muestra esta ayuda");
    println!("  init      - Inicializa un nuevo proyecto");
    println!("  add       - Agrega una nueva tarea");
    println!("  list      - Lista todas las tareas");
    println!("  complete  - Marca una tarea como completada");
    println!("  delete    - Elimina una tarea");
    println!("  search    - Busca tareas por palabra clave");
    println!("  stats     - Muestra estadísticas de las tareas");
}


pub fn create_new_todo(file_name: &str) {
    if create_new_todo_file(file_name) {
        println!("Nuevo proyecto inicializado en '{}'.", file_name);
    }
}


pub fn list_todos() {
    let contents = match list_get_todo_content() {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error al leer el archivo: {}", e);
            return;
        }
    };
    println!("Contenido del archivo '{}':", contents);
}


pub fn add_todo(file_name: &str) -> Result<bool, Box<dyn std::error::Error>> {
    if !file_exists(file_name) {
        return Err(Box::new(io::Error::new(io::ErrorKind::NotFound, "Archivo no encontrado. Inicializa un proyecto primero.")));
    }

    let contents = list_get_todo_content()?;
    let mut todos: serde_json::Value = deserialize_str_json_data(&contents)?;
    

    Ok(true)
}