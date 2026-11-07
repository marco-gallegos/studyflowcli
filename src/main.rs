mod cli;

use cli::{CliArgs, parse_args};

fn main() {     
    let args: Vec<String> = std::env::args().collect();
    let args: CliArgs = parse_args(args); 

    println!("Hello, world! {} {}", args.subcommand, args.options.join(" "));
}
