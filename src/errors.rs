//! Tipos de error de la aplicación y alias de `Result`.
//! 

use core::fmt;
use std::{error::Error};

#[derive(Debug)]
pub enum AppError {
    ListaVacia,
    FileNotExist,
    InvalidJsonFile,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::FileNotExist => write!(f, "el archivo de tareas no existe"),
            AppError::ListaVacia => write!(f, "Lista vacia."),
            AppError::InvalidJsonFile => write!(f,"invalid json file content")
        }
    }
}

impl Error for AppError {}
