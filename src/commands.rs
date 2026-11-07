//! Implementación de la lógica de cada comando.
use std::io::{self, Error};
use serde_json::{self, value::Index};
use crate::{cli::CliArgs, storage::{create_new_todo_file, file_exists, list_get_todo_content}};
use crate::models::Tarea;


/// Deserializa una cadena JSON en un objeto `serde_json::Value`.
pub fn deserialize_str_json_data(json_str: &str) -> Result<Vec<Tarea>, serde_json::Error> {
    let todos: Vec<Tarea> = serde_json::from_str(json_str)?;
    Ok(todos)
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
    let mut todos: Vec<Tarea> = match deserialize_str_json_data(&contents) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Error al deserializar el contenido JSON: {}", e);
            return;
        }
    };

    if todos.is_empty() {
        println!("No hay tareas en el proyecto.");
        return;
    }

    for todo in todos.into_iter().enumerate() {
        println!("tarea {}: {} (Completada: {}, Prioridad: {})", todo.1.id, todo.1.descripcion, todo.1.completada, todo.1.prioridad);
    }
}


pub fn add_todo(file_name: &str, args: &CliArgs) -> Result<bool, Box<dyn std::error::Error>> {
    if !file_exists(file_name) {
        return Err(Box::new(io::Error::new(io::ErrorKind::NotFound, "Archivo no encontrado. Inicializa un proyecto primero.")));
    }
    

    // validar inputs
    if args.options.is_empty() {
        println!("Error: No se proporcionó una descripción para la nueva tarea.");
        return Ok(false);
    }

    let todo_description = args.options[0].clone();



    let contents = list_get_todo_content()?;
    let mut todos: Vec<Tarea> = match deserialize_str_json_data(&contents) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Error al deserializar el contenido JSON: {}", e);
            return Ok(false);
        }
    };


    let new_todo = Tarea::nueva_tarea(todo_description.clone(), false, 3);

    todos.push(new_todo);

    let updated_json = serde_json::to_string(&todos)?;
    std::fs::write(file_name, updated_json)?;
    

    Ok(true)
}