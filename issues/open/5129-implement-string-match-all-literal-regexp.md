---
id: 5129
title: "Implement String.prototype.matchAll literal RegExp lowering"
type: feature
area: runtime/builtins
class: implementation-ready
priority: P1
depends_on: []
blocks: []
parent: 4291
created: 2026-05-06
updated: 2026-05-06
---

## Summary

Implement the narrow `String.prototype.matchAll` slice needed by the tsc
reference case `stringMatchAll.ts`.

The first supported shape should be a string receiver with a RegExp literal
argument that has the `g` flag, producing match records that can be spread into
an array and expose at least match text, `index`, and `input`.

## Problem

`reference/typescript/tests/cases/compiler/stringMatchAll.ts` fails during
builtin resolution:

```text
UnsupportedBuiltin: String.prototype.matchAll is not supported in this milestone at 38..64
```

Problem: `String.prototype.matchAll` is rejected before lowering/runtime, so the
reference case cannot build and no Node/iwasm parity fixture covers the behavior.

## Current failure

```sh
python scripts/manager.py reference-triage --format json tsc reference/typescript/tests/cases/compiler/stringMatchAll.ts
```

Current result:

```text
diagnostic.code=UnsupportedBuiltin
diagnostic.feature_label=builtin-api
source line 3: const matches = "matchAll".matchAll(/\w/g);
source line 4: const array = [...matches];
source line 5: const { index, input } = array[0];
```

Focused coverage:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/stringMatchAll.ts --detail
```

Current result:

```text
executed=1
build_pass=0
unsupported=1
unsupported_diagcodes=UnsupportedSyntax:1
unsupported_features=unknown-unsupported:1
```

## Desired final state

The reference case builds without `UnsupportedBuiltin`, and a small
Node/iwasm differential fixture proves observable behavior for the supported
literal-regexp slice.

## Scope

In scope:

- [ ] Resolve `String.prototype.matchAll` for string receivers with RegExp literal arguments carrying the `g` flag
- [ ] Lower the supported shape to runtime behavior that can feed `[...matches]`
- [ ] Represent each match result with the matched text and `index` / `input` properties needed by the reference case
- [ ] Add or update a fixture under `fixtures/builtins-and-io/` with Node/iwasm differential coverage

Out of scope:

- Full RegExp engine behavior beyond already-supported literal regexp subset
- Non-global regexp validation beyond a clear unsupported diagnostic
- `Symbol.matchAll` direct calls
- Lazy iterator identity/prototype fidelity

## Affected paths

Expected:

- `crates/ir/src/lowered/program_builtins.rs`
- `crates/ir/src/lowered/resolver_expr.rs`
- `crates/backend-wasm/src/`
- `fixtures/builtins-and-io/`
- `crates/cli/tests/common/m2_node_diff_fixture_tests.rs`

Do not touch:

- capability manifest unless a host import is added
- docs final-state files unless the implementation changes documented support policy

## Acceptance criteria

- [ ] `python scripts/manager.py reference-triage --format json tsc reference/typescript/tests/cases/compiler/stringMatchAll.ts` reports `BuildPass`
- [ ] `python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/stringMatchAll.ts --detail` reports `build_pass=1` and `unsupported=0`
- [ ] A Node/iwasm differential fixture covers `"matchAll".matchAll(/\w/g)` spread into an array and reads the first result's match text, `index`, and `input`
- [ ] Existing string regexp fixtures still pass

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
python scripts/manager.py reference-triage --format json tsc reference/typescript/tests/cases/compiler/stringMatchAll.ts
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/stringMatchAll.ts --detail
cargo nextest run -p ts2wasm-cli string_match_all_fixture_matches_node_output_under_iwasm string_match_fixture_matches_node_output_under_iwasm
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none

## Notes

Smart triage showed parser and AST support already recognize the RegExp literal
and spread array syntax; the current blocker is builtin support.

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
