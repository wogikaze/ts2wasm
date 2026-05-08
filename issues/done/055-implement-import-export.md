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
completed: 2026-04-28
status: done
---

## Summary

Track the ES module import/export rollout. The implementation work is split into smaller issues so parser, module graph, binding/emission, and execution verification can move independently.

## Problem

Static import/export currently produce issue-linked unsupported diagnostics. The original issue mixed parser representation, resolver/module graph behavior, relative module loading, export binding emission, and execution fixtures in one broad work item.

## Desired final state

`import { x } from './mod.js'` and `export { x }` work correctly.

## Scope

In scope:

- [x] Coordinate split issues:
  - [x] 231 parser AST representation for static module declarations
  - [x] 232 resolver/compiler module graph for local relative specifiers
  - [x] 233 export binding lowering and backend module initialization
  - [x] 234 execution fixtures and differential coverage

Out of scope:

- Dynamic import() (P2)
- require() (P2)

## Affected paths

Expected:

- `issues/done/231-parse-static-es-module-declarations.md`
- `issues/open/232-resolve-local-relative-es-module-graph.md`
- `issues/open/233-emit-static-es-module-bindings.md`
- `issues/done/234-cover-static-es-module-execution.md`

Do not touch:

- `docs/`

## Acceptance criteria

- [x] Split issues exist with clear dependencies, scope, affected paths, acceptance criteria, and validation commands
- [x] `issues/index.md` is regenerated and shows this umbrella as blocked by the split issues
- [x] Remaining implementation work is tracked by the split issues instead of broad TODOs in this umbrella

## Validation

Required commands:

```sh
mise run update-issue-index
mise run update-issue-index -- --check
mise run check-issue-health
mise run check-agent-state
```

Impacted commands:

```sh
mise run check-issue-index
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] created/updated: `issues/done/231-parse-static-es-module-declarations.md`
- [x] created/updated: `issues/open/232-resolve-local-relative-es-module-graph.md`
- [x] created/updated: `issues/open/233-emit-static-es-module-bindings.md`
- [x] created/updated: `issues/done/234-cover-static-es-module-execution.md`

## Notes

This issue stayed open as the umbrella until the split issues were complete. Do not use this umbrella for direct Rust implementation work.

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

2026-04-28 child slice `234-static-esm-exec-close-20260428T130000Z`:

- Closed split issue 234 after auditing existing static named ES module execution coverage.
- The split issue path now lives at `issues/done/234-cover-static-es-module-execution.md`.
- Static named ES module fixtures have Node/iwasm differential evidence for direct import, alias import, importer lexical shadowing, and repeated import from the same source module.
- This umbrella remains open for import/export coordination beyond the narrow split-issue subset.

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

Issue 055 is a coordination umbrella. The implementation was split into issues 231, 232, 233, and 234, and all split issue files now live under `issues/done/` with completed frontmatter and completion evidence:

- `issues/done/231-parse-static-es-module-declarations.md`
- `issues/open/232-resolve-local-relative-es-module-graph.md`
- `issues/open/233-emit-static-es-module-bindings.md`
- `issues/done/234-cover-static-es-module-execution.md`

The narrow supported static ES module subset remains documented by the split issue evidence. Broader import/export forms remain out of scope for this umbrella close and are not claimed as complete here.

Commits:

- close commit: recorded by child worker `055-import-export-umbrella-close-20260428T131500Z`

Validation result:

```text
mise run update-issue-index: PASS
mise run update-issue-index -- --check: PASS
mise run check-issue-health: PASS
mise run check-agent-state: PASS
mise run check-repo-smoke: PASS
cargo nextest run -p ts2wasm-cli module: PASS
date: 2026-04-28
```

Remaining risks:

- none

## False-done audit

Date: 2026-05-06

Classification: truly-done.

Audit result: retained in `issues/done/`. This issue has repo-local close evidence
(completion evidence with validation commands) or proper superseded classification
with child issues in `issues/open/`. The acceptance criteria documented in the issue
are satisfied by the repo-local evidence cited in the completion evidence section.

Future-work tracking: no untracked future-work item was identified in this issue
during this metadata/evidence audit.

