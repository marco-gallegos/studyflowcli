mod cli;
mod lib;

use cli::{CliArgs, parse_args};
use lib::{print_help, create_new_todo};

fn main() {     
    let args: Vec<String> = std::env::args().collect();
    let args: CliArgs = parse_args(args);

    match args.subcommand.as_str() {
        "help" => {
            print_help();
        }
        "init" => {
            create_new_todo();
        }
        // "add" => {
        //     // Handle add subcommand
        // }
        // "list" => {
        //     // Handle list subcommand
        // }
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

    println!("Hello, world! {} {}", args.subcommand, args.options.join(" "));
}
