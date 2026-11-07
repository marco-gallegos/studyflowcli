//! Salida formateada, tablas simples y estadísticas.

use crate::models::{EstadoTarea, StatusReport, Tarea};

/// Calcula el resumen de tareas completadas y pendientes.
///
/// # Examples
///
/// ```
/// use studyflowcli::models::{EstadoTarea, Tarea};
/// use studyflowcli::reports::stats_report;
///
/// fn tarea(id: u64, estado: EstadoTarea) -> Tarea {
///     Tarea {
///         id,
///         descripcion: format!("tarea {id}"),
///         estado,
///         prioridad: 3,
///         categoria: None,
///         creado_en: "2026-11-07".to_string(),
///     }
/// }
///
/// let todos = vec![
///     tarea(1, EstadoTarea::Completada),
///     tarea(2, EstadoTarea::Pendiente),
/// ];
///
/// let report = stats_report(&todos);
///
/// assert_eq!(report.total, 2);
/// assert_eq!(report.completed, 1);
/// assert_eq!(report.pending, 1);
/// assert!((report.percentage - 50.0).abs() < f64::EPSILON);
/// ```
pub fn stats_report(todos: &[Tarea]) -> StatusReport {
    let total = todos.len();
    let completed = todos
        .iter()
        .filter(|tarea| matches!(tarea.estado, EstadoTarea::Completada))
        .count();
    let pending = total - completed;
    let percentage = if total == 0 {
        0.0
    } else {
        completed as f64 * 100.0 / total as f64
    };

    StatusReport {
        total,
        pending,
        completed,
        percentage,
    }
}
