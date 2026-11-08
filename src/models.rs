//! Modelos de datos utilizados por StudyFlow CLI.

use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::errors::{AppError, AppResult};

static NEXT_ID: AtomicU64 = AtomicU64::new(0);

/// Genera un ID único a partir de la marca de tiempo y un contador.
///
/// # Examples
///
/// ```
/// use studyflowcli::models::generate_id;
///
/// let first = generate_id().unwrap();
/// let second = generate_id().unwrap();
///
/// assert_ne!(first, second);
/// ```
pub fn generate_id() -> AppResult<u64> {
    let start = SystemTime::now();
    let since_epoch = start
        .duration_since(UNIX_EPOCH)
        .map_err(|error| AppError::Reloj(error.to_string()))?;

    let timestamp = since_epoch.as_nanos() as u64;
    let sequence = NEXT_ID.fetch_add(1, Ordering::Relaxed);

    Ok(timestamp.wrapping_add(sequence))
}

/// Subcomando que el usuario solicitó ejecutar.
///
/// # Examples
///
/// ```
/// use studyflowcli::models::Subcommand;
///
/// let command = Subcommand::Add;
///
/// assert!(matches!(command, Subcommand::Add));
/// ```
pub enum Subcommand {
    /// Muestra la ayuda.
    Help,
    /// Inicializa el archivo de datos.
    Init,
    /// Agrega una tarea.
    Add,
    /// Lista las tareas.
    List,
    /// Marca una tarea como completada.
    Complete,
    /// Elimina una tarea.
    Delete,
    /// Busca tareas por texto.
    Search,
    /// Muestra estadísticas.
    Stats,
}

/// Estado de una tarea.
///
/// # Examples
///
/// ```
/// use studyflowcli::models::EstadoTarea;
///
/// let json = serde_json::to_string(&EstadoTarea::Pendiente).unwrap();
/// assert_eq!(json, "\"pendiente\"");
/// ```
#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum EstadoTarea {
    /// La tarea todavía no fue completada.
    Pendiente,
    /// La tarea ya fue completada.
    Completada,
}

/// Tarea de estudio almacenada por la aplicación.
///
/// # Examples
///
/// ```
/// use studyflowcli::models::{EstadoTarea, Tarea};
///
/// let tarea = Tarea {
///     id: 1,
///     descripcion: "Leer el capítulo 5".to_string(),
///     estado: EstadoTarea::Pendiente,
///     prioridad: 2,
///     categoria: Some("rust".to_string()),
///     creado_en: "2026-11-07".to_string(),
/// };
///
/// assert_eq!(tarea.id, 1);
/// assert_eq!(tarea.categoria.as_deref(), Some("rust"));
/// ```
#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct Tarea {
    /// Identificador único de la tarea.
    pub id: u64,
    /// Texto que describe la tarea.
    pub descripcion: String,
    /// Estado actual de la tarea.
    pub estado: EstadoTarea,
    /// Nivel de prioridad de la tarea.
    pub prioridad: u8,
    /// Categoría opcional, por ejemplo `rust` o `universidad`.
    pub categoria: Option<String>,
    /// Fecha o marca de creación de la tarea.
    pub creado_en: String,
}

impl Tarea {
    /// Crea una tarea nueva como pendiente.
    ///
    /// # Examples
    ///
    /// ```
    /// use studyflowcli::models::{EstadoTarea, Tarea};
    ///
    /// let categoria: Option<String> = None;
    /// let tarea = Tarea::nueva_tarea(
    ///     "Repasar ownership",
    ///     &EstadoTarea::Pendiente,
    ///     &3,
    ///     &categoria,
    ///     "2026-11-07",
    /// )
    /// .unwrap();
    ///
    /// assert_eq!(tarea.descripcion, "Repasar ownership");
    /// assert_eq!(tarea.prioridad, 3);
    /// assert!(matches!(tarea.estado, EstadoTarea::Pendiente));
    /// ```
    pub fn nueva_tarea(
        descripcion: &str,
        estado: &EstadoTarea,
        prioridad: &u8,
        categoria: &Option<String>,
        creado_en: &str,
    ) -> AppResult<Self> {
        Ok(Tarea {
            id: generate_id()?,
            descripcion: descripcion.to_owned(),
            estado: estado.clone(),
            prioridad: *prioridad,
            categoria: categoria.clone(),
            creado_en: creado_en.to_owned(),
        })
    }
}

/// Opciones tipadas recibidas por la CLI.
///
/// # Examples
///
/// ```
/// use studyflowcli::models::{CliOptions, EstadoTarea};
///
/// let options = CliOptions {
///     priority: 4,
///     priority_found: true,
///     status: Some(EstadoTarea::Pendiente),
/// };
///
/// assert_eq!(options.priority, 4);
/// assert_eq!(options.status, Some(EstadoTarea::Pendiente));
/// ```
#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct CliOptions {
    /// Prioridad solicitada; vale `3` cuando no se indica.
    pub priority: u8,
    /// Indica si la prioridad se especificó con `--priority`.
    pub priority_found: bool,
    /// Estado por el que filtrar `list`, si se indicó con `--status`.
    pub status: Option<EstadoTarea>,
}

/// Argumentos ya parseados de un comando.
///
/// # Examples
///
/// ```
/// use studyflowcli::models::{CliArgs, CliOptions, Subcommand};
///
/// let args = CliArgs {
///     subcommand: Subcommand::Add,
///     options: vec!["Repasar Rust".to_string()],
///     structured_options: CliOptions {
///         priority: 2,
///         priority_found: true,
///         status: None,
///     },
/// };
///
/// assert_eq!(args.options.len(), 1);
/// assert_eq!(args.structured_options.priority, 2);
/// ```
pub struct CliArgs {
    /// Subcomando que se debe ejecutar.
    pub subcommand: Subcommand,
    /// Argumentos posicionales del comando.
    pub options: Vec<String>,
    /// Opciones ya convertidas a tipos de dominio.
    pub structured_options: CliOptions,
}

/// Resumen calculado de las tareas almacenadas.
///
/// # Examples
///
/// ```
/// use studyflowcli::models::StatusReport;
///
/// let report = StatusReport {
///     total: 2,
///     pending: 1,
///     completed: 1,
///     percentage: 50.0,
/// };
///
/// assert_eq!(report.total, 2);
/// assert_eq!(report.percentage, 50.0);
/// ```
pub struct StatusReport {
    /// Cantidad total de tareas.
    pub total: usize,
    /// Cantidad de tareas pendientes.
    pub pending: usize,
    /// Cantidad de tareas completadas.
    pub completed: usize,
    /// Porcentaje de tareas completadas.
    pub percentage: f64,
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn generate_id_produce_valores_distintos() {
        let ids: HashSet<u64> = (0..100).map(|_| generate_id().unwrap()).collect();

        assert_eq!(ids.len(), 100);
    }

    #[test]
    fn nueva_tarea_copia_los_datos_recibidos() {
        let categoria = Some("rust".to_string());

        let tarea = Tarea::nueva_tarea(
            "Repasar ownership",
            &EstadoTarea::Pendiente,
            &4,
            &categoria,
            "2026-11-07",
        )
        .unwrap();

        assert_eq!(tarea.descripcion, "Repasar ownership");
        assert!(matches!(tarea.estado, EstadoTarea::Pendiente));
        assert_eq!(tarea.prioridad, 4);
        assert_eq!(tarea.categoria.as_deref(), Some("rust"));
        assert_eq!(tarea.creado_en, "2026-11-07");
    }
}
