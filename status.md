# Estado del proyecto: StudyFlow CLI

**Fecha:** 2026-11-07  
**Rama revisada:** `develop` (`1e6dcfe`)  
**Referencia:** [`requeriments.md`](./requeriments.md)  
**Demo reproducible:** [`demo.md`](./demo.md)

---

## Veredicto

**El proyecto pasa la checklist verificable de entrega y se considera
entregable.** La nota estimada es **≈88/100**, asumiendo una demo y una
explicación técnica correctas.

- **Verificable en el repositorio:** 74.5/85 ≈ **87.6%**.
- **Presentación técnica (15%):** 13/15 proyectada; no es verificable desde el
  repositorio.
- **Sin demo:** la nota máxima verificable sería **74.5/100**.

No quedan bloqueantes. Lo que impide llegar al 100% es un hueco funcional
(`--category` en `add`) y dos detalles de arquitectura (`storage.rs` no oculta
el formato y algunos tipos de CLI viven en `models.rs`).

---

## Verificación ejecutada

| Comando | Resultado |
|---|---|
| `cargo check` | ✅ Correcto |
| `cargo fmt --all -- --check` | ✅ Correcto |
| `cargo clippy --all-targets -- -D warnings` | ✅ Sin warnings |
| `cargo test` | ✅ 57 unitarias + 4 de integración + 33 doc tests |
| `cargo doc --no-deps` | ✅ Correcto |
| `RUSTDOCFLAGS="-D warnings" cargo doc --no-deps` | ✅ Sin warnings |

Prueba manual del binario con `STUDYFLOW_DATA_PATH` temporal: los siete
comandos obligatorios funcionan, los filtros y sus alias responden, la
persistencia se conserva entre ejecuciones y los errores esperados terminan con
código `1` sin `panic!`.

---

## Checklist final (sección 15)

| Ítem | Estado |
|---|---|
| Compila con `cargo build` | ✅ |
| La CLI muestra ayuda clara | ✅ |
| Los comandos obligatorios funcionan | ✅ |
| Los datos persisten entre ejecuciones | ✅ |
| Los errores esperados no causan `panic!` | ✅ |
| Código separado en módulos con responsabilidades claras | ✅ con reservas |
| Pruebas unitarias y al menos una de integración | ✅ |
| `cargo fmt`, `cargo clippy` y `cargo test` ejecutados | ✅ |
| README con instalación, uso, ejemplos y decisiones | ✅ |
| Demo final con funcionamiento y explicación del diseño | ⏳ Pendiente de presentar |

---

## Rúbrica estimada

| Criterio | Peso | Nota | Motivo |
|---|---:|---:|---|
| Funcionalidad CLI | 25% | **21.5/25** | Todos los comandos y filtros funcionan; falta `--category` en `add` y la ayuda está desactualizada |
| Arquitectura | 20% | **16/20** | Buena separación y `main.rs` delgado; `storage.rs` no oculta el JSON y hay tipos de CLI en `models.rs` |
| Persistencia y errores | 20% | **18.5/20** | Muy sólido; mensaje de prioridad fuera de rango y escrituras no atómicas como detalles menores |
| Testing y documentación | 20% | **18.5/20** | Suite amplia y verificaciones limpias; ayuda y URL del README por pulir |
| Presentación técnica | 15% | **13/15 proyectada** | Depende de la demo y la explicación; no verificable en el repositorio |
| **Total** | **100%** | **≈88/100** | **74.5/85 verificable + presentación proyectada** |

---

## Hallazgos por impacto

### 1. `add` no acepta categoría y la ignora en silencio — Funcionalidad (25%) — alto

**Requisito:** sección 3, tabla de comandos: `add` agrega una tarea con
descripción, prioridad y categoría opcional. `search` debe buscar por texto,
categoría o prioridad.

**Evidencia:**

- `src/commands.rs:344` fija `let categoria: Option<String> = None;`.
- `src/cli.rs:152` solo excluye `--priority` y `--status` de los argumentos
  posicionales; `--category` no se reconoce.
- `studyflowcli add "Repasar ownership" --priority 5 --category rust` guarda
  `"categoria":null` y no avisa.

**Impacto:** el usuario cree que guardó la categoría y la búsqueda por
categoría no es utilizable desde la CLI normal, aunque el código y los tests la
soporten con datos creados manualmente.

**Pista de estudio:** agregar `--category` al parser, guardarlo como
`Option<String>` y escribir primero un test que falle. Bloques 5 y 9.

### 2. `storage.rs` no oculta el formato del archivo — Arquitectura (20%) — medio-alto

**Requisito:** sección 6: `storage.rs` debe ocultar los detalles del formato.

**Evidencia:**

- `src/commands.rs:27` deserializa con `serde_json::from_str`.
- `src/commands.rs:356`, `src/commands.rs:427` y `src/commands.rs:487`
  escriben con `fs::write(file_name, serde_json::to_string(&todos)?)`.

**Impacto:** cambiar el formato obliga a tocar varios comandos; la
responsabilidad de persistencia queda repartida.

**Pista de estudio:** mover lectura y escritura de tareas a `storage.rs`.
Bloque 8.

### 3. Tipos de CLI y reporte en `models.rs` — Arquitectura — medio

**Requisito:** sección 6: `cli.rs` define subcomandos, argumentos y opciones;
`models.rs` define el dominio; `reports.rs` genera estadísticas.

**Evidencia:**

- `src/models.rs:45`: `Subcommand`.
- `src/models.rs:175`: `CliOptions`.
- `src/models.rs:204`: `CliArgs`.
- `src/models.rs:230`: `StatusReport`.

**Impacto:** mezcla dominio con entrada y reportes; `models.rs` depende de
decisiones de la CLI.

**Pista de estudio:** mover esos tipos a `cli.rs` y `reports.rs`. Bloque 8.

### 4. La ayuda sigue desactualizada — Documentación — bajo

**Evidencia:**

- `src/cli.rs:23` dice “Lista todas las tareas”, sin mencionar filtros.
- `src/cli.rs:31` no menciona los alias `pending`/`completed`.

**Impacto:** el README está actualizado, pero la ayuda que ve el usuario en la
terminal no refleja el comportamiento real.

**Pista de estudio:** actualizar `print_cli_help` y, si es posible, probar que
la ayuda menciona `--status` y `--priority`. Bloque 11.

### 5. Mensaje engañoso para prioridad fuera de rango — Errores — bajo

**Evidencia:** `studyflowcli add "x" --priority 300` responde “La prioridad
'300' no es un número válido” (`src/errors.rs:70-72`, parseo en
`src/cli.rs:107-109`). `300` sí es un número; no cabe en `u8`.

**Impacto:** la sección 9 pide mensajes accionables.

**Pista de estudio:** distinguir error de parseo y error de rango, o indicar
“debe estar entre 0 y 255”. Bloque 7.

### 6. Detalles de entrega — bajo

- `.gitignore` solo contiene `/target`; `studyflow_project.json` no está
  ignorado y podría subirse por accidente.
- `README.md:19` conserva `git clone <url-del-repositorio>`.

**Pista de estudio:** ignorar el archivo de datos local y colocar la URL real.
Sección 12.

### 7. Pulido opcional

- `CliOptions` usa `priority: u8` + `priority_found: bool` en vez de
  `Option<u8>` (`src/models.rs:175-182`). **Bloque 5.**
- Las opciones se validan globalmente: `complete 1 --status foo` falla por una
  opción que ese comando no usa. **Bloque 9.**
- `filter_active_todos` y `get_todos(..., true)` existen y están probados, pero
  el CLI no los usa. **Bloque 6.**
- Las escrituras no son atómicas; no es obligatorio, pero es una mejora real.
  **Bloque 9.**
- `creado_en` siempre vale `"hoy"`; está documentado como limitación y la fecha
  es opcional según la sección 7.

---

## Lo que ya está bien

- Los siete comandos obligatorios funcionan: `init`, `add`, `list`, `complete`,
  `delete`, `search` y `stats`.
- `list` filtra por estado (`pendiente`/`pending`, `completada`/`completed`),
  por prioridad y por combinación de ambos.
- Persistencia entre ejecuciones; `complete` y `delete` conservan las demás
  tareas y no corrompen el archivo.
- Errores esperados sin `panic!`: archivo inexistente, JSON inválido sin
  sobrescritura, ID inválido o inexistente, descripción vacía, argumentos
  faltantes, estado inválido y opción sin valor.
- `STUDYFLOW_DATA_PATH` funciona; las pruebas usan rutas temporales y no tocan
  datos reales.
- `main.rs` delgado, lógica en la librería, módulos por responsabilidad y
  `AppError`/`AppResult` con `Result` y `?`.
- 57 pruebas unitarias, 4 de integración y 33 doc tests; `fmt`, `clippy -D
  warnings`, `test` y `doc -D warnings` limpios.
- README con instalación, uso, formato JSON, arquitectura, decisiones y
  limitaciones; doc comments en la API pública.
- `stats` calcula totales, pendientes, completadas y porcentaje con división por
  cero controlada y soporte para más de 255 tareas.
- `serde` y `serde_json` resuelven la persistencia; el parser con `std` es
  válido para el alcance del curso.

---

## Plan para llegar a 93–96/100

1. Implementar `--category` en `add` y conectarlo con `search`.
2. Mover la lectura y escritura JSON a `storage.rs`.
3. Mover `Subcommand`, `CliOptions` y `CliArgs` a `cli.rs`, y `StatusReport` a
   `reports.rs`.
4. Actualizar el texto de `help`.
5. Ignorar el archivo de datos local, colocar la URL real del README y preparar
   la demo final.
6. Opcional: distinguir prioridad fuera de rango, usar `Option<u8>` para la
   prioridad y validar opciones por subcomando.

---

## Conclusión

El proyecto es **entregable y pasa la checklist verificable**. La base técnica
es buena: comandos completos, persistencia confiable, errores recuperables,
pruebas amplias y documentación sólida. El siguiente salto de calidad está en
cerrar el hueco de categoría y afinar la separación de responsabilidades para
que `storage.rs`, `cli.rs` y `reports.rs` cumplan exactamente el rol que pide la
sección 6.
