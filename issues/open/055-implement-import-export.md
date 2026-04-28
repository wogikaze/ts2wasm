---
id: 055
title: "Umbrella: implement import and export"
type: feature
area: frontend/semantics
class: design-ready
priority: P1
depends_on: [232, 233, 234]
blocks: [232, 233, 234]
created: 2026-04-26
updated: 2026-04-28
---

## Summary

Track the ES module import/export rollout. The implementation work is split into smaller issues so parser, module graph, binding/emission, and execution verification can move independently.

## Problem

Static import/export currently produce issue-linked unsupported diagnostics. The original issue mixed parser representation, resolver/module graph behavior, relative module loading, export binding emission, and execution fixtures in one broad work item.

## Desired final state

`import { x } from './mod.js'` and `export { x }` work correctly.

## Scope

In scope:

- [ ] Coordinate split issues:
  - [x] 231 parser AST representation for static module declarations
  - [ ] 232 resolver/compiler module graph for local relative specifiers
  - [ ] 233 export binding lowering and backend module initialization
  - [ ] 234 execution fixtures and differential coverage

Out of scope:

- Dynamic import() (P2)
- require() (P2)

## Affected paths

Expected:

- `issues/done/231-parse-static-es-module-declarations.md`
- `issues/done/232-resolve-local-relative-es-module-graph.md`
- `issues/done/233-emit-static-es-module-bindings.md`
- `issues/open/234-cover-static-es-module-execution.md`

Do not touch:

- `docs/`

## Acceptance criteria

- [ ] Split issues exist with clear dependencies, scope, affected paths, acceptance criteria, and validation commands
- [ ] `issues/index.md` is regenerated and shows this umbrella as blocked by the split issues
- [ ] Remaining implementation work is tracked by the split issues instead of broad TODOs in this umbrella

## Validation

Required commands:

```sh
scripts/manager update-issue-index
scripts/manager update-issue-index --check
scripts/manager check-issue-health
scripts/manager check-agent-state
```

Impacted commands:

```sh
scripts/manager check-issue-index
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [ ] not affected

Current state:

- [ ] not affected

Follow-up issues:

- [x] created/updated: `issues/done/231-parse-static-es-module-declarations.md`
- [x] created/updated: `issues/done/232-resolve-local-relative-es-module-graph.md`
- [x] created/updated: `issues/done/233-emit-static-es-module-bindings.md`
- [x] created/updated: `issues/open/234-cover-static-es-module-execution.md`

## Notes

Keep this issue open as the umbrella until the split issues are complete. Do not use this umbrella for direct Rust implementation work.

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

2026-04-28 child slice `055-module-diagnostics-next-20260428T045453Z`:

- Added regression coverage for the already-classified static module diagnostic forms:
  - side-effect import: `import "./module-source";`
  - namespace import: `import * as mod from "./module-source";`
  - default import: `import value from "./module-source";`
  - re-export: `export * from "./module-source";`
- Added fixtures:
  - `fixtures/module-system/static-side-effect-import-unsupported.ts`
  - `fixtures/module-system/static-namespace-import-unsupported.ts`
  - `fixtures/module-system/static-default-import-unsupported.ts`
  - `fixtures/module-system/static-re-export-unsupported.ts`
- Focused validation:
  - `cargo nextest run -p ts2wasm-frontend rejects_side_effect_import_with_issue_linked_diagnostic rejects_namespace_import_with_issue_linked_diagnostic rejects_default_import_with_issue_linked_diagnostic rejects_re_export_with_issue_linked_diagnostic` passed.
  - `cargo nextest run -p ts2wasm-cli static_side_effect_import_reports_issue_055 static_namespace_import_reports_issue_055 static_default_import_reports_issue_055 static_re_export_reports_issue_055` passed.
  - Direct build checks for all four added fixtures failed as expected with `[UnsupportedSyntax]` and the relevant `issue-055` unsupported form message.

Remaining scope: parser representation for supported module declarations, module resolution, module loading, and execution fixtures remain open.

2026-04-28 child split `055-import-export-split-20260428T053058Z`:

- Split remaining scope into issues 231, 232, 233, and 234.
- This umbrella now depends on the split issues and remains open for progress tracking.

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
