//! Definición de subcomandos, argumentos y opciones de la CLI.

use crate::errors::{AppError, AppResult};
use crate::models::{CliArgs, CliOptions, Subcommand};

/// Muestra la ayuda de la CLI en la salida estándar.
///
/// # Examples
///
/// ```
/// studyflowcli::cli::print_cli_help();
/// ```
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

/// Busca un argumento con doble guion y devuelve el valor que le sigue.
///
/// # Examples
///
/// ```
/// use studyflowcli::cli::search_double_dashed_arg;
///
/// let args: Vec<String> = ["studyflowcli", "add", "tarea", "--priority", "4"]
///     .iter()
///     .map(|arg| arg.to_string())
///     .collect();
///
/// assert_eq!(search_double_dashed_arg(&args, "priority").as_deref(), Some("4"));
/// assert_eq!(search_double_dashed_arg(&args, "categoria"), None);
/// ```
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
///
/// Si no se indica `--priority`, se usa `3` como prioridad por defecto.
///
/// # Examples
///
/// ```
/// use studyflowcli::cli::options_to_struct;
///
/// let args: Vec<String> = ["studyflowcli", "add", "tarea", "--priority", "5"]
///     .iter()
///     .map(|arg| arg.to_string())
///     .collect();
///
/// assert_eq!(options_to_struct(&args).unwrap().priority, 5);
///
/// let args: Vec<String> = ["studyflowcli", "add", "tarea"]
///     .iter()
///     .map(|arg| arg.to_string())
///     .collect();
///
/// assert_eq!(options_to_struct(&args).unwrap().priority, 3);
/// ```
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

/// Convierte los argumentos del proceso en una estructura [`CliArgs`].
///
/// El primer elemento se interpreta como el nombre del programa y el segundo
/// como el subcomando. Si no se indica subcomando, se usa `help`.
///
/// # Errors
///
/// Devuelve [`AppError::ComandoDesconocido`] si el subcomando no existe y
/// [`AppError::PrioridadInvalida`] si `--priority` no es un número válido.
///
/// # Examples
///
/// ```
/// use studyflowcli::cli::parse_args;
/// use studyflowcli::models::Subcommand;
///
/// let args: Vec<String> = ["studyflowcli", "add", "Repasar Rust", "--priority", "2"]
///     .iter()
///     .map(|arg| arg.to_string())
///     .collect();
///
/// let parsed = parse_args(&args).unwrap();
///
/// assert!(matches!(parsed.subcommand, Subcommand::Add));
/// assert_eq!(parsed.options, vec!["Repasar Rust".to_string()]);
/// assert_eq!(parsed.structured_options.priority, 2);
///
/// let args: Vec<String> = ["studyflowcli", "desconocido"]
///     .iter()
///     .map(|arg| arg.to_string())
///     .collect();
///
/// assert!(parse_args(&args).is_err());
/// ```
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
