---
id: 5038
title: "[compiler] Harden module graph resolution and diagnostics (audit reopened #5038)"
type: feature
area: cli
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-03
updated: 2026-05-06
status: done
---

## Summary

local `.ts/.js` 以外、missing module、cycle、default/named export mismatch の診断と graph order をより厳密にする。

## Problem

現在の module graph resolution は正常系に重点を置いており、エラーケース（missing module、cycle、export mismatch）の診断が弱い。

## Current failure

不正な module graph が検出されず、実行時エラーになる。

## Desired final state

module graph resolution が以下を確実に診断する：missing module、cycle、default/named export mismatch、不正な graph order。

## Scope

In scope:
- [x] missing module 診断の追加 (existing)
- [x] cycle 検出と診断 (implemented + surfaced as build errors)
- [x] default/named export mismatch 診断 (existing)
- [x] graph order 検証 (validate_init_order)

Out of scope:
- [x] 外部 package resolution (out of scope)
- [x] dynamic import (out of scope)

## Affected paths

Expected:
- `crates/cli/`
- `crates/ir/`

## Acceptance criteria

- [x] 各エラーケースの診断 test fixture が追加される
- [x] 診断が source span 付きで報告される

## Validation

```sh
cargo fmt --all --check
cargo nextest run
```

## Docs / current-state / issue sync

Final-state docs:
- [x] not affected

Current state:
- [x] not affected

Follow-up issues:
- [x] none

## Completion evidence

### Implementation commit

`158e2f42` feat(compiler): harden module graph resolution with cycle detection (5038)

### What was implemented

- Cycle detection during module graph construction (`ModuleGraphBuilder` in `crates/compiler/src/module_graph.rs`): when a resolved dependency path is currently being visited, a `Diagnostic` with code `issue-5038` is collected
- Non-fatal cycle diagnostics (ES modules support cyclic imports via live bindings, so graph construction continues)
- Public `cycle_diagnostics()` accessor on `ModuleGraph`
- `validate_cycle_free()` validation function that surfaces first cycle diagnostic as a hard build error
- Tests: `detects_direct_cycle_with_diagnostic`, `detects_self_import_cycle_with_diagnostic`, `represents_static_local_cycles_with_existing_module_ids`, `builds_dependency_first_once_only_initialization_steps_from_static_graph`

### Pre-existing coverage (issue creation already found these)

- Missing module detection with source span (issue-232 diagnostic): covers bare specifiers, relative missing, named import missing, namespace import missing, side-effect import missing, re-export missing
- Default/named export mismatch (issue-233 diagnostic): covers named import missing export, re-export missing export
- Graph init order verification (`validate_init_order`): produces dependency-first initialization steps from static graph

### Scope verification

| Scope item | Status | Evidence |
|---|---|---|
| missing module diagnostics | Existing (pre-date issue) | issue-232 tests in `m9_modules` |
| cycle detection and diagnostics | Implemented | `158e2f42`, tested in `module_graph::tests` |
| default/named export mismatch | Existing (pre-date issue) | issue-233 tests in `m9_modules` |
| graph order verification | Existing (pre-date issue) | `backwards_init_order()` test |

### Test results

```
cargo nextest run -p ts2wasm-compiler -E 'test(module_graph)' => 8/8 pass
cargo nextest run -p ts2wasm-cli -E 'test(module)'         => 27/27 pass
cargo fmt --all --check                                     => clean
```

### Files changed in implementation

- `crates/compiler/src/module_graph.rs` (+135/-1): cycle detection, validation, tests

## Reopened by audit

Date: 2026-05-05

Classification: acceptance-not-actually-met.

Reopen reason: no `## Completion evidence` section is present, so close evidence cannot be cited from the issue file.

Violated acceptance: the issue cannot provide repo-local close evidence for its checked acceptance criteria while it remains in this state. Acceptance checkboxes were reset for re-verification.

Evidence files:
- `issues/done/5038-compiler-module-graph-resolution.md` (reopened by audit from done/, completion evidence added, re-closed with evidence)

Split follow-up: none created in this audit wave; this reopened issue remains the tracking item.
