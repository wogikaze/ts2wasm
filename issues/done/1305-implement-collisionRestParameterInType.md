---
id: 1305
title: "Implement Collisionrestparameterintype"
type: spike
area: frontend/syntax
class: done
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-07
completed: 2026-05-07
status: done
---

## Summary

Triage collisionRestParameterInType across 1 reference case and close it as
superseded by the existing object type literal rest-parameter signature issue.

## Problem

Reference test results previously showed 1 case failing in directory
`collisionRestParameterInType` with diagnostics: parser-syntax. Fresh triage
shows the current blocker is `UnsupportedTypeScriptSyntax` for an unterminated
object type literal annotation containing call, construct, method, and
function-valued property signatures with rest parameters.

Problem: object type literal signature members with rest parameters are not
parsed as complete TypeScript type annotations.

## Current failure

Representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/collisionRestParameterInType.ts
```

Coverage window:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/collisionRestParameterInType.ts --detail --no-dashboard-data
```

## Desired final state

This generated bucket is closed because the current observable blocker is
owned by
`issues/open/5336-parse-object-type-literal-signatures-with-rest-parameters.md`.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Supersede this bucket with the existing object type literal rest-parameter signature issue
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence

Out of scope:

- Direct implementation from this generated bucket
- Broad multi-feature fixes without child issue split

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/cli/src/`
- `fixtures/`
- `scripts/run/reference-triage.py`

Do not touch:

- unrelated runtime/backend code unless `reference-triage` proves the failure is not frontend-owned

## Acceptance criteria

- [x] Duplicate candidates below are confirmed and this issue is superseded
- [x] Superseding issue 5336 owns object type literal signature members with rest parameters
- [x] This issue preserves failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Completion evidence names the exact reference path and diagnostic change

## Validation

Required commands:

```sh
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
git diff --check
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/collisionRestParameterInType.ts --detail --no-dashboard-data
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/collisionRestParameterInType.ts
```

Not run:

- `cargo fmt --all --check`; issue metadata only, no Rust code changed
- `cargo nextest run`; issue metadata only, no implementation changed

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none; superseded by `issues/open/5336-parse-object-type-literal-signatures-with-rest-parameters.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/collisionRestParameterInType.ts`

## Duplicate detection

- `issues/open/5336-parse-object-type-literal-signatures-with-rest-parameters.md` - exact owner for object type literal call/construct/method/property signatures with rest parameters
- `issues/open/5201-parse-object-type-literal-call-signatures.md` - related call-signature-only parser issue, narrower than this mixed signature shape
- `issues/open/5257-parse-object-type-literal-construct-signatures.md` - related construct-signature-only parser issue, narrower than this mixed signature shape

## Smart triage

Fresh triage shows this generated parser-syntax bucket is currently blocked by
object type literal signature parsing already tracked by issue 5336.

### Smart triage: Triage parser syntax: collisionRestParameterInType

- Issue class: `triage-needed`
- Feature label: `parser-syntax`
- Diagnostic: `UnsupportedTypeScriptSyntax` / `unsupported-feature-boundary`
- Path: `reference/typescript/tests/cases/compiler/collisionRestParameterInType.ts`

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/collisionRestParameterInType.ts
```

Coverage:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/collisionRestParameterInType.ts --detail --no-dashboard-data
```

Coverage result:

```text
executed=1
build_pass=0
semantic_pass=0
unsupported=1
blocked=0
unsupported_diagcodes=UnsupportedSyntax:1
unsupported_features=unknown-unsupported:1
semantic_enabled=0
```

Source context:

```ts
var v2: {
    (_i: number, ...restParameters);
    new (_i: number, ...restParameters);
    foo(_i: number, ...restParameters);
    prop: (_i: number, ...restParameters) => void;
}
```

Compiler evidence:

```text
tokens: ok; includes type literal braces, call signature, new construct signature, method signature, function-valued property signature, and DotDotDot rest parameters
ast: fails with UnsupportedTypeScriptSyntax unterminated TypeScript type annotation at 403..404
resolved: same parser boundary
visible symbols: bindings v1 and v2 before failure
```

TypeScript oracle evidence:

```text
ok: true
diagnostics: []
```

Superseded by:

- `issues/open/5336-parse-object-type-literal-signatures-with-rest-parameters.md`

## Completion evidence

Commits:

- Superseded by `issues/open/5336-parse-object-type-literal-signatures-with-rest-parameters.md`.

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/collisionRestParameterInType.ts
result: pass on the main checkout with TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm; parser-syntax object type literal rest-parameter signature blocker superseded by issue 5336
date: 2026-05-07

command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/collisionRestParameterInType.ts --detail --no-dashboard-data
result: pass on the main checkout with TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm; UnsupportedSyntax/unknown-unsupported
date: 2026-05-07
```

Remaining risks:

- After issue 5336 lands, this reference may expose later type-only
  rest-parameter semantics that need a separate child issue.
