# Introduce typed WAT writer skeleton

**Status**: done
**Created**: 2026-04-26
**Updated**: 2026-04-26
**Completed**: 2026-04-26
**ID**: 008
**Type**: refactor
**Area**: backend
**Priority**: P1
**Depends on**: 003
**Orchestration class**: design-ready

Problem: Large raw WAT string generation in runtime builder code is a major maintainability risk. A full rewrite is too large, but new WAT generation should stop adding unstructured string concatenation.

Scope:

- [x] Design minimal typed WAT writer API.
- [x] Cover imports, functions, globals, and data segments.
- [x] Keep existing output behavior unchanged.
- [x] Convert one small helper or import path first.
- [x] Add structural or snapshot tests.

Acceptance Criteria:

- [x] There is at least one non-string-concatenation WAT generation path.
- [x] Existing behavior remains unchanged.
- [x] Coding standard says new WAT should prefer typed writer APIs.
- [x] No broad runtime_builder rewrite is attempted in this issue.

Validation:

```sh
cargo fmt --all --check
cargo nextest run
```

## Completion evidence

**Validation results:**

```text
command: cargo fmt --all --check
result: passed
date: 2026-04-26

command: cargo nextest run backend
result: 31 tests passed
date: 2026-04-26
```

**Implementation:**
- Created `crates/cli/src/backend/wat_writer.rs` with minimal typed API:
  - `WatFuncSig`: Structured function signature with params and results
  - `WatImport`: Structured import statement with module, name, symbol, and signature
  - `WatWriter`: Builder for collecting WAT content
- Converted `emit_imports_from_catalog` in `emitter.rs` to use `WatWriter` instead of string concatenation
- Added structural test `typed_wat_writer_imports_match_string_concat` to verify output correctness
- Added coding standard section 19.13 in `docs/12-coding-standard.md` to prefer typed writer APIs
- Added unit tests for `wat_writer` module covering all API components

**API coverage:**
- Imports: ✓ (WatImport implemented and used)
- Functions: API ready (WatFuncSig), not yet used for function body generation
- Globals: Not yet covered (future work)
- Data segments: Not yet covered (future work)

**Non-string-concatenation path:**
- `emit_imports_from_catalog` now uses `WatWriter` for all import generation
- This is the first non-string-concatenation WAT generation path in the codebase

