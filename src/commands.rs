//! Implementación de la lógica de cada comando.

use std::fs;

use crate::{
    cli::print_cli_help,
    errors::{AppError, AppResult},
    models::{CliArgs, EstadoTarea, Tarea},
    reports::stats_report,
    storage::{create_new_todo_file, file_exists, read_todo_file},
};

/// Deserializa las tareas almacenadas en una cadena JSON.
pub fn deserialize_str_json_data(json_str: &str) -> AppResult<Vec<Tarea>> {
    Ok(serde_json::from_str(json_str)?)
}

pub fn filter_active_todos(todos: Vec<Tarea>) -> Vec<Tarea> {
    todos
        .into_iter()
        .filter(|tarea| !matches!(tarea.estado, EstadoTarea::Completada))
        .collect()
}

pub fn get_todos(file_name: &str, just_active: bool) -> AppResult<Vec<Tarea>> {
    if !file_exists(file_name) {
        return Err(AppError::ArchivoNoEncontrado(file_name.to_string()));
    }

    let contents = read_todo_file(file_name)?;
    let todos = deserialize_str_json_data(&contents)?;

    Ok(if just_active {
        filter_active_todos(todos)
    } else {
        todos
    })
}

pub fn get_todo_by_id(todos: &[Tarea], id: u64) -> Option<usize> {
    todos.iter().position(|tarea| tarea.id == id)
}

pub fn print_help() -> AppResult<()> {
    print_cli_help();
    Ok(())
}

pub fn create_new_todo(file_name: &str) -> AppResult<()> {
    create_new_todo_file(file_name)?;
    println!("Nuevo proyecto inicializado en '{file_name}'.");
    Ok(())
}

pub fn list_todos(file_name: &str) -> AppResult<()> {
    let todos = get_todos(file_name, false)?;

    if todos.is_empty() {
        println!("No hay tareas en el proyecto.");
        return Ok(());
    }

    for tarea in &todos {
        println!(
            "tarea {}: {} (Estado: {:?}, Prioridad: {})",
            tarea.id, tarea.descripcion, tarea.estado, tarea.prioridad
        );
    }

    Ok(())
}

pub fn add_todo(file_name: &str, args: &CliArgs) -> AppResult<()> {
    let raw_description = args.options.first().ok_or(AppError::ArgumentoFaltante {
        comando: "add",
        uso: "studyflowcli add \"descripción\" [--priority N]",
    })?;

    let description = raw_description.trim();
    if description.is_empty() {
        return Err(AppError::DescripcionVacia);
    }

    let mut todos = get_todos(file_name, false)?;
    let descripcion = description.to_string();
    let categoria: Option<String> = None;
    let creado_en = "hoy".to_string();

    let new_todo = Tarea::nueva_tarea(
        &descripcion,
        &EstadoTarea::Pendiente,
        &args.structured_options.priority,
        &categoria,
        &creado_en,
    )?;

    todos.push(new_todo);
    fs::write(file_name, serde_json::to_string(&todos)?)?;

    println!("Tarea agregada exitosamente.");
    Ok(())
}

fn parse_task_id(args: &CliArgs, comando: &'static str, uso: &'static str) -> AppResult<u64> {
    let raw_id = args
        .options
        .first()
        .ok_or(AppError::ArgumentoFaltante { comando, uso })?;

    raw_id
        .parse::<u64>()
        .map_err(|_| AppError::IdInvalido(raw_id.clone()))
}

pub fn complete_todo(file_name: &str, args: &CliArgs) -> AppResult<()> {
    let id = parse_task_id(args, "complete", "studyflowcli complete <id>")?;
    let mut todos = get_todos(file_name, false)?;
    let index = get_todo_by_id(&todos, id).ok_or(AppError::TareaNoEncontrada(id))?;

    todos[index].estado = EstadoTarea::Completada;
    fs::write(file_name, serde_json::to_string(&todos)?)?;

    println!("Tarea con ID {id} marcada como completada.");
    Ok(())
}

pub fn delete_todo(file_name: &str, args: &CliArgs) -> AppResult<()> {
    let id = parse_task_id(args, "delete", "studyflowcli delete <id>")?;
    let mut todos = get_todos(file_name, false)?;
    let index = get_todo_by_id(&todos, id).ok_or(AppError::TareaNoEncontrada(id))?;

    todos.remove(index);
    fs::write(file_name, serde_json::to_string(&todos)?)?;

    println!("Tarea con ID {id} eliminada exitosamente.");
    Ok(())
}

pub fn search_todos(file_name: &str, args: &CliArgs) -> AppResult<Vec<Tarea>> {
    let raw_keyword = args.options.first().ok_or(AppError::ArgumentoFaltante {
        comando: "search",
        uso: "studyflowcli search <texto>",
    })?;

    let keyword = raw_keyword.trim().to_lowercase();
    if keyword.is_empty() {
        return Err(AppError::BusquedaVacia);
    }

    let todos = get_todos(file_name, false)?;
    let results: Vec<Tarea> = todos
        .into_iter()
        .filter(|tarea| {
            tarea.descripcion.to_lowercase().contains(&keyword)
                || tarea
                    .categoria
                    .as_ref()
                    .is_some_and(|categoria| categoria.to_lowercase().contains(&keyword))
                || tarea.prioridad.to_string() == keyword
        })
        .collect();

    if results.is_empty() {
        println!("No se encontraron tareas que coincidan con la búsqueda.");
    } else {
        for tarea in &results {
            println!(
                "tarea {}: {} (Estado: {:?}, Prioridad: {})",
                tarea.id, tarea.descripcion, tarea.estado, tarea.prioridad
            );
        }
    }

    Ok(results)
}

pub fn get_stats(file_name: &str) -> AppResult<()> {
    let todos = get_todos(file_name, false)?;

    let report = stats_report(&todos);

    println!("Estadísticas de tareas:");
    println!("Total de tareas: {}", report.total);
    println!("Tareas completadas: {}", report.completed);
    println!("Tareas pendientes: {}", report.pending);
    println!("Avance: {:.1}%", report.percentage);

    Ok(())
}
