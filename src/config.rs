

//! Configuración: ruta del archivo de datos y variables de entorno.
pub const DEFAULT_TODO_FILE: &str = "studyflow_project.json";


pub struct Config {
    pub todo_file: String,
}


pub fn get_config() -> Config {
    let todo_file = std::env::var("STUDYFLOW_TODO_FILE").unwrap_or_else(|_| DEFAULT_TODO_FILE.to_string());
    Config { todo_file }
}
