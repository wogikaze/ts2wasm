---
id: 5140
title: "Implement type alias object parsing before exported interface"
type: feature
area: frontend/syntax
class: done
priority: P1
depends_on: []
blocks: [074]
created: 2026-05-06
updated: 2026-05-06
---

## Summary

Implement the narrow parser support needed for `DeclarationErrorsNoEmitOnError.ts`: a semicolonless TypeScript object-type alias followed by an exported interface declaration.

## Problem

The TypeScript parser path does not accept an object type literal body in a type alias when the alias is not terminated with an explicit semicolon. The failing reference case is small and isolated, so it should be implemented as a focused parser slice instead of staying hidden inside issue 074.

## Current failure

~~Reproduction:~~

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/DeclarationErrorsNoEmitOnError.ts
```

~~Current diagnostic: `UnsupportedSyntax / parser-or-frontend-unsupported / unterminated TypeScript type alias declaration`~~

**Resolved**: BuildPass (ts2wasm build succeeded). The underlying fix was implemented in `f23bdc92` (issue-345: parse semicolonless type aliases) — the `skip_typescript_type_alias_body` already handles `{ ... }` object literals paired with ASI boundary detection for `export`/`interface` keywords.

## Completion

### Acceptance criteria

- [x] `mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/DeclarationErrorsNoEmitOnError.ts` no longer reports `unterminated TypeScript type alias declaration`. (Confirmed: BuildPass.)
- [x] A focused parser test covers `type T = { x : number }` followed by `export interface I { f: T; }`. (Added: `accepts_semicolonless_type_alias_object_followed_by_export_interface`.)
- [x] Malformed type aliases still reject with a precise parser diagnostic. (Added: `rejects_unterminated_type_alias_at_end_of_input` — checks `unterminated` diagnostic for `type T = ` at EoF.)
- [x] Issue 074 remains closed as a superseded generated bucket. (Unchanged.)

### Validation

```sh
cargo fmt --all --check       # pass
cargo test -p ts2wasm-frontend parser  # 135 passed, 0 failed
mise run reference-triage -- tsc .../DeclarationErrorsNoEmitOnError.ts  # BuildPass
```

### Commits

- `5140-implement-type-alias-object-before-export-interface`: Add focused parser tests for semicolonless type alias followed by export interface.

### Files changed

- `crates/frontend/src/parser/tests.rs`: Added two tests.

## False-done audit

Date: 2026-05-06

Classification: truly-done.

Audit result: retained in `issues/done/`. This issue has repo-local close evidence
(completion evidence with validation commands) or proper superseded classification
with child issues in `issues/open/`. The acceptance criteria documented in the issue
are satisfied by the repo-local evidence cited in the completion evidence section.

Future-work tracking: no untracked future-work item was identified in this issue
during this metadata/evidence audit.

