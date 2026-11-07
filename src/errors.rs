//! Tipos de error de la aplicación y alias de `Result`.

use std::fmt;
use std::io;

/// Errores recuperables de StudyFlow CLI.
#[derive(Debug)]
pub enum AppError {
    Io(io::Error),
    Json(serde_json::Error),
    ArchivoNoEncontrado(String),
    ArchivoYaExiste(String),
    TareaNoEncontrada(u64),
    DescripcionVacia,
    BusquedaVacia,
    IdInvalido(String),
    PrioridadInvalida(String),
    ComandoDesconocido(String),
    ArgumentoFaltante {
        comando: &'static str,
        uso: &'static str,
    },
    Reloj(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(error) => write!(f, "Error de entrada/salida: {error}"),
            Self::Json(error) => {
                write!(f, "El archivo de datos no contiene JSON válido: {error}")
            }
            Self::ArchivoNoEncontrado(path) => {
                write!(
                    f,
                    "No se encontró el archivo '{path}'. Ejecuta 'init' primero."
                )
            }
            Self::ArchivoYaExiste(path) => write!(f, "El archivo '{path}' ya existe."),
            Self::TareaNoEncontrada(id) => write!(f, "No existe una tarea con ID {id}."),
            Self::DescripcionVacia => write!(f, "La descripción no puede estar vacía."),
            Self::BusquedaVacia => write!(f, "La búsqueda no puede estar vacía."),
            Self::IdInvalido(value) => write!(f, "El ID '{value}' no es un número válido."),
            Self::PrioridadInvalida(value) => {
                write!(f, "La prioridad '{value}' no es un número válido.")
            }
            Self::ComandoDesconocido(command) => write!(
                f,
                "Comando desconocido: '{command}'. Usa 'help' para ver las opciones."
            ),
            Self::ArgumentoFaltante { comando, uso } => {
                write!(f, "Falta un argumento para '{comando}'. Uso: {uso}")
            }
            Self::Reloj(error) => write!(f, "No se pudo obtener la hora del sistema: {error}"),
        }
    }
}

impl std::error::Error for AppError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(error) => Some(error),
            Self::Json(error) => Some(error),
            _ => None,
        }
    }
}

impl From<io::Error> for AppError {
    fn from(error: io::Error) -> Self {
        Self::Io(error)
    }
}

impl From<serde_json::Error> for AppError {
    fn from(error: serde_json::Error) -> Self {
        Self::Json(error)
    }
}

/// Resultado de una operación de StudyFlow CLI.
pub type AppResult<T> = Result<T, AppError>;
