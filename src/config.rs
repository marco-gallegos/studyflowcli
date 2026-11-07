//! Configuración: ruta del archivo de datos y variables de entorno.

pub const DEFAULT_TODO_FILE: &str = "studyflow_project.json";

/// Devuelve la ruta del archivo de datos.
///
/// Usa `STUDYFLOW_DATA_PATH` cuando está definida y no está vacía.
pub fn data_path() -> String {
    std::env::var("STUDYFLOW_DATA_PATH")
        .ok()
        .filter(|path| !path.trim().is_empty())
        .unwrap_or_else(|| DEFAULT_TODO_FILE.to_string())
}
