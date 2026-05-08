---
id: 3416
title: "Split multivar to namespace var merge diagnostic"
type: maintenance
area: frontend/semantics
class: done
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-08
completed: 2026-05-08
status: done
---

## Summary

Closed the generated `multivar` bucket after splitting the remaining semantic
parity gap into `issues/open/5436-report-mixed-exported-local-namespace-vars.md`.

## Problem

Fresh focused coverage shows the original import/export compiler blocker is
stale: `multivar.ts` now build-passes. TypeScript still reports TS2395 for
mixed exported/local declarations of `b2` in `namespace m2`, so the remaining
work is a semantic false-build-pass issue rather than this generated bucket.

Focused coverage:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/multivar.ts --detail --no-dashboard-data
result: executed=1, build_pass=1, unsupported=0, blocked=0
date: 2026-05-08
```

Focused triage:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/multivar.ts
result: BuildPass; TypeScript oracle reports TS2395 for merged declaration b2
date: 2026-05-08
```

## Evidence

Source context:

```ts
var a,b,c;
var x=1,y=2,z=3;

namespace m2 {
    export var a, b2: number = 10, b;
    var b2;
}
```

Compiler evidence:

```text
tokens: ok through multi-declarator vars, namespace m2, export var a/b2/b, and later var b2
ast/resolved: build succeeds; namespace members are erased, top-level vars and for-loop remain
visible symbols: a, x, namespace-local a/b2/b/m1/a2/b22/b222, class C/C2, d/a22/b22/c22/dn, foo, i, b5
```

TypeScript oracle evidence:

```text
TS2395 at line 8: Individual declarations in merged declaration 'b2' must be all exported or all local.
TS2395 at line 24: Individual declarations in merged declaration 'b2' must be all exported or all local.
```

Existing export-var owners 5283 and 5285 cover entry-module parser/module
boundaries and are not exact matches for this namespace merge diagnostic.

## Validation

Issue sync and health checks:

```text
python scripts/manager.py update-issue-index
git diff --check
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
```

Focused reference checks:

```text
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/multivar.ts --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/multivar.ts
```

Rust gates were not run because this slice only changes issue metadata.

## Completion evidence

Commits:

- filled by commit

Remaining risks:

- Implementation remains open in issue 5436.
