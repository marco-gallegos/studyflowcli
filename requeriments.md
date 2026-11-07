## Proyecto final integrador Bloque 13

Especificaciones para el proyecto autónomo

Curso de Rust - Opción A: herramienta de línea de comandos (CLI)

Este documento une las secciones 13.1 a 13.5 del temario: definición del proyecto, ruta sugerida, uso de librerías, arquitectura y presentación final. La sección 13.6 se trabaja como cierre del curso por separado.

Nombre sugerido del proyecto: StudyFlow CLI

El estudiante puede cambiar el nombre, siempre que conserve el alcance

técnico definido.


## Contenido del documento

- 1 Propósito del proyecto integrador.

- 2 Problema a resolver y alcance esperado.

- 3 Ruta oficial del curso: Opción A, herramienta CLI.

- 4 Requisitos funcionales y técnicos.

- 5 Arquitectura recomendada del proyecto.

- 6 Uso responsable de crates del ecosistema.

- 7 Persistencia, configuración y manejo de errores.

- 8 Testing, documentación y calidad.

- 9 Ejemplos básicos de apoyo, sin resolver el proyecto final.

- 10 Entrega, presentación final, rúbrica y checklist.

Importante: los ejemplos de código de este PDF son fragmentos aislados para orientar el diseño. No constituyen la solución final del proyecto. El proyecto completo debe ser diseñado e implementado por el estudiante.

## 1. Propósito del proyecto integrador

El proyecto final integrador tiene como objetivo que el estudiante construya una herramienta CLI en Rust que combine los temas centrales del curso: sintaxis, ownership, errores, módulos, crates, entrada/salida, archivos, pruebas, documentación y buenas prácticas de arquitectura.

La intención no es memorizar código, sino demostrar que el estudiante puede diseñar una aplicación pequeña pero completa, tomar decisiones técnicas razonables y explicar su implementación con claridad.

## Resultado esperado

Al terminar, el estudiante deberá entregar una aplicación de línea de comandos funcional. La herramienta sugerida será un gestor de tareas de estudio llamado StudyFlow CLI, aunque el nombre puede cambiar. La aplicación deberá permitir registrar tareas, consultarlas, marcarlas como completadas, buscar información, generar estadísticas básicas y persistir datos en un archivo

local.

| Aspecto | Expectativa |
| --- | --- |
| Tipo de proyecto | Herramienta CLI escrita en Rust. |
| Dominio | Gestión de tareas, pendientes o actividades de estudio. |
| Persistencia | Archivo local en JSON, TOML, CSV o formato propio documentado. |
| Interacción | Argumentos CLI, salida formateada y mensajes de error claros. |
| Arquitectura | Separación entre modelos, comandos, errores, almacenamiento, |
|   | configuración y tests. |
| Calidad | Pruebas, documentación, cargo fmt, cargo clippy y README de uso. |

## 2. Problema a resolver y alcance

Muchos estudiantes gestionan tareas, entregas, prácticas y temas de estudio en notas dispersas. El proyecto propone construir una herramienta CLI que permita guardar y consultar ese avance desde


la terminal. El problema no es solo guardar texto: el reto técnico está en convertir entradas del usuario en comandos, validar datos, persistir información, manejar errores recuperables y mantener el código organizado.

## Planteamiento del problema

Diseñar e implementar una herramienta CLI que permita registrar tareas de estudio y consultar el estado de avance de manera confiable, usando Rust y buenas prácticas de desarrollo.

## Alcance mínimo obligatorio

- Crear un proyecto Cargo con una estructura modular clara.

- Definir modelos de dominio, por ejemplo Tarea, Estado, Prioridad o Categoria.

- Implementar subcomandos para agregar, listar, completar, eliminar, buscar y mostrar estadísticas.

- Guardar los datos en un archivo local y leerlos en ejecuciones posteriores.

- Manejar errores con Result, el operador ? y, preferentemente, un tipo de error propio o una crate adecuada.

- Agregar pruebas unitarias y al menos una prueba de integración.

- Incluir documentación de funciones públicas importantes y un README con instrucciones de uso.

## Fuera de alcance

- No se requiere crear una API web.

- No se requiere usar una base de datos externa.

- No se requiere interfaz gráfica.

- No se espera que el proyecto use unsafe. Si se usa, debe estar plenamente justificado y aislado.

- No se debe copiar una solución completa generada por terceros. Se permite consultar documentación y ejemplos básicos.

## 3. Ruta oficial del curso: Opción A - herramienta CLI

Aunque el temario menciona cuatro rutas sugeridas, para este curso el proyecto final será únicamente la Opción A: herramienta CLI. Esto permite integrar de manera natural los bloques de entrada/salida, archivos, configuración, errores, testing, documentación y arquitectura.

## Comandos obligatorios sugeridos

| Comando | Descripción | Ejemplo de uso |
| --- | --- | --- |
| init | Crea archivo de datos inicial si no existe. | studyflow init |
| add | Agrega una tarea con descripción, prioridad y | studyflow add "Repasar |
|   | categoría opcional. | ownership" --priority 3 |
| list | Lista tareas con filtros opcionales. | studyflow list --status pending |
| complete | Marca una tarea como completada usando su id. | studyflow complete 2 |
| delete | Elimina una tarea por id, idealmente con | studyflow delete 2 |
|   | confirmación o mensaje claro. |   |
| search | Busca tareas por texto, categoría o prioridad. | studyflow search rust |
| stats | Muestra conteos y porcentajes de avance. | studyflow stats |


## Comandos opcionales para nivel avanzado

- edit: modificar descripción, categoría o prioridad de una tarea.

- export: exportar tareas a Markdown o CSV.

- import: importar tareas desde un archivo simple.

- config: mostrar la ruta del archivo de datos o preferencias de la aplicación.

- clean: eliminar o archivar tareas completadas.

## 4. Requisitos funcionales

| ID | Requisito | Criterio de aceptación |
| --- | --- | --- |
| RF-01 | Inicialización | Si el archivo de datos no existe, el programa puede crearlo con un |
|   |   | comando o al primer uso. |
| RF-02 | Agregar tareas | El usuario puede registrar una tarea con descripción no vacía. |
| RF-03 | Listar tareas | El programa muestra tareas con id, estado, prioridad y |
|   |   | descripción. |
| RF-04 | Completar tareas | El usuario puede marcar como completada una tarea existente. |
| RF-05 | Eliminar tareas | El usuario puede eliminar una tarea existente sin corromper el |
|   |   | archivo. |
| RF-06 | Buscar | El usuario puede buscar por texto y obtener coincidencias claras. |
| RF-07 | Estadísticas | El programa calcula totales, pendientes, completadas y porcentaje |
|   |   | de avance. |
| RF-08 | Persistencia | Los datos se conservan entre ejecuciones. |
| RF-09 | Ayuda | La CLI ofrece mensajes de ayuda comprensibles. |

## 5. Requisitos técnicos

## Temas del curso que deben aparecer

| Tema | Cómo debe reflejarse en el proyecto |
| --- | --- |
| Fundamentos y sintaxis | Uso claro de variables, funciones, structs, enums, match e iterators. |
| Ownership y borrowing | Evitar clones innecesarios; usar referencias cuando solo se necesita leer. |
| Errores | Uso de Result, ?, mensajes claros y errores recuperables. |
| Módulos y crates | Separar el código en lib.rs, main.rs y módulos por responsabilidad. |
| CLI e I/O | Leer argumentos, escribir stdout/stderr y manejar archivos. |
| Testing | Pruebas unitarias, al menos una prueba de integración y, si aplica, doc |
|   | tests. |
| Documentación | README, comentarios de documentación y ejemplos de uso. |
| Crates externos | Integración responsable de librerías externas, justificando su uso. |
| Concurrencia o async | Opcional avanzado: procesamiento concurrente de import/export, |
|   | estadísticas o tareas independientes. |
| Smart | No son obligatorios; pueden usarse solo si aportan algo real y se justifican. |
| pointers/macros/unsafe |   |


## Comandos de verificación esperados

Antes de entregar, el estudiante debe ejecutar estos comandos desde la raíz del proyecto:

```
cargo check
cargo fmt
cargo clippy
cargo test
cargo doc --no-deps
```

## 6. Arquitectura recomendada

Una buena arquitectura no significa tener muchos archivos por tenerlos. Significa separar responsabilidades para que el código sea fácil de leer, probar y modificar. Se recomienda que main.rs sea pequeño y que la lógica principal viva en lib.rs y en módulos internos.

## Estructura sugerida de carpetas

```
studyflow/
├── Cargo.toml
├── README.md
├── src/
│ ├── main.rs # Entrada de la aplicación
│ ├── lib.rs # API interna principal
│ ├── cli.rs # Parsing de argumentos y subcomandos
│ ├── models.rs # Tarea, Estado, Prioridad, Categoria
│ ├── storage.rs # Lectura y escritura del archivo de datos
│ ├── errors.rs # Tipo de error o alias de Result
│ ├── config.rs # Ruta de datos y variables de entorno
│ ├── reports.rs # Salida formateada y estadísticas
│ └── commands.rs # Funciones que ejecutan comandos
└── tests/
└── cli_flow_test.rs # Prueba de integración sugerida
```

## Responsabilidades por módulo

| Módulo | Responsabilidad |
| --- | --- |
| main.rs | Recolectar argumentos, llamar al parser, ejecutar y mostrar errores generales. |
| cli.rs | Definir subcomandos, argumentos y opciones. |
| models.rs | Definir los tipos del dominio: Tarea, Estado, Prioridad. |
| storage.rs | Leer y guardar datos. Debe ocultar detalles del formato de archivo. |
| errors.rs | Centralizar errores propios o alias de Result. |
| config.rs | Resolver rutas, variables de entorno y valores por defecto. |
| commands.rs | Implementar la lógica de cada comando. |
| reports.rs | Generar textos de salida, tablas simples o estadísticas. |

## 7. Datos, persistencia y configuración

## Modelo de datos mínimo

El modelo puede variar, pero debe incluir suficiente información para operar la herramienta. Una tarea debería tener al menos id, descripción, estado y fecha o marca de creación si el estudiante decide manejar fechas.

| Campo | Tipo sugerido | Descripción |
| --- | --- | --- |
| id | u64 o String | Identificador único de la tarea. |
| descripcion | String | Texto principal de la tarea. |


| Campo | Tipo sugerido | Descripción |
| --- | --- | --- |
| estado | enum Estado | Pendiente o completada. |
| prioridad | u8 o enum | Nivel de importancia. |
| categoria | Option | Tema o agrupación opcional. |
| creada_en | String u opción con crate | Fecha opcional de creación. |
|   | de fecha |   |

## Persistencia

El programa debe guardar sus datos en un archivo local. La opción recomendada es JSON usando serde y serde_json, porque permite representar structs y enums de forma clara. También se permite CSV, TOML o un formato propio, siempre que esté documentado y probado.

## Configuración

La herramienta debe permitir definir la ruta del archivo de datos. Puede tener una ruta por defecto, pero también debe aceptar una variable de entorno, por ejemplo STUDYFLOW_DATA_PATH. Esto permite probar la aplicación sin modificar archivos reales del usuario.

\# Ejemplo de ejecución con ruta de datos temporal: STUDYFLOW_DATA_PATH=./datos_prueba.json cargo run -- list

## 8. Uso responsable de librerías del ecosistema

El proyecto debe integrar crates externos de forma responsable. No se trata de agregar dependencias por cantidad, sino de elegir librerías que resuelvan problemas reales y que mejoren la seguridad, claridad o mantenibilidad.

| Crate | Uso posible | Recomendación |
| --- | --- | --- |
| clap | Parsing de argumentos, subcomandos | Recomendado para una CLI más |
|   | y ayuda. | profesional. |
| serde | Serializar y deserializar modelos. | Recomendado si se usa JSON, TOML u |
|   |   | otro formato soportado. |
| serde_json | Persistencia en JSON. | Recomendado para el formato de datos |
|   |   | del proyecto. |
| thiserror | Errores personalizados con menos | Opcional, útil si se diseñan errores |
|   | boilerplate. | propios. |
| anyhow | Manejo flexible de errores en | Opcional, útil en capa de aplicación, no |
|   | aplicaciones. | tanto en librerías públicas. |
| chrono o time | Fechas y horas. | Opcional si se registran fechas de |
|   |   | creación o vencimiento. |
| uuid | IDs únicos. | Opcional si no se quiere generar IDs |
|   |   | incrementales. |

## Checklist antes de agregar una crate

- Revisar documentación y ejemplos básicos.

- Confirmar que la crate resuelve un problema real del proyecto.

- Revisar actividad reciente, versión y adopción.

- Revisar licencia si el proyecto se publicará.


- Evitar dependencias para tareas triviales que se pueden resolver claramente con la librería estándar.

## 9. Manejo de errores

El proyecto debe evitar panic! para errores esperados del usuario, como argumentos faltantes, archivo inexistente, JSON inválido o id no encontrado. Esos casos son recuperables y deben comunicarse con mensajes claros.

## Errores esperados

| Caso | Respuesta esperada |
| --- | --- |
| El archivo de datos no existe | Crear archivo inicial o mostrar instrucción clara. |
| JSON o formato inválido | Mostrar error y no sobrescribir datos automáticamente. |
| ID inexistente | Indicar que no hay tarea con ese identificador. |
| Descripción vacía | Rechazar la tarea con un mensaje claro. |
| Argumentos incompletos | Mostrar uso esperado o ayuda del comando. |

## Ejemplo básico de error propio

```
// Ejemplo aislado. No es la solución final.
#[derive(Debug)]
enum AppError {
Io(std::io::Error),
DatosInvalidos(String),
TareaNoEncontrada(u64),
}
type AppResult<T> = Result<T, AppError>;
```

## 10. Testing, documentación y calidad

## Pruebas mínimas

- Pruebas unitarias para funciones puras: filtros, búsqueda, cálculo de estadísticas y validación de datos.

- Pruebas para casos borde: lista vacía, id inexistente, descripción vacía y archivo inválido.

- Al menos una prueba de integración que simule un flujo principal, por ejemplo agregar y completar una tarea usando funciones de la librería.

- Si se documentan funciones públicas con ejemplos, ejecutar doc tests cuando aplique.

## Documentación requerida

- README con descripción del proyecto, instalación, comandos y ejemplos de uso.

- Comentarios de documentación en funciones públicas importantes.

- Explicación del formato de archivo usado para persistencia.

- Sección de decisiones técnicas: crates elegidas, estructura de módulos y manejo de errores.

## Calidad

- El proyecto debe pasar cargo fmt.

- El proyecto debe pasar cargo clippy o justificar advertencias relevantes.


- El proyecto debe compilar sin warnings importantes.

- Los mensajes de error deben ser entendibles para una persona usuaria, no solo para quien programó.

## 11. Ejemplos básicos de apoyo

Los siguientes fragmentos son ejemplos mínimos para orientar el diseño. No resuelven el proyecto final y no deben copiarse como arquitectura completa.

## Ejemplo 1: modelo de comando

```
// Ejemplo conceptual de modelado con enum.
enum Comando {
Agregar { descripcion: String },
Listar,
Completar { id: u64 },
}
```

## Ejemplo 2: modelo de tarea

```
// Ejemplo conceptual. El modelo final puede tener más campos.
struct Tarea {
id: u64,
descripcion: String,
completada: bool,
}
```

## Ejemplo 3: función pura fácil de probar

```
// Esta clase de función es ideal para pruebas unitarias.
fn contar_completadas(tareas: &[Tarea]) -> usize {
tareas.iter().filter(|tarea| tarea.completada).count()
}
```

## Ejemplo 4: separación de main y librería

```
// main.rs debería coordinar, no contener toda la lógica.
fn main() {
// 1. Leer argumentos.
// 2. Construir configuración.
// 3. Ejecutar comando.
// 4. Mostrar errores de forma clara.
}
```

## Ejemplo 5: prueba unitaria simple

```
#[test]
fn cuenta_tareas_completadas() {
let tareas = vec![
Tarea { id: 1, descripcion: "A".into(), completada: true },
Tarea { id: 2, descripcion: "B".into(), completada: false },
];
assert_eq!(contar_completadas(&tareas), 1);
}
```

## 12. Entregables

| Entregable | Contenido mínimo |
| --- | --- |
| Repositorio | Código fuente completo, Cargo.toml, README y tests. |
| README | Descripción, instalación, comandos, ejemplos y decisiones técnicas. |
| Demo | Video corto o presentación en vivo ejecutando comandos principales. |
| Explicación técnica | Arquitectura, módulos, errores, persistencia, pruebas y crates. |
| Reflexión final | Qué fue lo más difícil, qué mejorarías y qué ruta de Rust te interesa seguir. |


## 13. Presentación final

La presentación final debe demostrar el funcionamiento del proyecto y explicar las decisiones técnicas. No basta con mostrar que el comando funciona; también se debe explicar cómo está organizado el código y por qué se tomaron ciertas decisiones.

## Estructura sugerida de presentación

- 1 Problema: qué necesidad resuelve la herramienta.

- 2 Demo: comandos principales y salida esperada.

- 3 Arquitectura: módulos, separación entre app y librería, modelos y persistencia.

- 4 Manejo de errores: ejemplos de casos inválidos y cómo responde la app.

- 5 Testing y documentación: pruebas implementadas y comandos de verificación.

- 6 Crates usadas: por qué se eligieron y qué problema resolvieron.

- 7 Mejoras futuras: qué agregarías con más tiempo.

## 14. Rúbrica de evaluación

| Criterio | Porcentaje | Qué se evalúa |
| --- | --- | --- |
| Funcionalidad CLI | 25% | Comandos principales, filtros, estadísticas, salida clara |
|   |   | y manejo de entradas inválidas. |
| Arquitectura | 20% | Separación de responsabilidades, módulos, |
|   |   | lib.rs/main.rs y claridad del diseño. |
| Persistencia y errores | 20% | Lectura/escritura segura, Result, ?, errores |
|   |   | recuperables y mensajes accionables. |
| Testing y documentación | 20% | Pruebas, README, doc comments y comandos de |
|   |   | verificación. |
| Presentación técnica | 15% | Demo clara, explicación de decisiones, limitaciones y |
|   |   | mejoras futuras. |

## 15. Checklist final del estudiante

- El proyecto compila con cargo build.

- La CLI muestra ayuda clara.

- Los comandos obligatorios funcionan.

- Los datos persisten entre ejecuciones.

- Los errores esperados no causan panic!.

- El código está separado en módulos con responsabilidades claras.

- Hay pruebas unitarias y al menos una prueba de integración.

- cargo fmt, cargo clippy y cargo test se ejecutaron antes de entregar.

- README incluye instalación, uso, ejemplos y decisiones técnicas.

- La demo final muestra tanto funcionamiento como explicación del diseño.

## 16. Referencias recomendadas


The Rust Programming Language - An I/O Project: Building a Command Line Program https://doc.rust-lang.org/book/ch12-00-an-io-project.html

The Rust Programming Language - Accepting Command Line Arguments https://doc.rust-lang.org/book/ch12-01-accepting-command-line-arguments.html

The Rust Programming Language - Recoverable Errors with Result https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html

The Cargo Book - cargo check https://doc.rust-lang.org/cargo/commands/cargo-check.html

The Cargo Book - cargo test https://doc.rust-lang.org/cargo/commands/cargo-test.html

The Cargo Book - cargo doc https://doc.rust-lang.org/cargo/commands/cargo-doc.html

clap documentation https://docs.rs/clap

Serde overview https://serde.rs/

rustdoc documentation tests https://doc.rust-lang.org/rustdoc/documentation-tests.html
