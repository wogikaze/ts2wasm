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
completed: 2026-04-28
status: done
---

## Summary

Implement the remaining logical assignment target forms that require temporary reference storage to preserve JavaScript single-evaluation semantics.

## Problem

Issue 228 implemented logical assignment operators for identifiers, static members on identifier receivers, and string-literal computed members on identifier receivers. The remaining target forms are intentionally diagnosed because they need a reference-temporary design before the compiler can preserve single evaluation of object and key expressions.

## Desired final state

Dynamic computed and non-identifier receiver logical assignment targets either execute with ECMAScript single-evaluation semantics or report a narrower issue-linked diagnostic for any explicitly unsupported subset.

## Scope

In scope:

- [x] Implement or design lowering for non-identifier member receivers such as `getObj().value ||= rhs()`.
- [x] Implement or design lowering for dynamic computed keys such as `target[key] &&= rhs()`.
- [x] Preserve one evaluation of object and key expressions and short-circuit RHS behavior.
- [x] Add Node/iwasm differential fixtures for object, key, and RHS evaluation counts.

Out of scope:

- [x] Broad assignment-target validation unrelated to logical assignment.
- [x] Annex B `[[IsHTMLDDA]]` browser compatibility, tracked separately by issue 237.

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

- [x] `fixtures/core-semantics/logical-assignment-computed-member.ts` replaces the previous unsupported receiver/key fixture with positive regression coverage.
- [x] Dynamic computed logical assignment keys are evaluated exactly once.
- [x] Non-identifier receiver logical assignment targets are evaluated exactly once.
- [x] RHS evaluation still follows `&&=`, `||=`, and `??=` short-circuit semantics.
- [x] `cargo fmt --all --check` and `cargo nextest run` pass.

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

- [x] not affected

Current state:

- [x] not affected beyond issue evidence and regression fixtures

Follow-up issues:

- [x] none

## Notes

Split from issue 228 close audit. Existing diagnostics point to this issue for target forms that need temporary reference storage.

2026-04-28 progress: dynamic computed logical-assignment keys on identifier receivers
(`target[key] ||= rhs()`, `&&=`, `??=`) now lower through a computed
property assignment path that evaluates the key expression before the
short-circuit branch and reuses the stored key value for any write. Regression
coverage in `fixtures/core-semantics/logical-assignment-index.ts` records key
and RHS side effects for the supported dynamic-key slice. At that point,
non-identifier receivers still remained issue-linked unsupported coverage.

2026-04-28 progress: static member logical-assignment targets with
non-identifier receivers now lower through an explicit receiver-temporary path
(`getObj().value ||= rhs()`, `&&=`, `??=`). The backend evaluates the receiver
once into a rooted temporary before the property read and reuses that temporary
for any short-circuited write. Regression coverage in
`fixtures/core-semantics/logical-assignment-member.ts` records receiver and RHS
side-effect markers for skip/run branches. At that point, dynamic computed keys
on non-identifier receivers still remained issue-linked unsupported coverage.

2026-04-28 progress: dynamic computed logical-assignment keys on
non-identifier receivers now lower through a combined receiver/key temporary
path (`getObj()[key()] ||= rhs()`, `&&=`, `??=`). The backend evaluates the
receiver once into a rooted temporary, evaluates the key once into a separate
rooted temporary, uses the stored key for both the read and any short-circuited
write, and keeps RHS evaluation gated by the logical assignment operator.
Regression coverage in
`fixtures/core-semantics/logical-assignment-computed-member.ts` records
receiver, key, and RHS side-effect markers for skip/run branches.

## Completion evidence

Issue 236 is closed by the cumulative logical-assignment target-form work:

Commits:

- `43d9b04dffcbbe636e461c4b75ab741576d5a2a7` - static member logical assignment on non-identifier receivers.
- `2b963e4e766db6864af2f644a704cd2864d3644e` - dynamic computed logical assignment on non-identifier receivers.

Validation result:

```text
cargo fmt --all --check
result: pass
date: 2026-04-28

cargo nextest run -E 'test(logical_assignment)'
result: pass; 7 tests passed
date: 2026-04-28

node fixtures/core-semantics/logical-assignment-member.ts
node fixtures/core-semantics/logical-assignment-index.ts
node fixtures/core-semantics/logical-assignment-computed-member.ts
result: pass; direct Node fixture commands produced expected side-effect traces
date: 2026-04-28

cargo run -q -p ts2wasm-cli -- build fixtures/core-semantics/logical-assignment-member.ts -o /tmp/ts2wasm-issue236-member.wasm && iwasm /tmp/ts2wasm-issue236-member.wasm
cargo run -q -p ts2wasm-cli -- build fixtures/core-semantics/logical-assignment-index.ts -o /tmp/ts2wasm-issue236-index.wasm && iwasm /tmp/ts2wasm-issue236-index.wasm
cargo run -q -p ts2wasm-cli -- build fixtures/core-semantics/logical-assignment-computed-member.ts -o /tmp/ts2wasm-issue236-computed-member.wasm && iwasm /tmp/ts2wasm-issue236-computed-member.wasm
result: pass; iwasm outputs matched Node for receiver/key/RHS side-effect coverage
date: 2026-04-28

scripts/manager check-agent-state
scripts/manager check-issue-health
scripts/manager check-repo-smoke
result: pass
date: 2026-04-28

cargo nextest run
result: pass; 378 passed, 4 skipped in child validation
date: 2026-04-28
```

Remaining risks:

- Broad assignment target validation remains out of scope.
- Annex B `[[IsHTMLDDA]]` compatibility remains tracked by issue 237.
