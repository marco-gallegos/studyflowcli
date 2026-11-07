//! Definición de subcomandos, argumentos y opciones de la CLI.

pub struct CliArgs {
    pub subcommand: String,
    pub options: Vec<String>,
}

pub enum Subcommand {
    Help,
    Init,
    Add,
    List,
    Complete,
    Delete,
    Search,
    Stats,
}

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

pub fn options_to_struct(args: &Vec<String>) -> CliArgs {
    let 
}

pub fn parse_args(args: Vec<String>) -> CliArgs {
    let subcommand = args.get(1).cloned().unwrap_or_else(|| "help".into());
    let options = args.into_iter().skip(2).collect();
    
    CliArgs { subcommand, options }
}