use studyflowcli::{
    commands::{add_todo, complete_todo, create_new_todo, list_todos, print_help,
        delete_todo, search_todos, get_stats},
    config::DEFAULT_TODO_FILE,
    cli::{parse_args},
    models::{CliArgs, Subcommand},
};

fn main() {     
    let args: Vec<String> = std::env::args().collect();
    let args: CliArgs = parse_args(&args);

    match args.subcommand {
        Subcommand::Help  => {
            print_help();
        }
        Subcommand::Init => {
            create_new_todo(&DEFAULT_TODO_FILE);
        }
        Subcommand::Add => {
            match add_todo(&DEFAULT_TODO_FILE, &args) {
                Ok(success) => {
                    if success {
                        println!("Tarea agregada exitosamente.");
                    } else {
                        println!("No se pudo agregar la tarea.");
                    }
                }
                Err(e) => {
                    eprintln!("Error al agregar la tarea: {}", e);
                }
            }
        }
        Subcommand::List => {
            match list_todos() {
                Err(e) => {
                    println!("{}",e)
                },
                Ok(_) => {}
            }
        }
        Subcommand::Complete => {
            match complete_todo(&DEFAULT_TODO_FILE, &args) {
                Ok(()) => {
                    println!("Tarea completada exitosamente.");
                }
                Err(e) => {
                    eprintln!("Error al completar la tarea: {}", e);
                }
            }
        }
        Subcommand::Delete => {
            match delete_todo(&DEFAULT_TODO_FILE, &args) {
                Ok(()) => {
                    println!("Tarea eliminada exitosamente.");
                }
                Err(e) => {
                    eprintln!("Error al eliminar la tarea: {}", e);
                }
            }
        }
        Subcommand::Search => {
            match search_todos(&DEFAULT_TODO_FILE, &args) {
                Ok(results) => {
                    if results.is_empty() {
                        println!("No se encontraron tareas que coincidan con la búsqueda.");
                    }
                }
                Err(e) => {
                    eprintln!("Error al buscar tareas: {}", e);
                }
            }
        }
        Subcommand::Stats => {
            match get_stats(&DEFAULT_TODO_FILE) {
                Ok(_) => {
                    // Estadísticas mostradas exitosamente
                }
                Err(e) => {
                    eprintln!("Error al obtener estadísticas: {}", e);
                }
            }
        }
    }
}
