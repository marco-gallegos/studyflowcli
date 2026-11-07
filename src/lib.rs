//! Núcleo de StudyFlow CLI.
//!
//! Aquí se declaran los módulos del proyecto y la API interna principal.
//! 

use std::fs::File;
use std::io::{self, Write, Read};
use std::path::Path;

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

pub fn create_new_todo() -> bool {
    let file_name = "studyflow_project.json";
    if std::path::Path::new(file_name).exists() {
        println!("El archivo '{}' ya existe. No se puede crear un nuevo proyecto.", file_name);
        return false;
    }
    let mut file = File::create(file_name).unwrap();
    writeln!(file, "{{}}").unwrap();
    file.flush().unwrap();
    return true;
}