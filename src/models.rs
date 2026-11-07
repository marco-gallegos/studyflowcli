//! Implementación de los modelos de datos utilizados en la aplicación.



pub fn generate_id() -> u32 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let start = SystemTime::now();
    let since_epoch = start.duration_since(UNIX_EPOCH).expect("Error al obtener la hora actual");
    since_epoch.as_secs() as u32
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

#[derive(serde::Deserialize, serde::Serialize, Debug, std::clone::Clone)]
#[serde(rename_all = "snake_case")]
pub enum EstadoTarea {
    Pendiente,
    Completada,
}


#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct Tarea {
    pub id: u32,
    pub descripcion: String,
    pub estado: EstadoTarea,
    pub prioridad: u8,
    pub categoria: Option<String>,
    pub creado_en: String,
}

impl Tarea {
    pub fn nueva_tarea(descripcion: &String, estado: &EstadoTarea, prioridad: &u8, categoria: &Option<String>, creado_en: &String) -> Self {
        Tarea {
            id: generate_id(), // Se asignará un ID único al agregar la tarea al proyecto
            descripcion: descripcion.clone(),
            estado: estado.clone(),
            prioridad: *prioridad,
            categoria: categoria.clone(),
            creado_en: creado_en.clone(),
        }
    }
}


#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct CliOptions {
    pub priority: u8,
    pub status: Option<String>,
}



pub struct CliArgs {
    pub subcommand: Subcommand,
    pub options: Vec<String>,
    pub structured_options: CliOptions,
}