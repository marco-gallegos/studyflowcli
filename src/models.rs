//! Modelos de datos utilizados por StudyFlow CLI.

use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::errors::{AppError, AppResult};

static NEXT_ID: AtomicU64 = AtomicU64::new(0);

/// Genera un ID único a partir de la marca de tiempo y un contador.
pub fn generate_id() -> AppResult<u64> {
    let start = SystemTime::now();
    let since_epoch = start
        .duration_since(UNIX_EPOCH)
        .map_err(|error| AppError::Reloj(error.to_string()))?;

    let timestamp = since_epoch.as_nanos() as u64;
    let sequence = NEXT_ID.fetch_add(1, Ordering::Relaxed);

    Ok(timestamp.wrapping_add(sequence))
}

pub enum Subcommand {
    Help,
    Init,
    Add,
    List,
    Complete,
    Delete,
    Search,
    Stats,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum EstadoTarea {
    Pendiente,
    Completada,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct Tarea {
    pub id: u64,
    pub descripcion: String,
    pub estado: EstadoTarea,
    pub prioridad: u8,
    pub categoria: Option<String>,
    pub creado_en: String,
}

impl Tarea {
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

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct CliOptions {
    pub priority: u8,
}

pub struct CliArgs {
    pub subcommand: Subcommand,
    pub options: Vec<String>,
    pub structured_options: CliOptions,
}

pub struct StatusReport {
    pub total: u8,
    pub pending: u8,
    pub completed: u8,
    pub percentage: f64,
}
