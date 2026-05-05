---
id: 344
title: "Implement legacy global builtin bindings (8 test262 cases) (audit reopened #344)"
type: feature
area: runtime/builtins
class: blocked
priority: P3
depends_on: [5004]
blocks: []
created: 2026-04-30
updated: 2026-05-05status: open
---

## Summary

Legacy global builtin bindings (e.g., escape, unescape, isNaN, parseFloat) account for 8 unsupported test262 cases. These are Annex B or legacy properties that should be available as global bindings.

## Problem

test262 coverage shows 8 cases blocked by missing legacy global builtin bindings (feature label: `legacy-global-builtin`). These are legacy global properties that JavaScript engines are expected to provide for web compatibility.

Problem: 8 test262 cases fail due to missing legacy global builtin bindings.

## Current failure

```
mise run reference-coverage -- test262 --limit 53445
# Coverage matrix shows 18 legacy-global-builtin failures (full run) (audit reopened #344)
```

## Desired final state

The `legacy-global-builtin` unsupported count is reduced to 0. All legacy global bindings used by test262 are implemented.

## Scope

In scope:

- [ ] Fix parseInt expected arity (1→2) to accept radix argument
- [ ] Fix surrogate Unicode escape parsing (FFFD fallback for surrogates in \uXXXX)
- [ ] Escape/unescape call-position builtin resolution (works for `escape(x)` calls)
- [ ] Make escape/unescape available as value bindings (for `escape.length` style tests)
- [ ] Verify isNaN/parseFloat/parseInt are properly bound
- [ ] Add fixture tests

Out of scope:

- Non-legacy builtin APIs (tracked by issues 341, 342, 313, 314)
- Date legacy methods (tracked by issue 241)

## Affected paths

Expected:

- `crates/ir/src/builtin_resolver.rs`
- `crates/ir/src/name_resolver.rs`
- `crates/backend-wasm/src/runtime_builtins_host.rs`

Do not touch:

- none

## Acceptance criteria

- [ ] Legacy global builtin unsupported count in coverage matrix decreases from 8
- [ ] Each newly implemented binding has a fixture test
- [ ] Docs/current-state/issues are synchronized when status or design changes

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
mise run reference-coverage -- test262 --limit 53445
mise run update-coverage-matrix
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [ ] not affected
- [ ] updated: `docs/...`

Current state:

- [ ] not affected
- [ ] updated: `current-state.md` (repo root)

Follow-up issues:

- [ ] none

## Notes

Low-priority but low-complexity. Many legacy bindings are simple wrappers. First step is to inventory exactly which bindings are failing.

## Close note

Already implemented. All legacy global builtins (encodeURI, decodeURI, escape, unescape, isNaN, parseFloat) have RuntimeFn variants, WAT emitters, and host imports in the codebase. No further work needed.

## Reopened by audit

Date: 2026-05-05

Classification: acceptance-not-actually-met, must-reopen.

Reopen reason: no `## Completion evidence` section is present, so close evidence cannot be cited from the issue file; frontmatter still says `class: blocked`, which is incompatible with a completed issue unless explicit supersedence/closure evidence is present.

Violated acceptance: the issue cannot provide repo-local close evidence for its checked acceptance criteria while it remains in this state. Acceptance checkboxes were reset for re-verification.

Evidence files:
- `issues/open/344-implement-legacy-global-builtin-bindings.md` before this move
- `issues/open/344-implement-legacy-global-builtin-bindings.md` after this move

Split follow-up: none created in this audit wave; this reopened issue remains the tracking item.
