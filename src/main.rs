mod cli;
mod storage;
mod commands;
mod config;

use crate::cli::{CliArgs, parse_args, Subcommand};
use crate::commands::{create_new_todo, list_todos, print_help};

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
            // Handle add subcommand
        }
        Subcommand::List => {
            list_todos();
        }
        Subcommand::Complete => {
            // Handle complete subcommand
        }
        Subcommand::Delete => {
            // Handle delete subcommand
        }
        Subcommand::Search => {
            // Handle search subcommand
        }
        Subcommand::Stats => {
            // Handle stats subcommand
        }
    }
}
