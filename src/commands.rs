//! Implementación de la lógica de cada comando.

use std::fs;

use crate::{
    cli::print_cli_help,
    errors::{AppError, AppResult},
    models::{CliArgs, EstadoTarea, Tarea},
    reports::stats_report,
    storage::{create_new_todo_file, file_exists, read_todo_file},
};

/// Deserializa las tareas almacenadas en una cadena JSON.
///
/// # Examples
///
/// ```
/// use studyflowcli::commands::deserialize_str_json_data;
///
/// let json = r#"[{"id":1,"descripcion":"Repasar ownership","estado":"pendiente","prioridad":3,"categoria":null,"creado_en":"2026-11-07"}]"#;
/// let tareas = deserialize_str_json_data(json).unwrap();
///
/// assert_eq!(tareas.len(), 1);
/// assert_eq!(tareas[0].descripcion, "Repasar ownership");
/// ```
pub fn deserialize_str_json_data(json_str: &str) -> AppResult<Vec<Tarea>> {
    Ok(serde_json::from_str(json_str)?)
}

/// Filtra las tareas que todavía no fueron completadas.
///
/// # Examples
///
/// ```
/// use studyflowcli::commands::filter_active_todos;
/// use studyflowcli::models::{EstadoTarea, Tarea};
///
/// let todos = vec![
///     Tarea {
///         id: 1,
///         descripcion: "pendiente".to_string(),
///         estado: EstadoTarea::Pendiente,
///         prioridad: 3,
///         categoria: None,
///         creado_en: "hoy".to_string(),
///     },
///     Tarea {
///         id: 2,
///         descripcion: "completada".to_string(),
///         estado: EstadoTarea::Completada,
///         prioridad: 3,
///         categoria: None,
///         creado_en: "hoy".to_string(),
///     },
/// ];
///
/// let activos = filter_active_todos(todos);
///
/// assert_eq!(activos.len(), 1);
/// assert_eq!(activos[0].id, 1);
/// ```
pub fn filter_active_todos(todos: Vec<Tarea>) -> Vec<Tarea> {
    todos
        .into_iter()
        .filter(|tarea| !matches!(tarea.estado, EstadoTarea::Completada))
        .collect()
}

/// Lee las tareas del archivo y, opcionalmente, descarta las completadas.
///
/// # Errors
///
/// Devuelve [`AppError::ArchivoNoEncontrado`] si el archivo no existe y
/// [`AppError::Json`] si su contenido no es JSON válido.
///
/// # Examples
///
/// ```
/// use std::fs;
/// use studyflowcli::commands::get_todos;
/// use studyflowcli::models::{EstadoTarea, Tarea};
///
/// let ruta = std::env::temp_dir().join(format!(
///     "studyflow_doc_get_todos_{}.json",
///     std::process::id()
/// ));
/// let nombre = ruta.to_str().unwrap();
///
/// let todos = vec![
///     Tarea {
///         id: 1,
///         descripcion: "activa".to_string(),
///         estado: EstadoTarea::Pendiente,
///         prioridad: 3,
///         categoria: None,
///         creado_en: "hoy".to_string(),
///     },
///     Tarea {
///         id: 2,
///         descripcion: "hecha".to_string(),
///         estado: EstadoTarea::Completada,
///         prioridad: 3,
///         categoria: None,
///         creado_en: "hoy".to_string(),
///     },
/// ];
///
/// fs::write(nombre, serde_json::to_string(&todos).unwrap()).unwrap();
///
/// assert_eq!(get_todos(nombre, false).unwrap().len(), 2);
/// assert_eq!(get_todos(nombre, true).unwrap().len(), 1);
///
/// fs::remove_file(nombre).unwrap();
/// ```
pub fn get_todos(file_name: &str, just_active: bool) -> AppResult<Vec<Tarea>> {
    if !file_exists(file_name) {
        return Err(AppError::ArchivoNoEncontrado(file_name.to_string()));
    }

    let contents = read_todo_file(file_name)?;
    let todos = deserialize_str_json_data(&contents)?;

    Ok(if just_active {
        filter_active_todos(todos)
    } else {
        todos
    })
}

/// Busca la posición de una tarea por su ID.
///
/// Devuelve `None` si no existe ninguna tarea con ese ID.
///
/// # Examples
///
/// ```
/// use studyflowcli::commands::get_todo_by_id;
/// use studyflowcli::models::{EstadoTarea, Tarea};
///
/// let todos = vec![Tarea {
///     id: 7,
///     descripcion: "Repasar Rust".to_string(),
///     estado: EstadoTarea::Pendiente,
///     prioridad: 2,
///     categoria: None,
///     creado_en: "hoy".to_string(),
/// }];
///
/// assert_eq!(get_todo_by_id(&todos, 7), Some(0));
/// assert_eq!(get_todo_by_id(&todos, 99), None);
/// ```
pub fn get_todo_by_id(todos: &[Tarea], id: u64) -> Option<usize> {
    todos.iter().position(|tarea| tarea.id == id)
}

/// Muestra la ayuda de la aplicación.
///
/// # Examples
///
/// ```
/// assert!(studyflowcli::commands::print_help().is_ok());
/// ```
pub fn print_help() -> AppResult<()> {
    print_cli_help();
    Ok(())
}

/// Inicializa un archivo de datos vacío.
///
/// # Errors
///
/// Devuelve [`AppError::ArchivoYaExiste`] si el archivo ya existe.
///
/// # Examples
///
/// ```
/// use std::fs;
/// use studyflowcli::commands::create_new_todo;
///
/// let ruta = std::env::temp_dir().join(format!(
///     "studyflow_doc_init_{}.json",
///     std::process::id()
/// ));
/// let nombre = ruta.to_str().unwrap();
/// let _ = fs::remove_file(nombre);
///
/// create_new_todo(nombre).unwrap();
/// assert_eq!(fs::read_to_string(nombre).unwrap(), "[]\n");
///
/// fs::remove_file(nombre).unwrap();
/// ```
pub fn create_new_todo(file_name: &str) -> AppResult<()> {
    create_new_todo_file(file_name)?;
    println!("Nuevo proyecto inicializado en '{file_name}'.");
    Ok(())
}

/// Imprime todas las tareas almacenadas.
///
/// # Errors
///
/// Devuelve [`AppError::ArchivoNoEncontrado`] si el archivo no existe y
/// [`AppError::Json`] si su contenido no es JSON válido.
///
/// # Examples
///
/// ```
/// use std::fs;
/// use studyflowcli::commands::list_todos;
/// use studyflowcli::models::{EstadoTarea, Tarea};
///
/// let ruta = std::env::temp_dir().join(format!(
///     "studyflow_doc_list_{}.json",
///     std::process::id()
/// ));
/// let nombre = ruta.to_str().unwrap();
///
/// let todos = vec![Tarea {
///     id: 1,
///     descripcion: "Repasar ownership".to_string(),
///     estado: EstadoTarea::Pendiente,
///     prioridad: 3,
///     categoria: None,
///     creado_en: "hoy".to_string(),
/// }];
///
/// fs::write(nombre, serde_json::to_string(&todos).unwrap()).unwrap();
/// assert!(list_todos(nombre).is_ok());
///
/// fs::remove_file(nombre).unwrap();
/// assert!(list_todos(nombre).is_err());
/// ```
pub fn list_todos(file_name: &str) -> AppResult<()> {
    let todos = get_todos(file_name, false)?;

    if todos.is_empty() {
        println!("No hay tareas en el proyecto.");
        return Ok(());
    }

    for tarea in &todos {
        println!(
            "tarea {}: {} (Estado: {:?}, Prioridad: {})",
            tarea.id, tarea.descripcion, tarea.estado, tarea.prioridad
        );
    }

    Ok(())
}

/// Agrega una tarea nueva al archivo de datos.
///
/// La descripción se toma del primer argumento posicional y la prioridad de
/// [`CliOptions`](crate::models::CliOptions).
///
/// # Errors
///
/// Devuelve [`AppError::ArgumentoFaltante`] si no hay descripción,
/// [`AppError::DescripcionVacia`] si solo contiene espacios y los errores de
/// lectura o escritura del archivo.
///
/// # Examples
///
/// ```
/// use std::fs;
/// use studyflowcli::commands::{add_todo, create_new_todo};
/// use studyflowcli::models::{CliArgs, CliOptions, Subcommand};
///
/// let ruta = std::env::temp_dir().join(format!(
///     "studyflow_doc_add_{}.json",
///     std::process::id()
/// ));
/// let nombre = ruta.to_str().unwrap();
/// let _ = fs::remove_file(nombre);
/// create_new_todo(nombre).unwrap();
///
/// let args = CliArgs {
///     subcommand: Subcommand::Add,
///     options: vec!["Repasar ownership".to_string()],
///     structured_options: CliOptions { priority: 2 },
/// };
///
/// add_todo(nombre, &args).unwrap();
///
/// let contenido = fs::read_to_string(nombre).unwrap();
/// assert!(contenido.contains("Repasar ownership"));
/// assert!(contenido.contains("\"prioridad\":2"));
///
/// fs::remove_file(nombre).unwrap();
/// ```
pub fn add_todo(file_name: &str, args: &CliArgs) -> AppResult<()> {
    let raw_description = args.options.first().ok_or(AppError::ArgumentoFaltante {
        comando: "add",
        uso: "studyflowcli add \"descripción\" [--priority N]",
    })?;

    let description = raw_description.trim();
    if description.is_empty() {
        return Err(AppError::DescripcionVacia);
    }

    let mut todos = get_todos(file_name, false)?;
    let descripcion = description.to_string();
    let categoria: Option<String> = None;
    let creado_en = "hoy".to_string();

    let new_todo = Tarea::nueva_tarea(
        &descripcion,
        &EstadoTarea::Pendiente,
        &args.structured_options.priority,
        &categoria,
        &creado_en,
    )?;

    todos.push(new_todo);
    fs::write(file_name, serde_json::to_string(&todos)?)?;

    println!("Tarea agregada exitosamente.");
    Ok(())
}

fn parse_task_id(args: &CliArgs, comando: &'static str, uso: &'static str) -> AppResult<u64> {
    let raw_id = args
        .options
        .first()
        .ok_or(AppError::ArgumentoFaltante { comando, uso })?;

    raw_id
        .parse::<u64>()
        .map_err(|_| AppError::IdInvalido(raw_id.clone()))
}

/// Marca una tarea como completada conservando las demás.
///
/// # Errors
///
/// Devuelve [`AppError::ArgumentoFaltante`] o [`AppError::IdInvalido`] si el
/// ID no es válido, y [`AppError::TareaNoEncontrada`] si no existe.
///
/// # Examples
///
/// ```
/// use std::fs;
/// use studyflowcli::commands::complete_todo;
/// use studyflowcli::models::{CliArgs, CliOptions, EstadoTarea, Subcommand, Tarea};
///
/// let ruta = std::env::temp_dir().join(format!(
///     "studyflow_doc_complete_{}.json",
///     std::process::id()
/// ));
/// let nombre = ruta.to_str().unwrap();
///
/// let todos = vec![Tarea {
///     id: 7,
///     descripcion: "Repasar ownership".to_string(),
///     estado: EstadoTarea::Pendiente,
///     prioridad: 3,
///     categoria: None,
///     creado_en: "hoy".to_string(),
/// }];
/// fs::write(nombre, serde_json::to_string(&todos).unwrap()).unwrap();
///
/// let args = CliArgs {
///     subcommand: Subcommand::Complete,
///     options: vec!["7".to_string()],
///     structured_options: CliOptions { priority: 3 },
/// };
///
/// complete_todo(nombre, &args).unwrap();
///
/// let guardadas: Vec<Tarea> =
///     serde_json::from_str(&fs::read_to_string(nombre).unwrap()).unwrap();
/// assert!(matches!(guardadas[0].estado, EstadoTarea::Completada));
///
/// fs::remove_file(nombre).unwrap();
/// ```
pub fn complete_todo(file_name: &str, args: &CliArgs) -> AppResult<()> {
    let id = parse_task_id(args, "complete", "studyflowcli complete <id>")?;
    let mut todos = get_todos(file_name, false)?;
    let index = get_todo_by_id(&todos, id).ok_or(AppError::TareaNoEncontrada(id))?;

    todos[index].estado = EstadoTarea::Completada;
    fs::write(file_name, serde_json::to_string(&todos)?)?;

    println!("Tarea con ID {id} marcada como completada.");
    Ok(())
}

/// Elimina una tarea por su ID.
///
/// # Errors
///
/// Devuelve [`AppError::ArgumentoFaltante`] o [`AppError::IdInvalido`] si el
/// ID no es válido, y [`AppError::TareaNoEncontrada`] si no existe.
///
/// # Examples
///
/// ```
/// use std::fs;
/// use studyflowcli::commands::delete_todo;
/// use studyflowcli::models::{CliArgs, CliOptions, EstadoTarea, Subcommand, Tarea};
///
/// let ruta = std::env::temp_dir().join(format!(
///     "studyflow_doc_delete_{}.json",
///     std::process::id()
/// ));
/// let nombre = ruta.to_str().unwrap();
///
/// let todos = vec![Tarea {
///     id: 9,
///     descripcion: "Tarea temporal".to_string(),
///     estado: EstadoTarea::Pendiente,
///     prioridad: 3,
///     categoria: None,
///     creado_en: "hoy".to_string(),
/// }];
/// fs::write(nombre, serde_json::to_string(&todos).unwrap()).unwrap();
///
/// let args = CliArgs {
///     subcommand: Subcommand::Delete,
///     options: vec!["9".to_string()],
///     structured_options: CliOptions { priority: 3 },
/// };
///
/// delete_todo(nombre, &args).unwrap();
///
/// let guardadas: Vec<Tarea> =
///     serde_json::from_str(&fs::read_to_string(nombre).unwrap()).unwrap();
/// assert!(guardadas.is_empty());
///
/// fs::remove_file(nombre).unwrap();
/// ```
pub fn delete_todo(file_name: &str, args: &CliArgs) -> AppResult<()> {
    let id = parse_task_id(args, "delete", "studyflowcli delete <id>")?;
    let mut todos = get_todos(file_name, false)?;
    let index = get_todo_by_id(&todos, id).ok_or(AppError::TareaNoEncontrada(id))?;

    todos.remove(index);
    fs::write(file_name, serde_json::to_string(&todos)?)?;

    println!("Tarea con ID {id} eliminada exitosamente.");
    Ok(())
}

/// Busca tareas por descripción, categoría o prioridad.
///
/// Devuelve las tareas que coinciden e imprime un aviso cuando no hay
/// resultados.
///
/// # Errors
///
/// Devuelve [`AppError::ArgumentoFaltante`] si no hay texto de búsqueda,
/// [`AppError::BusquedaVacia`] si solo contiene espacios y los errores de
/// lectura del archivo.
///
/// # Examples
///
/// ```
/// use std::fs;
/// use studyflowcli::commands::search_todos;
/// use studyflowcli::models::{CliArgs, CliOptions, EstadoTarea, Subcommand, Tarea};
///
/// let ruta = std::env::temp_dir().join(format!(
///     "studyflow_doc_search_{}.json",
///     std::process::id()
/// ));
/// let nombre = ruta.to_str().unwrap();
///
/// let todos = vec![
///     Tarea {
///         id: 1,
///         descripcion: "Repasar Rust".to_string(),
///         estado: EstadoTarea::Pendiente,
///         prioridad: 2,
///         categoria: Some("rust".to_string()),
///         creado_en: "hoy".to_string(),
///     },
///     Tarea {
///         id: 2,
///         descripcion: "Comprar café".to_string(),
///         estado: EstadoTarea::Pendiente,
///         prioridad: 1,
///         categoria: None,
///         creado_en: "hoy".to_string(),
///     },
/// ];
/// fs::write(nombre, serde_json::to_string(&todos).unwrap()).unwrap();
///
/// let args = CliArgs {
///     subcommand: Subcommand::Search,
///     options: vec!["rust".to_string()],
///     structured_options: CliOptions { priority: 3 },
/// };
///
/// let resultados = search_todos(nombre, &args).unwrap();
///
/// assert_eq!(resultados.len(), 1);
/// assert_eq!(resultados[0].descripcion, "Repasar Rust");
///
/// fs::remove_file(nombre).unwrap();
/// ```
pub fn search_todos(file_name: &str, args: &CliArgs) -> AppResult<Vec<Tarea>> {
    let raw_keyword = args.options.first().ok_or(AppError::ArgumentoFaltante {
        comando: "search",
        uso: "studyflowcli search <texto>",
    })?;

    let keyword = raw_keyword.trim().to_lowercase();
    if keyword.is_empty() {
        return Err(AppError::BusquedaVacia);
    }

    let todos = get_todos(file_name, false)?;
    let results: Vec<Tarea> = todos
        .into_iter()
        .filter(|tarea| {
            tarea.descripcion.to_lowercase().contains(&keyword)
                || tarea
                    .categoria
                    .as_ref()
                    .is_some_and(|categoria| categoria.to_lowercase().contains(&keyword))
                || tarea.prioridad.to_string() == keyword
        })
        .collect();

    if results.is_empty() {
        println!("No se encontraron tareas que coincidan con la búsqueda.");
    } else {
        for tarea in &results {
            println!(
                "tarea {}: {} (Estado: {:?}, Prioridad: {})",
                tarea.id, tarea.descripcion, tarea.estado, tarea.prioridad
            );
        }
    }

    Ok(results)
}

/// Imprime el resumen de tareas completadas, pendientes y el avance.
///
/// # Errors
///
/// Devuelve los errores de lectura del archivo de datos.
///
/// # Examples
///
/// ```
/// use std::fs;
/// use studyflowcli::commands::get_stats;
/// use studyflowcli::models::{EstadoTarea, Tarea};
///
/// let ruta = std::env::temp_dir().join(format!(
///     "studyflow_doc_stats_{}.json",
///     std::process::id()
/// ));
/// let nombre = ruta.to_str().unwrap();
///
/// let todos = vec![
///     Tarea {
///         id: 1,
///         descripcion: "hecha".to_string(),
///         estado: EstadoTarea::Completada,
///         prioridad: 3,
///         categoria: None,
///         creado_en: "hoy".to_string(),
///     },
///     Tarea {
///         id: 2,
///         descripcion: "pendiente".to_string(),
///         estado: EstadoTarea::Pendiente,
///         prioridad: 3,
///         categoria: None,
///         creado_en: "hoy".to_string(),
///     },
/// ];
/// fs::write(nombre, serde_json::to_string(&todos).unwrap()).unwrap();
///
/// assert!(get_stats(nombre).is_ok());
///
/// fs::remove_file(nombre).unwrap();
/// ```
pub fn get_stats(file_name: &str) -> AppResult<()> {
    let todos = get_todos(file_name, false)?;

    let report = stats_report(&todos);

    println!("Estadísticas de tareas:");
    println!("Total de tareas: {}", report.total);
    println!("Tareas completadas: {}", report.completed);
    println!("Tareas pendientes: {}", report.pending);
    println!("Avance: {:.1}%", report.percentage);

    Ok(())
}
