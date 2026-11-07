//! Núcleo de StudyFlow CLI.
//!
//! StudyFlow CLI es una herramienta de línea de comandos para registrar y
//! consultar tareas de estudio. La lógica vive en esta librería y `main.rs`
//! solo coordina la entrada, la salida y los códigos de error.
//!
//! # Ejemplo
//!
//! ```
//! use studyflowcli::models::{EstadoTarea, Tarea};
//!
//! let tarea = Tarea {
//!     id: 1,
//!     descripcion: "Repasar ownership".to_string(),
//!     estado: EstadoTarea::Pendiente,
//!     prioridad: 3,
//!     categoria: None,
//!     creado_en: "2026-11-07".to_string(),
//! };
//!
//! assert_eq!(tarea.descripcion, "Repasar ownership");
//! assert!(matches!(tarea.estado, EstadoTarea::Pendiente));
//! ```

pub mod cli;
pub mod commands;
pub mod config;
pub mod errors;
pub mod models;
pub mod reports;
pub mod storage;
