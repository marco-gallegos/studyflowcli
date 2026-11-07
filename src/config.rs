//! Configuración: ruta del archivo de datos y variables de entorno.

/// Nombre del archivo de datos usado cuando no hay variable de entorno.
///
/// # Examples
///
/// ```
/// use studyflowcli::config::DEFAULT_TODO_FILE;
///
/// assert_eq!(DEFAULT_TODO_FILE, "studyflow_project.json");
/// ```
pub const DEFAULT_TODO_FILE: &str = "studyflow_project.json";

/// Devuelve la ruta del archivo de datos.
///
/// Usa `STUDYFLOW_DATA_PATH` cuando está definida y no está vacía. En caso
/// contrario devuelve [`DEFAULT_TODO_FILE`].
///
/// # Examples
///
/// ```
/// use studyflowcli::config::data_path;
///
/// let ruta = data_path();
///
/// assert!(!ruta.is_empty());
/// ```
pub fn data_path() -> String {
    std::env::var("STUDYFLOW_DATA_PATH")
        .ok()
        .filter(|path| !path.trim().is_empty())
        .unwrap_or_else(|| DEFAULT_TODO_FILE.to_string())
}
