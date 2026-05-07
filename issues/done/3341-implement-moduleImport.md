---
id: 3341
title: "Implement Moduleimport"
type: spike
area: reference/triage
class: superseded
priority: P2
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-08
completed: 2026-05-08
status: done
---

## Summary

Closed this generated import/export bucket as superseded by the existing
implementation-ready namespace alias member diagnostic issue:
`issues/open/5397-report-missing-namespace-alias-member-diagnostic.md`.

## Problem

Fresh triage for the representative file build-passes in ts2wasm. TypeScript's
oracle reports TS2694 for an import-equals alias that targets a missing exported
namespace member.

## Current failure

Focused triage:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleImport.ts
```

Observed compiler result:

```text
BuildPass: ts2wasm build succeeded
```

Source shape:

```ts
namespace A.B.C {
    import XYZ = X.Y.Z;
    export function ping(x: number) {
        if (x > 0) XYZ.pong(x - 1);
    }
}

namespace X {
    import ABC = A.B.C;
    export function pong(x: number) {
        if (x > 0) ABC.ping(x - 1);
    }
}
```

Compiler evidence:

```text
tokens: ok through dotted namespaces, import-equals aliases, exported functions, and qualified calls
ast: ok
resolved: ok
```

TypeScript oracle evidence:

```text
TS2694: Namespace 'X' has no exported member 'Y'.
```

Coverage note:

```text
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleImport --detail --no-dashboard-data
```

The broad path filter matches both `moduleImport.ts` and the separate
`moduleImportedForTypeArgumentPosition.ts` bucket. `moduleImport.ts` is
`build_pass`; the second path remains tracked by its own generated issue 3342.

## Desired final state

Implement issue 5397. Do not implement directly from this generated bucket.

## Scope

In scope:

- [x] Confirm the representative `moduleImport.ts` no longer has a compiler blocker.
- [x] Record the remaining TypeScript TS2694 oracle diagnostic.
- [x] Supersede this bucket with the existing namespace alias missing-member issue.

Out of scope:

- Direct implementation from this generated bucket.
- Runtime namespace object emission.
- The separate `moduleImportedForTypeArgumentPosition.ts` bucket.

## Affected paths

Expected implementation owner:

- `crates/frontend/src/`
- `crates/ir/src/`
- focused namespace/import-alias diagnostic tests

## Acceptance criteria

- [x] Exact reproduction commands and diagnostics are recorded.
- [x] Superseding issue identified: `issues/open/5397-report-missing-namespace-alias-member-diagnostic.md`.
- [x] The generated bucket no longer remains as a stale blocked import/export issue.

## Validation

Required commands:

```sh
python scripts/manager.py update-issue-index
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
git diff --check
git diff --cached --check
```

Reference commands already run:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleImport --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleImport.ts
```

Not run:

- `cargo fmt --all --check` (issue lifecycle only; no Rust changes)
- `cargo nextest run` (issue lifecycle only; no Rust changes)

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] existing issue 5397 owns the semantic diagnostic

## Notes

Superseded by
`issues/open/5397-report-missing-namespace-alias-member-diagnostic.md`.

## Completion evidence

Commits:

- filled by commit

Validation result:

```text
command: python scripts/manager.py update-issue-index
result: pass
date: 2026-05-08

command: python scripts/manager.py update-issue-index --check
result: pass
date: 2026-05-08

command: python scripts/manager.py check-issue-health
result: pass
date: 2026-05-08

command: python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
result: pass
date: 2026-05-08

command: git diff --check
result: pass
date: 2026-05-08

command: git diff --cached --check
result: pass
date: 2026-05-08
```

Remaining risks:

- After issue 5397 lands, this reference may need re-triage for namespace alias
  runtime/lowering behavior.
