//! Salida formateada, tablas simples y estadísticas.

use crate::models::{EstadoTarea, StatusReport, Tarea};

pub fn stats_report(todos: &Vec<Tarea>) -> StatusReport {
    let total = todos.len() as u8;
    let completed = todos
        .iter()
        .filter(|tarea| matches!(tarea.estado, EstadoTarea::Completada))
        .count() as u8;
    let pending = total - completed;
    let percentage = if total == 0 {
        0.0
    } else {
        completed as f64 * 100.0 / total as f64
    };

    return StatusReport {
        total: total,
        pending: pending,
        completed: completed,
        percentage: percentage,
    };
}
