# StudyFlow CLI

Herramienta de línea de comandos escrita en Rust para registrar, consultar y
dar seguimiento a tareas de estudio. Los datos se guardan en un archivo JSON
local, por lo que persisten entre ejecuciones.

Proyecto final integrador del Bloque 13 del curso de Rust.

## Requisitos

- Rust y Cargo con soporte para edición 2024 (`rustc 1.85` o superior).
- No requiere servicios externos ni base de datos.

## Instalación

Clona el repositorio y compila el proyecto:

```bash
git clone <url-del-repositorio>
cd studyflowcli
cargo build
```

El binario queda en `target/debug/studyflowcli`. También puedes ejecutarlo
directamente con `cargo run -- <argumentos>` desde la raíz del proyecto.

Para instalarlo en el entorno local:

```bash
cargo install --path .
```

## Uso

```text
studyflowcli <subcomando> [opciones]
```

### Comandos

| Comando | Descripción | Ejemplo |
| --- | --- | --- |
| `help` | Muestra la ayuda. | `cargo run -- help` |
| `init` | Crea el archivo de datos vacío. | `cargo run -- init` |
| `add` | Agrega una tarea. | `cargo run -- add "Repasar ownership" --priority 5` |
| `list` | Lista todas las tareas. | `cargo run -- list` |
| `complete` | Marca una tarea como completada. | `cargo run -- complete 123` |
| `delete` | Elimina una tarea. | `cargo run -- delete 123` |
| `search` | Busca por descripción, categoría o prioridad. | `cargo run -- search rust` |
| `stats` | Muestra totales y porcentaje de avance. | `cargo run -- stats` |

`add` acepta `--priority N` antes o después de la descripción. Si no se indica,
la prioridad por defecto es `3`. El ID que necesitan `complete` y `delete` se
muestra con `list`.

### Ejemplo de flujo completo

```bash
cargo run -- init
cargo run -- add "Repasar ownership" --priority 5
cargo run -- add "Comprar café"
cargo run -- list
# tarea 1794090597441970655: Repasar ownership (Estado: Pendiente, Prioridad: 5)
# tarea 1794090597441980000: Comprar café (Estado: Pendiente, Prioridad: 3)

cargo run -- complete 1794090597441970655
cargo run -- search ownership
cargo run -- stats
# Total de tareas: 2
# Tareas completadas: 1
# Tareas pendientes: 1
# Avance: 50.0%

cargo run -- delete 1794090597441970655
```

### Códigos de salida

- `0`: la operación terminó correctamente.
- `1`: ocurrió un error esperado (archivo inexistente, ID inválido, tarea no
  encontrada, JSON inválido, argumentos faltantes, etc.).

Los errores se escriben en `stderr` con mensajes pensados para la persona
usuaria; la aplicación no usa `panic!` para esos casos.

## Formato de datos y configuración

La ruta del archivo se toma de la variable de entorno `STUDYFLOW_DATA_PATH`.
Si no está definida o está vacía, se usa `studyflow_project.json` en el
directorio actual.

```bash
STUDYFLOW_DATA_PATH=./datos_prueba.json cargo run -- init
STUDYFLOW_DATA_PATH=./datos_prueba.json cargo run -- list
```

El archivo contiene un arreglo JSON de tareas:

```json
[
  {
    "id": 1794090597441970655,
    "descripcion": "Repasar ownership",
    "estado": "pendiente",
    "prioridad": 5,
    "categoria": null,
    "creado_en": "hoy"
  }
]
```

- `id`: número entero sin signo, generado con la marca de tiempo y un contador
  atómico para evitar duplicados.
- `descripcion`: texto de la tarea.
- `estado`: `"pendiente"` o `"completada"`.
- `prioridad`: entero de `0` a `255`; por defecto `3`.
- `categoria`: texto opcional; actualmente queda en `null` al crear tareas.
- `creado_en`: texto informativo; actualmente se guarda como `"hoy"`.

`init` no sobrescribe un archivo existente. Si el JSON está corrupto, los
comandos de lectura devuelven un error y no reemplazan el archivo.

## Estructura del proyecto

```text
src/
├── main.rs       Punto de entrada; traduce resultados a códigos de salida.
├── lib.rs        Declaración de módulos y documentación del crate.
├── cli.rs        Parseo de argumentos y ayuda.
├── commands.rs   Lógica de cada comando.
├── config.rs     Ruta del archivo de datos y variable de entorno.
├── errors.rs     AppError, AppResult y mensajes de error.
├── models.rs     Tarea, estado, opciones y reporte.
├── reports.rs    Cálculo de estadísticas.
├── storage.rs    Lectura y escritura del archivo JSON.
└── test_support.rs  Utilidades para las pruebas unitarias.
tests/
└── cli_flow_test.rs Prueba de integración del flujo completo.
```

## Decisiones técnicas

- **`serde` y `serde_json`** para serializar y deserializar las tareas. El
  formato JSON es legible y permite representar el enum de estado con
  `#[serde(rename_all = "snake_case")]`.
- **Sin `clap`**: el parseo de argumentos se hace con `std`, suficiente para
  los subcomandos y la única opción `--priority`. Esto mantiene el proyecto
  alineado con lo trabajado en el curso.
- **`AppError` y `AppResult`** centralizan los errores recuperables. Se
  implementan `Display`, `std::error::Error`, `From<io::Error>` y
  `From<serde_json::Error>` para poder usar `?`.
- **`main.rs` delgado**: solo interpreta argumentos, delega en la librería y
  convierte el resultado en `ExitCode`. La lógica vive en `commands.rs` y
  módulos de apoyo.
- **IDs únicos** con `SystemTime` más un contador `AtomicU64`, porque usar solo
  los segundos del reloj generaba IDs repetidos al agregar tareas rápido.
- **Escritura completa del archivo**: cada modificación lee todas las tareas,
  cambia la necesaria y vuelve a escribir el arreglo, conservando las tareas
  completadas.
- **Estadísticas con `usize`** para soportar más de 255 tareas sin desbordar.

### Limitaciones conocidas

- No hay bloqueo de archivo ni soporte para varias instancias simultáneas.
- No existe un comando para editar una tarea o cambiar su prioridad.
- `categoria` y `creado_en` son campos provisionales: aún no se reciben desde
  la CLI y `creado_en` no guarda una fecha real.

## Pruebas

El proyecto incluye:

- Pruebas unitarias en cada módulo: mensajes de error, parseo de argumentos,
  almacenamiento, filtros, búsqueda, cálculo de estadísticas y operaciones de
  los comandos con archivos temporales.
- Una prueba de integración en `tests/cli_flow_test.rs` que ejecuta el binario
  completo con `STUDYFLOW_DATA_PATH` apuntando a un directorio temporal y
  recorre `init`, `add`, `list`, `complete`, `search`, `stats` y `delete`,
  además de verificar los códigos de salida de los errores esperados.

Las pruebas usan archivos únicos en el directorio temporal del sistema y los
eliminan al terminar, por lo que nunca modifican `studyflow_project.json`.

Comandos de verificación:

```bash
cargo fmt --all -- --check
cargo check
cargo clippy --all-targets
cargo test
cargo test --doc
cargo doc --no-deps
```

`cargo test` ejecuta las pruebas unitarias, la prueba de integración y los doc
tests de los ejemplos de la API pública.
