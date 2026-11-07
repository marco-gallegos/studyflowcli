//! Definición de subcomandos, argumentos y opciones de la CLI.

use crate::errors::{AppError, AppResult};
use crate::models::{CliArgs, CliOptions, Subcommand};

pub fn print_cli_help() {
    println!("StudyFlow CLI - Administrador de Tareas");
    println!();
    println!("Uso:");
    println!("  studyflowcli <subcomando> [opciones]");
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

pub fn search_double_dashed_arg(args: &[String], arg_name: &str) -> Option<String> {
    let double_dashed_arg = format!("--{arg_name}");

    for (index, arg) in args.iter().enumerate() {
        if arg == &double_dashed_arg {
            return args.get(index + 1).cloned();
        }
    }

    None
}

/// Convierte las opciones de texto a una estructura tipada.
pub fn options_to_struct(args: &[String]) -> AppResult<CliOptions> {
    let priority = match search_double_dashed_arg(args, "priority") {
        Some(value) => value
            .parse::<u8>()
            .map_err(|_| AppError::PrioridadInvalida(value))?,
        None => 3,
    };

    Ok(CliOptions { priority })
}

fn positional_options(args: &[String]) -> Vec<String> {
    let mut options = Vec::new();
    let mut skip_next = false;

    for arg in args.iter().skip(2) {
        if skip_next {
            skip_next = false;
            continue;
        }

        if arg == "--priority" {
            skip_next = true;
            continue;
        }

        options.push(arg.clone());
    }

    options
}

pub fn parse_args(args: &[String]) -> AppResult<CliArgs> {
    let subcommand_text = args.get(1).cloned().unwrap_or_else(|| "help".into());

    let subcommand = match subcommand_text.as_str() {
        "help" | "--help" | "-h" => Subcommand::Help,
        "init" => Subcommand::Init,
        "add" => Subcommand::Add,
        "list" => Subcommand::List,
        "complete" => Subcommand::Complete,
        "delete" => Subcommand::Delete,
        "search" => Subcommand::Search,
        "stats" => Subcommand::Stats,
        _ => return Err(AppError::ComandoDesconocido(subcommand_text)),
    };

    let options = positional_options(args);
    let structured_options = options_to_struct(args)?;

    Ok(CliArgs {
        subcommand,
        options,
        structured_options,
    })
}
