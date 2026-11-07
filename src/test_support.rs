//! Utilidades compartidas por las pruebas unitarias.

use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

static COUNTER: AtomicU64 = AtomicU64::new(0);

/// Archivo temporal único que se elimina automáticamente al salir de ámbito.
pub struct TempFile {
    path: PathBuf,
}

impl TempFile {
    /// Crea una ruta única dentro del directorio temporal del sistema.
    pub fn new(prefix: &str) -> Self {
        let sequence = COUNTER.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!(
            "studyflow_test_{prefix}_{}_{}.json",
            std::process::id(),
            sequence
        ));

        let _ = std::fs::remove_file(&path);
        Self { path }
    }

    /// Devuelve la ruta como texto.
    pub fn path(&self) -> &str {
        self.path.to_str().expect("la ruta temporal debe ser UTF-8")
    }
}

impl Drop for TempFile {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.path);
    }
}
