mod cli;
mod storage;
mod commands;
mod config;
mod models;

use crate::cli::{CliArgs, parse_args, Subcommand};
use crate::commands::{
    add_todo, complete_todo, create_new_todo, list_todos, print_help,
    delete_todo, search_todos, get_stats
};

use crate::config::DEFAULT_TODO_FILE;

fn main() {     
    let args: Vec<String> = std::env::args().collect();
    let args: CliArgs = parse_args(args);

    let command = match args.subcommand.as_str() {
        "help" => Subcommand::Help,
        "init" => Subcommand::Init,
        "add" => Subcommand::Add,
        "list" => Subcommand::List,
        "complete" => Subcommand::Complete,
        "delete" => Subcommand::Delete,
        "search" => Subcommand::Search,
        "stats" => Subcommand::Stats,
        _ => Subcommand::Help,
    };

    match command {
        Subcommand::Help  => {
            print_help();
        }
        Subcommand::Init => {
            create_new_todo(&DEFAULT_TODO_FILE);
        }
        Subcommand::Add => {
            match add_todo(&DEFAULT_TODO_FILE, &args) {
                Ok(success) => {
                    if success {
                        println!("Tarea agregada exitosamente.");
                    } else {
                        println!("No se pudo agregar la tarea.");
                    }
                }
                Err(e) => {
                    eprintln!("Error al agregar la tarea: {}", e);
                }
            }
        }
        Subcommand::List => {
            list_todos();
        }
        Subcommand::Complete => {
            match complete_todo(&DEFAULT_TODO_FILE, &args) {
                Ok(()) => {
                    println!("Tarea completada exitosamente.");
                }
                Err(e) => {
                    eprintln!("Error al completar la tarea: {}", e);
                }
            }
        }
        Subcommand::Delete => {
            match delete_todo(&DEFAULT_TODO_FILE, &args) {
                Ok(()) => {
                    println!("Tarea eliminada exitosamente.");
                }
                Err(e) => {
                    eprintln!("Error al eliminar la tarea: {}", e);
                }
            }
        }
        Subcommand::Search => {
            match search_todos(&DEFAULT_TODO_FILE, &args) {
                Ok(results) => {
                    if results.is_empty() {
                        println!("No se encontraron tareas que coincidan con la búsqueda.");
                    }
                }
                Err(e) => {
                    eprintln!("Error al buscar tareas: {}", e);
                }
            }
        }
        Subcommand::Stats => {
            match get_stats(&DEFAULT_TODO_FILE) {
                Ok(_) => {
                    // Estadísticas mostradas exitosamente
                }
                Err(e) => {
                    eprintln!("Error al obtener estadísticas: {}", e);
                }
            }
        }
    }
}
