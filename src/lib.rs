//! Núcleo de StudyFlow CLI.
//!
//! Aquí se declaran los módulos del proyecto y la API interna principal.
//! 


pub fn print_help() {
    println!("StudyFlow CLI - Comandos disponibles:");
    println!("  help      - Muestra esta ayuda");
    println!("  init      - Inicializa un nuevo proyecto");
    println!("  add       - Agrega una nueva tarea");
    println!("  list      - Lista todas las tareas");
    println!("  complete  - Marca una tarea como completada");
    println!("  delete    - Elimina una tarea");
    println!("  search    - Busca tareas por palabra clave");
    println!("  stats     - Muestra estadísticas de las tareas");
}