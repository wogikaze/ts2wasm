---
id: 059
title: "Implement parser syntax extensions for TypeScript and advanced JS"
type: feature
area: frontend
class: design-ready
priority: P1
depends_on: []
blocks: []
created: 2026-04-26
updated: 2026-04-28
---

## Summary

Implement parser syntax extensions to handle parser-syntax feature gaps in reference tests.

## Problem

Reference test results show 115 cases fail with parser-syntax diagnostic (test262:14, tsc:77, tsgo:24). The parser cannot handle various TypeScript and advanced JavaScript syntax constructs, preventing compilation of modern code.

## Desired final state

Parser supports common TypeScript and advanced JavaScript syntax constructs. parser-syntax diagnostic is only emitted for genuinely unsupported syntax.

## Scope

In scope:

- [ ] Add TypeScript type annotations to parser
- [ ] Add TypeScript interface declarations
- [ ] Add TypeScript generic syntax
- [ ] Add advanced JavaScript syntax (decorators, private fields, etc.)
- [ ] Update diagnostic to emit parser-syntax only when appropriate

Out of scope:

- [ ] Full TypeScript type checking (separate issue)
- [ ] TypeScript emit semantics (separate issue)

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/cli/src/`
- `fixtures/`

Do not touch:

- `crates/runtime-abi/`
- `crates/backend-wasm/`

## Acceptance criteria

- [ ] Parser accepts common TypeScript syntax
- [ ] parser-syntax diagnostic significantly reduced in reference tests
- [ ] Regression test added for parser syntax
- [ ] Docs updated if semantics change

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
scripts/manager reference-coverage test262 --limit 200
scripts/manager reference-coverage tsc --limit 100
scripts/manager reference-coverage tsgo --limit 50
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [ ] not affected

Current state:

- [ ] updated: `current-state.md` (repo root)

Follow-up issues:

- [ ] none

## Notes

Start with basic TypeScript type annotations before adding advanced features.

2026-04-28 progress evidence:

- Implemented a narrow erasable TypeScript type-annotation parser slice for variable declarations, function parameter annotations, and return annotations.
- Uninitialized `let` / `var` declarations after optional type annotations now parse as `undefined`; uninitialized `const` declarations still report a diagnostic.
- Added fixture `fixtures/basics-types/type-annotation-erasure.ts`.
- Added CLI coverage showing `dump --ast --unparse` erases annotations and build accepts the fixture.
- Validation passed:
  - `cargo fmt --all --check`
  - `cargo nextest run -p ts2wasm-frontend`
  - `cargo nextest run -p ts2wasm-cli --test dump_cli`
  - `cargo nextest run`
  - `scripts/manager update-issue-index --check`
  - `scripts/manager check-issue-health`
  - `scripts/manager check-agent-state`
  - `scripts/manager check-repo-smoke`
- Issue 059 remains open. Interfaces, generics, decorators, private fields, broader parser-syntax diagnostic reduction, and reference-ramp evidence remain outside this slice.

2026-04-28 progress evidence (interface-erasure slice):

- Implemented a narrow parser-only TypeScript `interface` / `export interface` declaration erasure slice.
- Interface declarations are consumed before AST construction, so dump `--ast --unparse` and build output omit them while preserving subsequent runtime statements.
- Added fixture `fixtures/basics-types/interface-erasure.ts`.
- Added frontend parser coverage for erased interface declarations with members, methods, optional members, `extends`, and nested type-literal braces.
- Added CLI coverage showing dump unparse erases interface declarations and build accepts the fixture.
- Validation passed:
  - `cargo test -p ts2wasm-frontend parses_typescript_interface_declarations_as_erased_syntax -- --nocapture`
  - `cargo test -p ts2wasm-cli --test dump_cli dump_ast_unparse_erases_typescript_interface_declarations -- --nocapture`
  - `cargo test -p ts2wasm-cli --test dump_cli build_accepts_erasable_typescript_interface_declarations -- --nocapture`
  - `cargo fmt --all --check`
  - `scripts/manager fmt`
  - `cargo nextest run -p ts2wasm-frontend`
  - `cargo nextest run -p ts2wasm-cli --test dump_cli`
  - `scripts/manager update-issue-index --check`
  - `scripts/manager check-agent-state`
- Validation not clean due unrelated pre-existing local-report references:
  - `scripts/manager check-issue-health` failed because issue 052 and done issue 228 reference missing `reports/runs/...` paths. `reports/` is local/gitignored and those issue files are outside this assignment.
  - `scripts/manager check-repo-smoke` failed at the same `check-issue-health` step after shell syntax checks passed.
- Parent validation note: after syncing the referenced local `reports/runs/...` artifacts into the merge-review worktree, `scripts/manager check-issue-health` and `scripts/manager check-repo-smoke` passed.
- Issue 059 remains open. Type aliases, generics, decorators, private fields, broader parser-syntax diagnostic reduction, and reference-ramp evidence remain outside this slice.

2026-04-28 progress evidence (type-alias-erasure slice):

- Implemented a narrow parser-only TypeScript `type` / `export type` alias declaration erasure slice.
- Type alias declarations are consumed before AST construction, so dump `--ast --unparse` and build output omit aliases while preserving subsequent runtime statements.
- Alias bodies are skipped with balanced parentheses, brackets, and braces, covering simple aliases plus object/function type bodies.
- Added fixture `fixtures/basics-types/type-alias-erasure.ts`.
- Added frontend parser coverage for erased type aliases with nested object type braces and function type members.
- Added CLI coverage showing dump unparse erases type alias declarations and build accepts the fixture.
- Validation passed:
  - `cargo test -p ts2wasm-frontend parses_typescript_type_alias_declarations_as_erased_syntax -- --nocapture`
  - `cargo test -p ts2wasm-cli --test dump_cli dump_ast_unparse_erases_typescript_type_alias_declarations -- --nocapture`
  - `cargo test -p ts2wasm-cli --test dump_cli build_accepts_erasable_typescript_type_alias_declarations -- --nocapture`
  - `cargo fmt --all --check`
  - `scripts/manager fmt`
  - `cargo nextest run -p ts2wasm-frontend`
  - `cargo nextest run -p ts2wasm-cli --test dump_cli`
  - `scripts/manager update-issue-index --check`
  - `scripts/manager check-agent-state`
- Validation not clean due unrelated pre-existing local-report references:
  - `scripts/manager check-issue-health` failed because issue 052 and done issue 228 reference missing `reports/runs/...` paths. `reports/` is local/gitignored and those issue files are outside this assignment.
  - `scripts/manager check-repo-smoke` failed at the same `check-issue-health` step after shell syntax checks passed.
- Parent validation note: after syncing the referenced local `reports/runs/...` artifacts into the merge-review worktree, `scripts/manager check-issue-health` and `scripts/manager check-repo-smoke` passed.
- Issue 059 remains open. Generics, decorators, private fields, broader parser-syntax diagnostic reduction, and reference-ramp evidence remain outside this slice.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `...`

Validation result:

```text
command:
result:
date:
```

Remaining risks:

- none
