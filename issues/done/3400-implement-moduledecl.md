---
id: 3400
title: "Close moduledecl bucket to ambient namespace value owner"
type: maintenance
area: frontend/resolver
class: superseded
priority: P1
depends_on: [432, 5370]
blocks: []
created: 2026-05-01
updated: 2026-05-08
completed: 2026-05-08
status: done
---

## Summary

Closed this generated import-export bucket as superseded by issue 5370. Fresh
triage shows the broad `moduledecl` reference advances through many namespace
declarations, then fails because an ambient namespace root is not resolver
visible for qualified value access.

## Problem

The original bucket listed one `moduledecl` reference file under
`import-export` without smart-triage evidence.

Fresh focused coverage reports:

```text
executed=1
build_pass=0
unsupported=1
unsupported_diagcodes=UnresolvedName:1
unsupported_features=name-resolution:1
```

## Current failure

Smart triage reports:

```text
UnresolvedName: unresolved name: `mAmbient`
```

Representative source shape:

```ts
declare namespace mAmbient {
    class C {
        public myProp: number;
    }

    function foo(): C;
    var aVar: C;

    namespace m3 {
        class C {
            public myProp: number;
        }

        function foo(): C;
        var aVar: C;
    }
}

function foo() {
    return mAmbient.foo();
}

var cVar = new mAmbient.C();
var aVar = mAmbient.aVar;
var m3cVar = new mAmbient.m3.C();
var m3aVar = mAmbient.m3.aVar;
```

Compiler evidence:

```text
tokens/ast: ok through declare namespace mAmbient and later qualified value accesses
resolved: UnresolvedName for mAmbient during resolve_names
TypeScript oracle: later strict-property initialization diagnostics, not this resolver blocker
```

## Desired final state

This generated bucket remains closed. The ambient namespace qualified value
access blocker is owned by
`issues/open/5370-bind-ambient-namespace-declarations-for-qualified-value-access.md`.

## Scope

Completed:

- [x] Re-ran focused coverage for the affected reference file.
- [x] Re-ran smart triage for the affected reference file.
- [x] Confirmed the current first blocker is ambient namespace root resolution.
- [x] Added an ownership note to issue 5370.

Out of scope:

- Direct implementation from this generated bucket.
- Strict property initialization diagnostics later reported by TypeScript.
- Full declaration emit or `.d.ts` parity for the broad module declaration file.

## Affected paths

Referenced only:

- `reference/typescript/tests/cases/compiler/moduledecl.ts`

## Acceptance criteria

- [x] Current first diagnostic state is recorded.
- [x] Matching owner issue 5370 is identified.
- [x] This bucket is moved to `done/`.

## Validation

Commands run:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduledecl.ts --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduledecl.ts
```

Not run:

- `cargo fmt --all --check` and `cargo nextest run`; no Rust source changes.

## Notes

Issue 5370 already tracks binding same-file ambient `declare namespace`
declarations as resolver-visible namespace values while preserving erasure.
This file is a broader representative of the same `mAmbient.*` resolver
boundary.

## Completion evidence

Commits:

- `...`

Validation result:

```text
command: focused coverage and triage listed above
result: pass; first blocker is UnresolvedName for ambient namespace root mAmbient
date: 2026-05-08
```

Remaining risks:

- After issue 5370 lands, this broad reference may expose additional namespace,
  declaration emit, or strict-property diagnostics.
