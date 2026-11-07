//! Definición de subcomandos, argumentos y opciones de la CLI.

use crate::models::{CliOptions , CliArgs, Subcommand};

pub fn print_cli_help() {
    println!("StudyFlow CLI - Administrador de Tareas");
    println!();
    println!("Uso:");
    println!("  studyflow <subcomando> [opciones]");
    println!();
    println!("Subcomandos:");
    println!("  help       Muestra esta ayuda.");
    println!("  init       Inicializa un nuevo archivo de tareas.");
    println!("  add        Agrega una nueva tarea.");
    println!("  list       Lista todas las tareas.");
    println!("  complete   Marca una tarea como completada.");
    println!("  delete     Elimina una tarea.");
    println!("  search     Busca tareas por palabra clave.");
    println!("  stats      Muestra estadísticas de las tareas.");
}

pub fn search_double_dashed_arg(args: &Vec<String>, arg_name: &str) -> Option<String> {
    let double_dashed_arg = format!("--{}", arg_name);
    for i in 0..args.len() {
        if args[i] == double_dashed_arg {
            if i + 1 < args.len() {
                return Some(args[i + 1].clone());
            }
        }
    }
    None
}

/// funcion para parsear los args esperados por erl cli.
pub fn options_to_struct(args: &Vec<String>) -> CliOptions {
    let priority = match search_double_dashed_arg(args, "priority") {
        Some(value) => value.parse::<u8>().unwrap_or(0),
        None => 3,
    };

    CliOptions { priority }
}

pub fn parse_args(args: &Vec<String>) -> CliArgs {
    let subcommand = args.get(1).cloned().unwrap_or_else(|| "help".into());
    let subcommand: Subcommand = match subcommand.as_str() {
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
    let options = args.into_iter().skip(2).cloned().collect::<Vec<String>>();
    let structured_options = options_to_struct(&args);
    
    CliArgs { subcommand, options, structured_options }
}