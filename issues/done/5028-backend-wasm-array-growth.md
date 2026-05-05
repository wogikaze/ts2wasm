---
id: 5028
title: "[backend-wasm] Implement array growth and reallocation for push/write paths (audit reopened #5028)"
type: feature
area: backend
class: done
priority: P0
depends_on: []
blocks: []
created: 2026-05-03
updated: 2026-05-06
---

## Summary

array push が「容量十分前提」になっているため、capacity check、reallocate、presence bitmap 更新、GC root 保護を含めて安全化する。

## Problem

現在の array push 実装は事前確保済み容量を前提としており、容量不足時の reallocation が行われない。その結果、配列の動的成長が必要なケースでメモリ破壊や crash が発生する。

## Current failure

`arr.push(value)` を繰り返す fixture が、配列容量を超えた時点で不正なメモリアクセスを起こす。

## Desired final state

array push が capacity check -> reallocation -> presence bitmap 更新の安全なパスを通り、動的成長が正しく動作する。

## Scope

In scope:
- [x] Capacity check の実装
- [x] Reallocation パス（新容量計算、コピー、旧領域解放）
- [x] Presence bitmap の更新
- [x] GC root 保護

Out of scope:
- [x] 疎配列の最適化
- [x] 型付き配列

## Affected paths

Expected:
- `crates/backend-wasm/src/`

## Acceptance criteria

- [x] 動的 push fixture が Node differential で一致
- [x] capacity 超過時の再割り当てが正しく動作
- [x] GC mark が再割り当て後も正しく動作

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

## Reopened by audit

Date: 2026-05-05

Classification: acceptance-not-actually-met.

Reopen reason: no `## Completion evidence` section is present, so close evidence cannot be cited from the issue file.

Violated acceptance: the issue cannot provide repo-local close evidence for its checked acceptance criteria while it remains in this state. Acceptance checkboxes were reset for re-verification.

Evidence files:
- former open issue file before this move
- `issues/done/5028-backend-wasm-array-growth.md` after this move

Split follow-up: none created in this audit wave; this reopened issue remains the tracking item.

## Completion evidence

Completed: 2026-05-06

Commits:
- (this issue-state commit)

Evidence:

- `crates/backend-wasm/src/runtime_arrays_objects.rs`: `$array_push_grow` computes old length/capacity from the GC body size, uses the in-capacity write path when possible, otherwise calculates a new capacity, extends in place when the array is top-of-heap and memory is committed, or allocates a replacement array via `$alloc_heap`, copies old elements with `$copy`, stores the pushed value, and preserves/updates the presence bitmap.
- `crates/backend-wasm/src/expr_emit_arrays.rs`: `ArrayPushGrow` call emission mirrors array/value operands through backend temporary locals before the runtime helper call, preserving evaluated values across helper-side allocation/GC.
- `crates/backend-wasm/src/runtime_link_plan.rs` and `crates/backend-wasm/src/runtime_fn_impl.rs`: `ArrayPushGrow` is cataloged and included through the runtime link plan, including `ArrayPushMany` dependency handling.
- `crates/backend-wasm/src/lib.rs`: `array_push_grow_emits_dedicated_helper_boundary` verifies the dedicated helper, call site, capacity local, `$alloc_heap`, and `$copy` are emitted, then compiles the WAT with `wat2wasm`.
- `fixtures/core-semantics/array-push-recursive-growth.ts`: recursive array growth fixture exercises repeated `answers.push(...)` under allocation pressure.
- `crates/cli/tests/common/m2_node_diff_fixture_tests.rs`: `array_push_recursive_growth_fixture_matches_node_output_under_iwasm` verifies the recursive growth fixture against Node/iwasm differential output.
- `current-state.md`: records that unused statement-form local-array `arr.push(value);` uses a grow path, mutates in place when possible, reallocates/copies when capacity is exhausted, and that the recursive growth fixture covers the ABC451 depth-3 reducer.

Validation result:

```text
command: python scripts/manager.py check issues
result: pass; issues/index.md queue OK; check_issue_health: OK

command: python scripts/manager.py check scripts
result: pass; shell syntax checks passed

command: python scripts/manager.py fmt
result: pass

command: python scripts/manager.py update-issue-index --check
result: pass; issues/index.md OK (up to date)

command: cargo test -p ts2wasm-backend-wasm array_push_grow_emits_dedicated_helper_boundary -- --nocapture
result: pass; 1 passed

command: cargo test -p ts2wasm-cli array_push_recursive_growth_fixture_matches_node_output_under_iwasm -- --nocapture
result: pass; 1 passed

command: cargo nextest run -p ts2wasm-backend-wasm array_push_grow_emits_dedicated_helper_boundary
result: pass; 1 passed

command: cargo nextest run -p ts2wasm-cli array_push_recursive_growth_fixture_matches_node_output_under_iwasm
result: pass; 1 passed

command: python scripts/manager.py check runtimefn
result: pass

command: python scripts/manager.py check wasm-validation
result: pass
```

Broader gate note:

```text
command: python scripts/manager.py check differential
result: fail; unrelated baseline failure in m2_node_diff_fixture_tests::eval_fixture_reports_unsupported.
expected [UnsupportedEval], got [UnsupportedSyntax] expected RightParen, got Some(Comma) at 621..622.
The 5028 focused fixture passed in the same m2_node_diff suite.
```
