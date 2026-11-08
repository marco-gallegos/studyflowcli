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

#[cfg(test)]
mod tests {
    use super::*;

    fn tarea(id: u64, estado: EstadoTarea) -> Tarea {
        Tarea {
            id,
            descripcion: format!("tarea {id}"),
            estado,
            prioridad: 3,
            categoria: None,
            creado_en: "2026-11-07".to_string(),
        }
    }

    #[test]
    fn reporte_vacio_no_divide_por_cero() {
        let report = stats_report(&[]);

        assert_eq!(report.total, 0);
        assert_eq!(report.completed, 0);
        assert_eq!(report.pending, 0);
        assert_eq!(report.percentage, 0.0);
    }

    #[test]
    fn reporte_mixto_calcula_porcentaje() {
        let todos = vec![
            tarea(1, EstadoTarea::Completada),
            tarea(2, EstadoTarea::Completada),
            tarea(3, EstadoTarea::Pendiente),
        ];

        let report = stats_report(&todos);

        assert_eq!(report.total, 3);
        assert_eq!(report.completed, 2);
        assert_eq!(report.pending, 1);
        assert!((report.percentage - 66.666_666).abs() < 0.001);
    }

    #[test]
    fn reporte_con_todo_completado_llega_al_cien_por_ciento() {
        let todos = vec![
            tarea(1, EstadoTarea::Completada),
            tarea(2, EstadoTarea::Completada),
        ];

        let report = stats_report(&todos);

        assert_eq!(report.completed, 2);
        assert_eq!(report.pending, 0);
        assert_eq!(report.percentage, 100.0);
    }

    #[test]
    fn reporte_soporta_mas_de_255_tareas() {
        let mut todos = Vec::with_capacity(300);
        for id in 0..300 {
            let estado = if id < 260 {
                EstadoTarea::Completada
            } else {
                EstadoTarea::Pendiente
            };
            todos.push(tarea(id, estado));
        }

        let report = stats_report(&todos);

        assert_eq!(report.total, 300);
        assert_eq!(report.completed, 260);
        assert_eq!(report.pending, 40);
        assert!((report.percentage - 86.666_666).abs() < 0.001);
    }
}
