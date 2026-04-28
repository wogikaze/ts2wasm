---
id: 236
title: "Complete logical assignment target forms"
type: feature
area: frontend/semantics
class: design-ready
priority: P2
depends_on: []
blocks: []
created: 2026-04-28
updated: 2026-04-28
---

## Summary

Implement the remaining logical assignment target forms that require temporary reference storage to preserve JavaScript single-evaluation semantics.

## Problem

Issue 228 implemented logical assignment operators for identifiers, static members on identifier receivers, and string-literal computed members on identifier receivers. The remaining target forms are intentionally diagnosed because they need a reference-temporary design before the compiler can preserve single evaluation of object and key expressions.

## Desired final state

Dynamic computed and non-identifier receiver logical assignment targets either execute with ECMAScript single-evaluation semantics or report a narrower issue-linked diagnostic for any explicitly unsupported subset.

## Scope

In scope:

- [ ] Implement or design lowering for non-identifier member receivers such as `getObj().value ||= rhs()`.
- [ ] Implement or design lowering for dynamic computed keys such as `target[key] &&= rhs()`.
- [ ] Preserve one evaluation of object and key expressions and short-circuit RHS behavior.
- [ ] Add Node/iwasm differential fixtures for object, key, and RHS evaluation counts.

Out of scope:

- Broad assignment-target validation unrelated to logical assignment.
- Annex B `[[IsHTMLDDA]]` browser compatibility, tracked separately by issue 237.

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/ir/src/`
- `crates/backend-wasm/src/`
- `crates/cli/tests/`
- `fixtures/core-semantics/`

Do not touch:

- `docs/`

## Acceptance criteria

- [ ] `fixtures/core-semantics/logical-assignment-member-unsupported.ts` is replaced or narrowed by positive regression fixtures.
- [ ] Dynamic computed logical assignment keys are evaluated exactly once.
- [ ] Non-identifier receiver logical assignment targets are evaluated exactly once.
- [ ] RHS evaluation still follows `&&=`, `||=`, and `??=` short-circuit semantics.
- [ ] `cargo fmt --all --check` and `cargo nextest run` pass.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
cargo nextest run -E 'test(logical_assignment)'
node fixtures/core-semantics/logical-assignment-member.ts
node fixtures/core-semantics/logical-assignment-index.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [ ] not affected

Current state:

- [ ] updated when behavior changes

Follow-up issues:

- [ ] none

## Notes

Split from issue 228 close audit. Existing diagnostics point to this issue for target forms that need temporary reference storage.

2026-04-28 progress: dynamic computed logical-assignment keys on identifier receivers
(`target[key] ||= rhs()`, `&&=`, `??=`) now lower through a computed
property assignment path that evaluates the key expression before the
short-circuit branch and reuses the stored key value for any write. Regression
coverage in `fixtures/core-semantics/logical-assignment-index.ts` records key
and RHS side effects for the supported dynamic-key slice. Non-identifier
receivers remain issue-linked unsupported coverage in
`fixtures/core-semantics/logical-assignment-member-unsupported.ts`.

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
