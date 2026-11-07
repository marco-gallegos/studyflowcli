use std::time::{SystemTime, UNIX_EPOCH};

pub fn generate_id() -> u32 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let start = SystemTime::now();
    let since_epoch = start.duration_since(UNIX_EPOCH).expect("Error al obtener la hora actual");
    since_epoch.as_secs() as u32
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct Tarea {
    pub id: u32,
    pub descripcion: String,
    pub completada: bool,
    pub prioridad: u8,
}

impl Tarea {
    pub fn nueva_tarea(descripcion: String, completada: bool, prioridad: u8) -> Self {
        Tarea {
            id: generate_id(), // Se asignará un ID único al agregar la tarea al proyecto
            descripcion,
            completada,
            prioridad,
        }
    }
}


pub struct Proyecto {
    pub nombre: String,
    pub tareas: Vec<Tarea>,
}