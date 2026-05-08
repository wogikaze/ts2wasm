---
id: 3399
title: "Close moduleWithValuesAsType bucket to TS2709 variable annotation owner"
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
TypeScript oracle reports TS2709 for a namespace root used as a variable type
annotation.

## Problem

The original bucket listed one `moduleWithValuesAsType` reference file under
`import-export` without smart-triage evidence.

Fresh focused coverage reports:

```text
executed=1
build_pass=1
unsupported=0
```

## Current failure

Smart triage reports `BuildPass` for ts2wasm. The TypeScript oracle reports:

```text
TS2709: Cannot use namespace 'A' as a type. at var a: A
```

Source shape:

```ts
namespace A {
    var b = 1;
}

var a: A; // no error
```

Compiler evidence:

```text
tokens: ok through namespace body var b and var a: A
ast/resolved: Let a = Undefined; namespace body and type annotation are erased
TypeScript oracle: TS2709 at the A annotation
```

## Desired final state

This generated bucket remains closed. The TS2709 namespace-root variable
annotation diagnostic is owned by
`issues/open/5411a-report-ts2709-for-namespace-variable-annotation.md`.

## Scope

Completed:

- [x] Re-ran focused coverage for the affected reference file.
- [x] Re-ran smart triage for the affected reference file.
- [x] Confirmed the current ts2wasm result is `BuildPass`.
- [x] Confirmed the oracle diagnostic matches issue 5411.
- [x] Added an ownership note to issue 5411.

Out of scope:

- Direct implementation from this generated bucket.
- General namespace body lowering.

## Affected paths

Referenced only:

- `reference/typescript/tests/cases/compiler/moduleWithValuesAsType.ts`

## Acceptance criteria

- [x] Current diagnostic state is recorded.
- [x] Matching owner issue 5411 is identified.
- [x] This bucket is moved to `done/`.

## Validation

Commands run:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleWithValuesAsType.ts --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleWithValuesAsType.ts
```

Not run:

- `cargo fmt --all --check` and `cargo nextest run`; no Rust source changes.

## Notes

Issue 5411 already tracks TS2709-style diagnostics for same-file namespace roots
used as variable type annotations. This file is the same shape with a
namespace body that contains a value declaration.

## Completion evidence

Commits:

- `...`

Validation result:

```text
command: focused coverage and triage listed above
result: pass; ts2wasm BuildPass with oracle TS2709 for namespace A as a type
date: 2026-05-08
```

Remaining risks:

- After issue 5411 lands, this file may expose namespace-body semantic parity,
  but that is outside this generated bucket.
