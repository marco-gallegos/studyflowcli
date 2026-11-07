//! Implementación de la lógica de cada comando.

use crate::storage::{create_new_todo_file, list_get_todo_content};

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