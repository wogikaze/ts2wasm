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
status: done
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

- [x] Resolve `String.prototype.matchAll` for string receivers with RegExp literal arguments carrying the `g` flag
- [x] Lower the supported shape to runtime behavior that can feed `[...matches]`
- [x] Represent each match result with the matched text and `index` / `input` properties needed by the reference case
- [x] Add or update a fixture under `fixtures/builtins-and-io/` with Node/iwasm differential coverage

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

- [x] `python scripts/manager.py reference-triage --format json tsc reference/typescript/tests/cases/compiler/stringMatchAll.ts` reports `BuildPass`
- [x] `python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/stringMatchAll.ts --detail` reports `build_pass=1` and `unsupported=0`
- [x] A Node/iwasm differential fixture covers `"matchAll".matchAll(/\w/g)` spread into an array and reads the first result's match text, `index`, and `input`
- [x] Existing string regexp fixtures still pass

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

- full `cargo nextest run`; focused affected tests passed

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

- `72b0f7d6` compiler: lower literal String.matchAll

Validation result:

```text
python scripts/manager.py reference-triage --format json tsc reference/typescript/tests/cases/compiler/stringMatchAll.ts
=> BuildPass

python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/stringMatchAll.ts --detail
=> build_pass=1, unsupported=0

cargo nextest run -p ts2wasm-cli string_match_all_fixture_matches_node_output_under_iwasm
=> pass

cargo nextest run -p ts2wasm-cli string_includes_fixture_matches_node_output_under_iwasm
=> pass

cargo nextest run -p ts2wasm-cli build_smoke_string_match_method
=> pass

cargo build -p ts2wasm-cli
=> pass

cargo fmt --all --check
=> fail: unrelated pre-existing crates/cli/tests/ir_lowering.rs debug assertion formatting

git diff --check
=> pass

date: 2026-05-06
```

Remaining risks:

- none

## False-done audit

Date: 2026-05-06

Classification: truly-done.

Audit result: retained in `issues/done/`. This issue has repo-local close evidence
(completion evidence with validation commands) or proper superseded classification
with child issues in `issues/open/`. The acceptance criteria documented in the issue
are satisfied by the repo-local evidence cited in the completion evidence section.

Future-work tracking: no untracked future-work item was identified in this issue
during this metadata/evidence audit.

