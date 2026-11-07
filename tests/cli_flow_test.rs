//! Prueba de integración del flujo principal de la CLI.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

static COUNTER: AtomicU64 = AtomicU64::new(0);

/// Directorio temporal único que se elimina al terminar la prueba.
struct TempDir {
    path: PathBuf,
}

impl TempDir {
    fn new(nombre: &str) -> Self {
        let sequence = COUNTER.fetch_add(1, Ordering::Relaxed);
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let path = std::env::temp_dir().join(format!(
            "studyflow_it_{nombre}_{}_{}_{}",
            std::process::id(),
            sequence,
            nanos
        ));

        fs::create_dir_all(&path).unwrap();
        Self { path }
    }

    fn data_file(&self) -> PathBuf {
        self.path.join("tasks.json")
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.path);
    }
}

fn run(data_file: &Path, argumentos: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_studyflowcli"))
        .args(argumentos)
        .env("STUDYFLOW_DATA_PATH", data_file)
        .output()
        .expect("no se pudo ejecutar studyflowcli")
}

fn stdout(output: &Output) -> String {
    String::from_utf8_lossy(&output.stdout).into_owned()
}

fn stderr(output: &Output) -> String {
    String::from_utf8_lossy(&output.stderr).into_owned()
}

fn tareas(data_file: &Path) -> Vec<serde_json::Value> {
    serde_json::from_str(&fs::read_to_string(data_file).unwrap()).unwrap()
}

#[test]
fn flujo_principal_persiste_entre_ejecuciones() {
    let dir = TempDir::new("flujo");
    let data = dir.data_file();

    let output = run(&data, &["init"]);
    assert!(output.status.success(), "init falló: {}", stderr(&output));
    assert!(data.exists());

    let output = run(&data, &["add", "Repasar ownership", "--priority", "5"]);
    assert!(output.status.success(), "add falló: {}", stderr(&output));

    let output = run(&data, &["add", "Comprar café"]);
    assert!(output.status.success(), "add falló: {}", stderr(&output));

    let guardadas = tareas(&data);
    assert_eq!(guardadas.len(), 2);
    assert_eq!(guardadas[0]["descripcion"], "Repasar ownership");
    assert_eq!(guardadas[0]["prioridad"], 5);
    assert_eq!(guardadas[1]["descripcion"], "Comprar café");

    let id = guardadas[0]["id"].as_u64().unwrap().to_string();

    let output = run(&data, &["list"]);
    assert!(output.status.success(), "list falló: {}", stderr(&output));
    let salida = stdout(&output);
    assert!(salida.contains("Repasar ownership"));
    assert!(salida.contains("Comprar café"));

    let output = run(&data, &["complete", &id]);
    assert!(
        output.status.success(),
        "complete falló: {}",
        stderr(&output)
    );

    let guardadas = tareas(&data);
    assert_eq!(guardadas[0]["estado"], "completada");
    assert_eq!(guardadas[1]["estado"], "pendiente");

    let output = run(&data, &["stats"]);
    assert!(output.status.success(), "stats falló: {}", stderr(&output));
    let salida = stdout(&output);
    assert!(salida.contains("Total de tareas: 2"));
    assert!(salida.contains("Tareas completadas: 1"));
    assert!(salida.contains("Tareas pendientes: 1"));
    assert!(salida.contains("Avance: 50.0%"));

    let output = run(&data, &["search", "ownership"]);
    assert!(output.status.success(), "search falló: {}", stderr(&output));
    let salida = stdout(&output);
    assert!(salida.contains("Repasar ownership"));
    assert!(!salida.contains("Comprar café"));

    let output = run(&data, &["delete", &id]);
    assert!(output.status.success(), "delete falló: {}", stderr(&output));

    let guardadas = tareas(&data);
    assert_eq!(guardadas.len(), 1);
    assert_eq!(guardadas[0]["descripcion"], "Comprar café");
}

#[test]
fn los_errores_esperados_terminan_con_codigo_uno() {
    let dir = TempDir::new("errores");
    let data = dir.data_file();

    let output = run(&data, &["list"]);
    assert_eq!(output.status.code(), Some(1));
    assert!(stderr(&output).contains("No se encontró el archivo"));

    let output = run(&data, &["comando-invalido"]);
    assert_eq!(output.status.code(), Some(1));
    assert!(stderr(&output).contains("Comando desconocido"));

    assert!(run(&data, &["init"]).status.success());

    let output = run(&data, &["add", "   "]);
    assert_eq!(output.status.code(), Some(1));
    assert!(stderr(&output).contains("La descripción no puede estar vacía"));

    let output = run(&data, &["complete", "999"]);
    assert_eq!(output.status.code(), Some(1));
    assert!(stderr(&output).contains("No existe una tarea con ID 999"));

    fs::write(&data, "no es json").unwrap();
    let output = run(&data, &["list"]);
    assert_eq!(output.status.code(), Some(1));
    assert!(stderr(&output).contains("JSON válido"));
    assert_eq!(fs::read_to_string(&data).unwrap(), "no es json");
}
