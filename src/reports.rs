//! Salida formateada, tablas simples y estadísticas.

use crate::models::{EstadoTarea, StatusReport, Tarea};

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
