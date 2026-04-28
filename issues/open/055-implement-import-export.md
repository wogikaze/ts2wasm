---
id: 055
title: "Implement import and export"
type: feature
area: frontend/semantics
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-04-26
updated: 2026-04-28
---

## Summary

Implement ES6 import/export static module system.

## Problem

Import/export are not implemented. They are essential for modular code organization.

## Desired final state

`import { x } from './mod.js'` and `export { x }` work correctly.

## Scope

In scope:

- [ ] Add import syntax to lexer/parser
- [ ] Add export syntax to lexer/parser
- [ ] Implement module resolution
- [ ] Implement module loading
- [ ] Add fixtures for import/export behavior

Out of scope:

- Dynamic import() (P2)
- require() (P2)

## Affected paths

Expected:

- `crates/cli/src/lib.rs` (lexer/parser)
- `crates/backend-wasm/src/` (module loading)
- `fixtures/`

Do not touch:

- `docs/`

## Acceptance criteria

- [ ] import parses correctly
- [ ] export parses correctly
- [ ] Module resolution works correctly
- [ ] Fixtures cover import/export behavior
- [ ] No regression in existing fixtures

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
cargo run -p ts2wasm-cli -- build fixtures/module-test.ts -o /tmp/test.wasm
iwasm /tmp/test.wasm
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [ ] not affected

Current state:

- [ ] not affected

Follow-up issues:

- [ ] none

## Notes

This is a major feature requiring module system design.

## Progress evidence

2026-04-28 child slice:

- Added parser diagnostics for unsupported static import/export entry forms with `issue-055` messages instead of generic expression failures.
- Added regression fixtures:
  - `fixtures/module-system/static-named-import-unsupported.ts`
  - `fixtures/module-system/static-named-export-unsupported.ts`
- Focused validation:
  - `cargo nextest run -p ts2wasm-frontend rejects_static_import_with_issue_linked_diagnostic rejects_named_export_with_issue_linked_diagnostic` passed.
  - `cargo nextest run -p ts2wasm-cli static_named_import_reports_issue_055 static_named_export_reports_issue_055` passed.
  - `cargo run -q -p ts2wasm-cli -- build fixtures/module-system/static-named-import-unsupported.ts -o /tmp/ts2wasm-055-import.wasm` failed as expected with `[UnsupportedSyntax] issue-055: unsupported named import; module resolution and loading are not implemented`.
  - `cargo run -q -p ts2wasm-cli -- build fixtures/module-system/static-named-export-unsupported.ts -o /tmp/ts2wasm-055-export.wasm` failed as expected with `[UnsupportedSyntax] issue-055: unsupported named export; module resolution and loading are not implemented`.

Remaining scope: parser representation for supported module declarations, module resolution, module loading, and execution fixtures remain open.

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
