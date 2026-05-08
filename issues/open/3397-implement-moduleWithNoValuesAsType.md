---
id: 3397
title: "Close moduleWithNoValuesAsType bucket to TS2709 variable annotation owner"
type: maintenance
area: frontend/semantics
class: superseded
priority: P1
depends_on: [432, 5411]
blocks: []
created: 2026-05-01
updated: 2026-05-08
completed: 2026-05-08
status: done
---

## Summary

Closed this generated import-export bucket as superseded by issue 5411. Fresh
coverage shows the reference file now build-passes in ts2wasm, while the
TypeScript oracle reports TS2709 diagnostics for namespace roots used as
variable annotation types.

## Problem

The original bucket listed one `moduleWithNoValuesAsType` reference file under
`import-export` without smart-triage evidence.

Fresh focused coverage reports:

```text
executed=1
build_pass=1
unsupported=0
```

## Current failure

Smart triage reports `BuildPass` for ts2wasm. The TypeScript oracle reports the
semantic diagnostics hidden by current erasure:

```text
TS2709: Cannot use namespace 'A' as a type. at var a: A
TS2709: Cannot use namespace 'B' as a type. at var b: B
TS2709: Cannot use namespace 'C' as a type. at var c: C
```

Source shape:

```ts
namespace A { }
var a: A; // error

namespace B {
    interface I {}
}
var b: B; // error

namespace C {
    namespace M {
        interface I {}
    }
}

var c: C; // error
```

The AST/resolved dumps retain only the erased variable declarations:

```text
Let a = Undefined
Let b = Undefined
Let c = Undefined
```

## Desired final state

This generated bucket remains closed. The TS2709 namespace-root variable
annotation diagnostic is owned by
`issues/open/5411-report-ts2709-for-namespace-variable-annotation.md`.

## Scope

Completed:

- [x] Re-ran focused coverage for the affected reference file.
- [x] Re-ran smart triage for the affected reference file.
- [x] Confirmed the current ts2wasm result is `BuildPass`.
- [x] Confirmed the oracle diagnostics match issue 5411.
- [x] Added an ownership note to issue 5411.

Out of scope:

- Direct implementation from this generated bucket.
- Class/interface heritage namespace-as-type diagnostics.
- Qualified namespace member annotation diagnostics.

## Affected paths

Referenced only:

- `reference/typescript/tests/cases/compiler/moduleWithNoValuesAsType.ts`

## Acceptance criteria

- [x] Current first diagnostic state is recorded.
- [x] Matching owner issue 5411 is identified.
- [x] This bucket is moved to `done/`.

## Validation

Commands run:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleWithNoValuesAsType.ts --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleWithNoValuesAsType.ts
```

Not run:

- `cargo fmt --all --check` and `cargo nextest run`; no Rust source changes.

## Notes

Issue 5411 already tracks TS2709-style diagnostics for same-file namespace roots
used as variable type annotations. This file is the same shape with empty and
nested namespace declarations.

## Completion evidence

Commits:

- `...`

Validation result:

```text
command: focused coverage and triage listed above
result: pass; ts2wasm BuildPass with oracle TS2709 for namespace A/B/C as types
date: 2026-05-08
```

Remaining risks:

- After issue 5411 lands, this file may expose narrower diagnostic parity around
  namespace contents, but that is outside this generated bucket.
