//! Tipos de error de la aplicación y alias de `Result`.

use std::fmt;
use std::io;

/// Errores recuperables de StudyFlow CLI.
///
/// # Ejemplo
///
/// ```
/// use studyflowcli::errors::AppError;
///
/// let error = AppError::DescripcionVacia;
///
/// assert_eq!(error.to_string(), "La descripción no puede estar vacía.");
/// ```
#[derive(Debug)]
pub enum AppError {
    /// Falló una operación de entrada o salida.
    Io(io::Error),
    /// El archivo de datos no contiene JSON válido.
    Json(serde_json::Error),
    /// No existe el archivo de datos indicado.
    ArchivoNoEncontrado(String),
    /// Ya existe un archivo donde se intentó inicializar el proyecto.
    ArchivoYaExiste(String),
    /// No existe una tarea con el ID indicado.
    TareaNoEncontrada(u64),
    /// La descripción de una tarea está vacía.
    DescripcionVacia,
    /// El texto de búsqueda está vacío.
    BusquedaVacia,
    /// El valor recibido como ID no es un número válido.
    IdInvalido(String),
    /// El valor recibido como prioridad no es un número válido.
    PrioridadInvalida(String),
    /// El subcomando no existe.
    ComandoDesconocido(String),
    /// Falta un argumento obligatorio para un comando.
    ArgumentoFaltante {
        /// Nombre del comando que necesita el argumento.
        comando: &'static str,
        /// Forma de uso esperada.
        uso: &'static str,
    },
    /// No se pudo consultar la hora del sistema.
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
///
/// # Ejemplo
///
/// ```
/// use studyflowcli::errors::{AppError, AppResult};
///
/// let result: AppResult<u8> = Ok(3);
/// assert_eq!(result.unwrap(), 3);
///
/// let error: AppResult<u8> = Err(AppError::DescripcionVacia);
/// assert!(error.is_err());
/// ```
pub type AppResult<T> = Result<T, AppError>;

#[cfg(test)]
mod tests {
    use std::error::Error;

    use super::*;

    #[test]
    fn los_mensajes_son_entendibles() {
        assert_eq!(
            AppError::DescripcionVacia.to_string(),
            "La descripción no puede estar vacía."
        );
        assert_eq!(
            AppError::BusquedaVacia.to_string(),
            "La búsqueda no puede estar vacía."
        );
        assert_eq!(
            AppError::TareaNoEncontrada(42).to_string(),
            "No existe una tarea con ID 42."
        );
        assert_eq!(
            AppError::IdInvalido("abc".to_string()).to_string(),
            "El ID 'abc' no es un número válido."
        );
        assert_eq!(
            AppError::PrioridadInvalida("alta".to_string()).to_string(),
            "La prioridad 'alta' no es un número válido."
        );
        assert_eq!(
            AppError::ComandoDesconocido("foo".to_string()).to_string(),
            "Comando desconocido: 'foo'. Usa 'help' para ver las opciones."
        );
        assert_eq!(
            AppError::ArchivoNoEncontrado("datos.json".to_string()).to_string(),
            "No se encontró el archivo 'datos.json'. Ejecuta 'init' primero."
        );
        assert_eq!(
            AppError::ArchivoYaExiste("datos.json".to_string()).to_string(),
            "El archivo 'datos.json' ya existe."
        );
    }

    #[test]
    fn argumento_faltante_incluye_el_uso() {
        let error = AppError::ArgumentoFaltante {
            comando: "add",
            uso: "studyflowcli add <descripción>",
        };

        assert_eq!(
            error.to_string(),
            "Falta un argumento para 'add'. Uso: studyflowcli add <descripción>"
        );
    }

    #[test]
    fn los_errores_de_io_y_json_exponen_su_causa() {
        let io_error = AppError::from(io::Error::new(io::ErrorKind::NotFound, "sin archivo"));
        assert!(io_error.source().is_some());

        let json_error = AppError::from(serde_json::from_str::<u8>("no").unwrap_err());
        assert!(json_error.source().is_some());
    }

    #[test]
    fn los_errores_simples_no_tienen_causa() {
        assert!(AppError::DescripcionVacia.source().is_none());
    }
}
