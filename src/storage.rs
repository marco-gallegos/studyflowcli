//! Lectura y escritura del archivo de datos.

use std::fs;

use crate::errors::{AppError, AppResult};

/// Comprueba si la ruta corresponde a un archivo existente.
pub fn file_exists(file_name: &str) -> bool {
    fs::metadata(file_name).is_ok()
}

/// Crea un archivo de datos vacío.
///
/// Falla si el archivo ya existe para no sobrescribir datos.
pub fn create_new_todo_file(file_name: &str) -> AppResult<()> {
    if file_exists(file_name) {
        return Err(AppError::ArchivoYaExiste(file_name.to_string()));
    }

    fs::write(file_name, "[]\n")?;
    Ok(())
}

/// Lee el contenido del archivo de datos.
pub fn read_todo_file(file_name: &str) -> AppResult<String> {
    if !file_exists(file_name) {
        return Err(AppError::ArchivoNoEncontrado(file_name.to_string()));
    }

    Ok(fs::read_to_string(file_name)?)
}
