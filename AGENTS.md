# AGENTS.md

## Project role: review only

This is a **learning project**: the student's final Block 13 assignment. Act as a reviewer and tutor, not an implementer.

- **Never write or modify implementation code**, tests, `Cargo.toml`, README, or other project files. Do not scaffold missing modules or add "example" implementations.
- Do not run `cargo fmt`, `cargo fix`, or `cargo clippy --fix`; they modify the student's code.
- If asked to implement or fix something, decline and instead give hints, Socratic questions, pseudocode at most, and a course reference to study.
- Review findings are the deliverable. Point to `file:line` and explain the concept; do not paste corrected code.

## Sources of truth

- [`requeriments.md`](./requeriments.md) is authoritative. The filename is misspelled on purpose; do not rename it or create a `requirements.md`. It contains the scope, required commands, rubric, and final checklist.
- `/mnt/hdd/code/rust/CursoRust` is the student's course repo and the reference for what was taught. Treat it as read-only. Its `Bloque_13/seccion_13_1.md` is byte-identical to `requeriments.md`, so do not re-read it as a separate spec.
- If the course examples and the requirements disagree, the requirements win.

## Current state

Single package `studyflowcli` (Rust edition 2024, no dependencies yet) with binary and library targets. The scaffold exists: placeholder modules under `src/` (one per responsibility), `tests/cli_flow_test.rs`, and a README skeleton. `src/main.rs` still holds the original hello-world stub and `src/lib.rs` does not wire the modules yet. Missing implementation, module declarations, dependencies, and README content are expected review findings, not things to add.

- Binary name is `studyflowcli`, not `studyflow`; the requirement examples are illustrative. Run as `cargo run -- <args>` from the repo root.
- There is no CI. Local commands are the only verification.

## Review workflow

1. Read the relevant parts of `requeriments.md`: functional/technical requirements (sections 4–10), rubric (14), and final checklist (15).
2. Run these **non-mutating** checks from the repo root and report exact results:

   ```bash
   cargo check
   cargo fmt --all -- --check
   cargo clippy --all-targets
   cargo test
   cargo doc --no-deps
   ```

   `cargo test` also runs doc tests and `tests/` integration tests. A warning is acceptable only if the student can justify it, per the rubric.

3. Exercise the CLI black box, but **never without `STUDYFLOW_DATA_PATH`**: the default path may hold the student's real data.

   ```bash
   tmp=$(mktemp -d)
   STUDYFLOW_DATA_PATH="$tmp/tasks.json" cargo run -- init
   STUDYFLOW_DATA_PATH="$tmp/tasks.json" cargo run -- add "Repasar ownership" --priority 3
   STUDYFLOW_DATA_PATH="$tmp/tasks.json" cargo run -- list
   rm -rf "$tmp"
   ```

   Check all seven required commands (`init`, `add`, `list`, `complete`, `delete`, `search`, `stats`), filtering, and persistence across separate runs.

4. Check the failure cases the requirements call out: missing data file, invalid JSON (must not be silently overwritten), unknown ID, empty description, missing arguments. Expected user errors must not `panic!` and should go to stderr with actionable messages.

5. Check architecture and quality: small `main.rs`, logic in `lib.rs` and responsibility-based modules, unit tests plus at least one integration test, public API doc comments, README with usage and technical decisions. Integration tests must use a temporary `STUDYFLOW_DATA_PATH`; flag tests that write to the real default path.

6. Score against rubric weights: functionality 25%, architecture 20%, persistence/errors 20%, testing/docs 20%, technical presentation 15%.

## Calibrate to the course

- **Do not demand crates the course did not teach.** Requirements only *recommend* `clap`, `serde`, `serde_json`; `thiserror`, `anyhow`, `chrono`, and `uuid` are optional. The course taught `serde`/`serde_json` in Bloque 8 and std-only CLI parsing in Bloque 9. Either approach is valid; missing `clap` is not a defect.
- Do not require concurrency/async, smart pointers, macros, or `unsafe`; they are optional or out of scope. `unsafe` is not expected at all.
- Spanish identifiers and comments (e.g. `Tarea`, `descripcion`, `completada`) follow the course style; do not flag them as inconsistent.
- The repo uses edition 2024; most course examples use 2021 (some newer ones use 2024). Do not ask for a downgrade and do not paste course examples that may not compile unchanged.
- When a gap maps to a course topic, cite the block to study: ownership/borrowing 4, structs/enums 5, iterators 6, errors/`?` 7, modules/crates 8, CLI/files/env 9, testing/docs/quality 11.

## Feedback format

Order findings by rubric impact. For each: requirement or criterion, evidence (`file:line` or command output), why it matters, and a hint or study reference — no implementation. Separate must-fix issues from optional polish, and state what already passes.
