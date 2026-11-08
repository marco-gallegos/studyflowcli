# Demo de StudyFlow CLI

Guion reproducible para demostrar el funcionamiento de la herramienta y las
decisiones técnicas del proyecto. Todos los comandos usan un archivo temporal
mediante `STUDYFLOW_DATA_PATH`, por lo que **no se modifica ningún dato real**.

## Preparación

Desde la raíz del repositorio:

```bash
cargo build
```

Crea un directorio temporal y apunta ahí el archivo de datos:

```bash
export STUDYFLOW_DATA_PATH="$(mktemp -d)/tasks.json"
echo "$STUDYFLOW_DATA_PATH"
```

Si prefieres usar el binario compilado directamente:

```bash
BIN=target/debug/studyflowcli
```

En los ejemplos siguientes se usa `$BIN`. También puedes reemplazarlo por
`cargo run --`.

---

## 1. Inicializar el proyecto

```bash
"$BIN" init
```

Salida esperada:

```text
Nuevo proyecto inicializado en '/tmp/.../tasks.json'.
```

`init` no sobrescribe un archivo existente:

```bash
"$BIN" init
```

```text
Error: El archivo '/tmp/.../tasks.json' ya existe.
```

---

## 2. Agregar tareas

```bash
"$BIN" add "Repasar ownership" --priority 5
"$BIN" add "Comprar café" --priority 3
"$BIN" add "Leer capítulo 5" --priority 2
```

Salida esperada de cada comando:

```text
Tarea agregada exitosamente.
```

La prioridad es opcional; si se omite, vale `3`.

---

## 3. Listar tareas

```bash
"$BIN" list
```

Salida esperada (los IDs cambian en cada ejecución):

```text
tarea 1794101646506013046: Repasar ownership (Estado: Pendiente, Prioridad: 5)
tarea 1794101646507138584: Comprar café (Estado: Pendiente, Prioridad: 3)
tarea 1794101646508335518: Leer capítulo 5 (Estado: Pendiente, Prioridad: 2)
```

### Filtros

```bash
"$BIN" list --status pendiente
"$BIN" list --status pending
"$BIN" list --status completada
"$BIN" list --status completed
"$BIN" list --priority 2
```

Los alias `pending` y `completed` se aceptan además de `pendiente` y
`completada`. Los filtros se pueden combinar:

```bash
"$BIN" list --status pendiente --priority 2
```

Si no hay coincidencias:

```text
No se encontraron tareas con los filtros indicados.
```

---

## 4. Completar una tarea

Obtén el ID de la primera tarea y márcala como completada:

```bash
ID=$(python3 -c 'import json,sys; print(json.load(open(sys.argv[1]))[0]["id"])' "$STUDYFLOW_DATA_PATH")
"$BIN" complete "$ID"
```

Salida esperada:

```text
Tarea con ID 1794101646506013046 marcada como completada.
```

Comprueba el cambio:

```bash
"$BIN" list --status completed
```

```text
tarea 1794101646506013046: Repasar ownership (Estado: Completada, Prioridad: 5)
```

---

## 5. Buscar tareas

```bash
"$BIN" search café
```

```text
tarea 1794101646507138584: Comprar café (Estado: Pendiente, Prioridad: 3)
```

La búsqueda no distingue mayúsculas de minúsculas y también acepta prioridad:

```bash
"$BIN" search 2
```

```text
tarea 1794101646508335518: Leer capítulo 5 (Estado: Pendiente, Prioridad: 2)
```

---

## 6. Ver estadísticas

```bash
"$BIN" stats
```

```text
Estadísticas de tareas:
Total de tareas: 3
Tareas completadas: 1
Tareas pendientes: 2
Avance: 33.3%
```

---

## 7. Eliminar una tarea

```bash
ID2=$(python3 -c 'import json,sys; print(json.load(open(sys.argv[1]))[1]["id"])' "$STUDYFLOW_DATA_PATH")
"$BIN" delete "$ID2"
```

```text
Tarea con ID 1794101646507138584 eliminada exitosamente.
```

---

## 8. Verificar la persistencia

Ejecuta `list` de nuevo, incluso en otra terminal, manteniendo
`STUDYFLOW_DATA_PATH`:

```bash
"$BIN" list
```

```text
tarea 1794101646506013046: Repasar ownership (Estado: Completada, Prioridad: 5)
tarea 1794101646508335518: Leer capítulo 5 (Estado: Pendiente, Prioridad: 2)
```

Las tareas completadas y las pendientes se conservan entre ejecuciones.

---

## 9. Mostrar errores controlados

Los errores esperados van a `stderr` y terminan con código `1`. No hay
`panic!`.

```bash
"$BIN" complete 999
```

```text
Error: No existe una tarea con ID 999.
```

```bash
"$BIN" list --status foo
```

```text
Error: El estado 'foo' no es válido. Usa 'pendiente'/'pending' o 'completada'/'completed'.
```

```bash
"$BIN" list --status
```

```text
Error: Falta un argumento para '--status'. Uso: studyflowcli list --status <pendiente|completada>
```

```bash
"$BIN" add "   "
```

```text
Error: La descripción no puede estar vacía.
```

El archivo inexistente y el JSON inválido también producen un mensaje claro.
En el caso de JSON inválido, el archivo **no se sobrescribe**.

---

## 10. Limpieza

```bash
rm -rf "$(dirname "$STUDYFLOW_DATA_PATH")"
unset STUDYFLOW_DATA_PATH
```

---

## Guion sugerido para la presentación (5–7 minutos)

1. **Problema:** centralizar tareas de estudio en una CLI con persistencia
   local.
2. **Demo:** `init`, `add`, `list` con filtros, `complete`, `search`, `stats`,
   `delete` y un error controlado.
3. **Arquitectura:** `main.rs` delgado, lógica en `lib.rs`, módulos `cli`,
   `commands`, `models`, `storage`, `errors`, `config` y `reports`.
4. **Errores:** `AppError`/`AppResult`, uso de `?`, mensajes en `stderr` y
   códigos de salida.
5. **Testing y documentación:** 57 pruebas unitarias, 4 de integración y 33
   doc tests; README y comandos de verificación.
6. **Crates:** `serde` y `serde_json` para persistencia JSON; parser con la
   librería estándar.
7. **Mejoras futuras:** `--category` en `add`, separar la persistencia en
   `storage.rs`, mover los tipos de CLI a `cli.rs` y escrituras atómicas.

---

## Resultado esperado de la demo

Al finalizar deben verse:

- Un archivo JSON con las tareas y sus estados.
- Filtros por estado y prioridad funcionando.
- Persistencia entre ejecuciones.
- Errores claros sin `panic!`.
- Estadísticas con total, completadas, pendientes y porcentaje.

El veredicto completo y la rúbrica están en [`status.md`](./status.md).
