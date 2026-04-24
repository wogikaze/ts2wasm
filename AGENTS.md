# Repository Guidelines

## Project Structure & Module Organization

This repository contains design documentation and the first Rust shared-definition crate for a TypeScript/JavaScript to WebAssembly compiler/runtime project.

- `README.md` is the project entry point and documentation map.
- `docs/00-*.md` through `docs/11-*.md` hold the split design documents. Keep detailed design changes in the relevant doc, not only in `README.md`.
- `docs/99-original-plan.md` preserves the original plan for loss-checking. Do not rewrite it as live design.
- `crates/shared/` contains M0 canonical Rust definitions for runtime ABI, capability manifests, and test status records.
- `.agents/skills/` contains repository-specific agent workflows. Keep them concise and procedural.
- `reference/` contains upstream reference material such as TypeScript, WASI, WebAssembly spec, and test262. Treat it as read-only unless explicitly updating references.

Future implementation should use clear top-level directories such as `runtime/`, `tests/`, and `fixtures/`, and update `README.md` plus `docs/00-docs-list.md`.

## Build, Test, and Development Commands

Key commands:

```bash
cargo test
cargo fmt --all
which iwasm
ig "process.env" docs
ig "runtime ABI" docs
```

`cargo test` runs the current Rust unit tests. `cargo fmt --all` formats Rust code. `which iwasm` verifies the required WASI runner is available. Use `ig` for code and document search; fall back to `rg` only if `ig` is unavailable.

## Coding Style & Naming Conventions

Use Markdown for documentation and Rust for compiler/runtime implementation unless a later design doc explicitly changes this.

- Use lowercase, hyphenated doc filenames with numeric prefixes, for example `docs/04-compiler-architecture-and-runtime.md`.
- Use Rust module and file names in `snake_case`; use type names in `UpperCamelCase`.
- Keep terminology consistent: `generated wasm`, `WASM runtime`, `host shim`, `capability manifest`, and `standalone WASI`.
- TypeScript/ECMAScript is the input-language baseline. Do not introduce AssemblyScript-only syntax or types as user-facing requirements.

## Testing Guidelines

Testing policy is documented in `docs/06-testing-and-coverage.md`. New implementation work should include fixtures that classify results as `pass`, `fail`, `unsupported`, `blocked`, or `skip-with-reason`.

For compiler/runtime changes, add or update tests for Node differential behavior, WASI/iwasm execution, runtime ABI compatibility, capability manifests, and unsupported-feature diagnostics.

M0 changes must keep `docs/11-shared-definitions.md` and `crates/shared/` aligned.

## Commit & Pull Request Guidelines

This checkout has no Git history, so no local commit convention can be inferred. Use short imperative commit messages, for example `docs: clarify env fallback policy`.

Commit in small logical work units. Do not collapse an entire assistant turn into one commit when it contains separable work. Prefer boundaries such as documentation baseline, shared schema implementation, repository agent workflow, and verification/configuration cleanup.

Pull requests should include the motivation, affected docs or modules, verification performed, and any unresolved compatibility or security tradeoffs. Link related issues when available.

## Security & Configuration Notes

Host capabilities must stay explicit. If a feature requires filesystem, environment, network, crypto, timers, or Node.js fallback, document the capability and update the relevant manifest/security docs.
