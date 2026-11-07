mod cli;
mod lib;
mod storage;
mod config;

use crate::cli::{CliArgs, parse_args};
use crate::lib::{print_help};
use crate::storage::{create_new_todo, list_todos};

use crate::config::DEFAULT_TODO_FILE;

fn main() {     
    let args: Vec<String> = std::env::args().collect();
    let args: CliArgs = parse_args(args);

    match args.subcommand.as_str() {
        "help" => {
            print_help();
        }
        "init" => {
            create_new_todo(&DEFAULT_TODO_FILE);
        }
        // "add" => {
        //     // Handle add subcommand
        // }
        "list" => {
            list_todos();
        }
        // "complete" => {
        //     // Handle complete subcommand
        // }
        // "delete" => {
        //     // Handle delete subcommand
        // }
        // "search" => {
        //     // Handle search subcommand
        // }
        // "stats" => {
        //     // Handle stats subcommand
        // }
        _ => {
            print_help();
        }
    }
}
