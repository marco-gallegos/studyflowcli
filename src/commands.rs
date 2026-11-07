//! Implementación de la lógica de cada comando.
use std::{io::{self}};
use serde_json::{self};
use crate::{
    cli::{print_cli_help}, models::{EstadoTarea, Tarea, CliArgs}, storage::{create_new_todo_file, file_exists, list_get_todo_content},
};


/// Deserializa una cadena JSON en un objeto `serde_json::Value`.
pub fn deserialize_str_json_data(json_str: &str) -> Result<Vec<Tarea>, serde_json::Error> {
    let todos: Vec<Tarea> = serde_json::from_str(json_str)?;
    Ok(todos)
}

pub fn filter_active_todos(todos: Vec<Tarea>) -> Vec<Tarea> {
    todos.into_iter().filter(|t| !matches!(t.estado, EstadoTarea::Completada)).collect()
}


pub fn get_todos(file_name: &str, just_active: bool) -> Result<Vec<Tarea>, Box<dyn std::error::Error>> {
    if !file_exists(file_name) {
        return Err(Box::new(io::Error::new(io::ErrorKind::NotFound, "Archivo no encontrado. Inicializa un proyecto primero.")));
    }

    let contents = list_get_todo_content()?;
    let todos: Vec<Tarea> = match deserialize_str_json_data(&contents) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Error al deserializar el contenido JSON: {}", e);
            return Err(Box::new(e));
        }
    };

    let todos = if just_active {
        filter_active_todos(todos)
    } else {
        todos
    };

    Ok(todos)
}


pub fn get_todo_by_id(todos: &Vec<Tarea>, id: u32) -> Result<Option<u32>, Box<dyn std::error::Error>> {
    for (index, todo) in todos.into_iter().enumerate() {
        if todo.id == id {
            return Ok(Some(index as u32));
        }
    }

    Ok(None)
}

pub fn print_help() {
    print_cli_help();
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
    let todos: Vec<Tarea> = match deserialize_str_json_data(&contents) {
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
        println!("tarea {}: {} (Completada: {:?}, Prioridad: {})", todo.1.id, todo.1.descripcion, todo.1.estado , todo.1.prioridad);
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

    let todo_description: String = args.options[0].clone();
    let creado_en: String = "hoy".to_string();
    let cvategoria: Option<String> = None; // Puedes cambiar esto según tus necesidades



    let contents = list_get_todo_content()?;
    let mut todos: Vec<Tarea> = match deserialize_str_json_data(&contents) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Error al deserializar el contenido JSON: {}", e);
            return Ok(false);
        }
    };


    let new_todo = Tarea::nueva_tarea(&todo_description, &EstadoTarea::Pendiente, &(3),  &cvategoria, &creado_en);

    todos.push(new_todo);

    let updated_json = serde_json::to_string(&todos)?;
    std::fs::write(file_name, updated_json)?;
    

    Ok(true)
}

pub fn complete_todo(file_name: &str, args: &CliArgs) -> Result<(), Box<dyn std::error::Error>> {
    if args.options.is_empty() {
        println!("Error: No se proporcionó un ID de tarea para completar.");
        return Ok(());
    }

    let id: u32 = match args.options[0].parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Error: El ID de tarea debe ser un número válido.");
            return Ok(());
        }
    };
    
    let mut todos = get_todos(file_name, true)?;

    if let Some(index) = get_todo_by_id(&todos, id)? {
        todos[index as usize].estado = EstadoTarea::Completada;

        let updated_json = serde_json::to_string(&todos)?;
        std::fs::write(file_name, updated_json)?;

        println!("Tarea con ID {} marcada como completada.", id);
    } else {
        println!("No se encontró ninguna tarea con ID {}.", id);
    }
    
    Ok(())
}


pub fn delete_todo(file_name: &str, args: &CliArgs) -> Result<(), Box<dyn std::error::Error>> {
    if args.options.is_empty() {
        println!("Error: No se proporcionó un ID de tarea para eliminar.");
        return Ok(());
    }

    let id: u32 = match args.options[0].parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Error: El ID de tarea debe ser un número válido.");
            return Ok(());
        }
    };
    
    let mut todos = get_todos(file_name, false)?;

    if let Some(index) = get_todo_by_id(&todos, id)? {
        todos.remove(index as usize);

        let updated_json = serde_json::to_string(&todos)?;
        std::fs::write(file_name, updated_json)?;

        println!("Tarea con ID {} eliminada exitosamente.", id);
    } else {
        println!("No se encontró ninguna tarea con ID {}.", id);
    }
    
    Ok(())
}

pub fn search_todos(file_name: &str, args: &CliArgs) -> Result<Vec<Tarea>, std::io::Error> {
    if args.options.is_empty() {
        return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "No se proporcionó una palabra clave para la búsqueda."));
    }

    let keyword = &args.options[0].to_lowercase();
    let todos = match get_todos(file_name, false) {
        Ok(todos) => todos,
        Err(e) => return Err(std::io::Error::new(std::io::ErrorKind::Other, format!("Error al obtener las tareas: {}", e))),
    };

    let results: Vec<Tarea> = todos.into_iter()
        .filter(|t| t.descripcion.to_lowercase().contains(keyword))
        .collect();

    if results.is_empty() {
        println!("No se encontraron tareas que coincidan con la búsqueda.");
    } else {
        for todo in &results {
            println!("tarea {}: {} (Completada: {:?}, Prioridad: {})", todo.id, todo.descripcion, todo.estado, todo.prioridad);
        }
    }

    Ok(results)
}

pub fn get_stats(file_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let todos = get_todos(file_name, false)?;

    let total = todos.len();
    let completed = todos.iter().filter(|t| matches!(t.estado, EstadoTarea::Completada) ).count();
    let pending = total - completed;

    println!("Estadísticas de tareas:");
    println!("Total de tareas: {}", total);
    println!("Tareas completadas: {}", completed);
    println!("Tareas pendientes: {}", pending);

    Ok(())
}