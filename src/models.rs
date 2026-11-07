

pub struct Tarea {
    pub id: u32,
    pub descripcion: String,
    pub completada: bool,
    pub prioridad: u8,
}


pub struct Proyecto {
    pub nombre: String,
    pub tareas: Vec<Tarea>,
}