//! Lectura y escritura del archivo de datos.

use std::fs;

use crate::errors::{AppError, AppResult};

/// Comprueba si la ruta corresponde a un archivo existente.
///
/// # Examples
///
/// ```
/// use std::fs;
/// use studyflowcli::storage::file_exists;
///
/// let ruta = std::env::temp_dir().join(format!(
///     "studyflow_doc_file_exists_{}.json",
///     std::process::id()
/// ));
/// let nombre = ruta.to_str().unwrap();
/// let _ = fs::remove_file(nombre);
///
/// assert!(!file_exists(nombre));
///
/// fs::write(nombre, "[]").unwrap();
/// assert!(file_exists(nombre));
///
/// fs::remove_file(nombre).unwrap();
/// ```
pub fn file_exists(file_name: &str) -> bool {
    fs::metadata(file_name).is_ok()
}

/// Crea un archivo de datos vacío.
///
/// Falla si el archivo ya existe para no sobrescribir datos.
///
/// # Examples
///
/// ```
/// use std::fs;
/// use studyflowcli::storage::create_new_todo_file;
///
/// let ruta = std::env::temp_dir().join(format!(
///     "studyflow_doc_create_{}.json",
///     std::process::id()
/// ));
/// let nombre = ruta.to_str().unwrap();
/// let _ = fs::remove_file(nombre);
///
/// create_new_todo_file(nombre).unwrap();
/// assert_eq!(fs::read_to_string(nombre).unwrap(), "[]\n");
///
/// // Una segunda llamada no debe sobrescribir el archivo existente.
/// assert!(create_new_todo_file(nombre).is_err());
///
/// fs::remove_file(nombre).unwrap();
/// ```
pub fn create_new_todo_file(file_name: &str) -> AppResult<()> {
    if file_exists(file_name) {
        return Err(AppError::ArchivoYaExiste(file_name.to_string()));
    }

    fs::write(file_name, "[]\n")?;
    Ok(())
}

/// Lee el contenido del archivo de datos.
///
/// # Examples
///
/// ```
/// use std::fs;
/// use studyflowcli::storage::read_todo_file;
///
/// let ruta = std::env::temp_dir().join(format!(
///     "studyflow_doc_read_{}.json",
///     std::process::id()
/// ));
/// let nombre = ruta.to_str().unwrap();
/// fs::write(nombre, "[]\n").unwrap();
///
/// assert_eq!(read_todo_file(nombre).unwrap(), "[]\n");
///
/// fs::remove_file(nombre).unwrap();
///
/// // Un archivo inexistente produce un error descriptivo.
/// assert!(read_todo_file(nombre).is_err());
/// ```
pub fn read_todo_file(file_name: &str) -> AppResult<String> {
    if !file_exists(file_name) {
        return Err(AppError::ArchivoNoEncontrado(file_name.to_string()));
    }

    Ok(fs::read_to_string(file_name)?)
}
