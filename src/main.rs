use std::process::ExitCode;

use studyflowcli::{
    cli::parse_args,
    commands::{
        add_todo, complete_todo, create_new_todo, delete_todo, get_stats, list_todos, print_help,
        search_todos,
    },
    config::data_path,
    errors::AppResult,
    models::Subcommand,
};

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("Error: {error}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> AppResult<()> {
    let raw_args: Vec<String> = std::env::args().collect();
    let args = parse_args(&raw_args)?;
    let data_file = data_path();

    match args.subcommand {
        Subcommand::Help => print_help()?,
        Subcommand::Init => create_new_todo(&data_file)?,
        Subcommand::Add => add_todo(&data_file, &args)?,
        Subcommand::List => list_todos(&data_file)?,
        Subcommand::Complete => complete_todo(&data_file, &args)?,
        Subcommand::Delete => delete_todo(&data_file, &args)?,
        Subcommand::Search => {
            search_todos(&data_file, &args)?;
        }
        Subcommand::Stats => get_stats(&data_file)?,
    }

    Ok(())
}
