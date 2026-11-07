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

pub fn parse_args(args: Vec<String>) -> CliArgs {
    let subcommand = args.get(1).cloned().unwrap_or_else(|| "help".into());
    let options = args.into_iter().skip(2).collect();
    
    CliArgs { subcommand, options }
}