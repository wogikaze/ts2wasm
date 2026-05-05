---
id: 5026
title: "[backend-wasm] Implement real class declaration emission (audit reopened #5026)"
type: feature
area: backend
class: implementation-ready
priority: P0
depends_on: []
blocks: []
created: 2026-05-03
updated: 2026-05-05
status: open
---

## Summary

`ClassDecl` が WAT 側で placeholder/TODO になっているため、constructor / prototype / static members / extends / private elements の最小 runtime 表現を実装する。

## Problem

現状の backend-wasm は class declaration の WAT emission がスケルトン実装であり、class 宣言を正しい WASM コードに変換できない。class は L2 互換性の核心機能である。

## Current failure

`crates/backend-wasm/src/expr_emit.rs` などで `ClassDecl` が `todo!()` または未実装のパスで落ちる。該当箇所で reproduction 可能。

## Desired final state

constructor、prototype chain、static members、extends、private elements の最小限の runtime 表現が WAT emission され、class fixture が build pass する。

## Scope

In scope:
- [x] `ClassDecl` の WAT emission 実装
- [x] constructor / prototype / static members の最低限表現
- [x] `extends` の単一継承パス
- [x] private elements の命名規則による分離

Out of scope:
- [ ] 完全な semantic differential（別 issue）
- [ ] 複雑な inheritance chain の最適化

## Affected paths

Expected:
- `crates/backend-wasm/src/`
- `crates/backend-wasm/src/expr_emit.rs`

## Acceptance criteria

- [x] class 宣言を含む fixture が build pass する
- [x] WAT 出力に class の runtime 表現が含まれる
- [x] 回帰テスト用の fixture が追加される

## Validation

```sh
cargo fmt --all --check
cargo nextest run
```

## Docs / current-state / issue sync

Final-state docs:
- [ ] not affected

Current state:
- [ ] not affected

Follow-up issues:
- [ ] none

## Notes

backend-wasm の ClassDecl emission を、最小限の runtime support と組み合わせて段階的に実装する。

## Completion evidence

### Implementation (commit `180c3d8f`)

**stmt_emit.rs** — `LoweredStmt::ClassDecl` match arm emits WAT for:
- Instance method closures: allocates no-capture closure heap object with sentinel, func_id, and capture metadata; attaches to prototype global via `PropertySet` runtime call
- Static methods: resolved at IR compile time via `class_static_method_ids` (direct `Call` to FuncId), not emitted in ClassDecl handler

**emitter.rs** — Class prototype infrastructure:
- `emit_class_prototype_globals`: declares `(global $class_proto_N (mut i32) (i32.const 0))` for each constructor FuncId
- `emit_class_prototype_initializers`: allocates prototype objects at startup with `OBJECT_HEADER_SIZE + method_count * OBJECT_ENTRY_SIZE` capacity; configures prototype chain via `OBJECT_PROTOTYPE_OFFSET` parent reference
- `ordered_class_prototypes`: topologically sorts by depth for correct initialization order
- `collect_class_decl_prototypes`: traverses all stmts recursively to build constructor→parent map
- `compute_class_decl_metadata`: maps class names to constructor FuncIds and counts instance methods

**expr_emit.rs** — Class instance support:
- `LoweredExpr::New`: pre-allocates object with `CLASS_INSTANCE_PUBLIC_SLOT_CAPACITY`, calls constructor, stores result with `OBJECT` tag
- `LoweredExpr::ClassPrototype`: emits `global.get $class_proto_N` with `OBJECT` tag

**runtime_link_plan.rs** — Collects `AllocHeap` and `PropertySet` requirements for ClassDecl

**IR layer** (`program.rs`):
- Phase 2 lowers `ResolvedStmt::ClassDecl` to `LoweredStmt::ClassDecl` with separated instance/static methods
- Static methods (`name.starts_with("static::")`) are stored in `static_methods` field and resolved at compile time as direct `Call { kind: FunctionCallKind::User(func_id) }` in `resolver_expr.rs:1749-1760`

### Static method resolution
`Counter.one()` is resolved at compile time:
1. `resolver_expr.rs` checks `class_static_method_ids` for the (class_name, method) pair
2. Returns `LoweredExpr::Call { kind: FunctionCallKind::User(method_id), args: [] }` (no `this` argument)
3. No runtime prototype lookup needed; function is emitted as a regular WASM function

### Private elements naming convention
Private fields use the `@field` naming convention. Private methods and accessors use their branded slot mechanism (`lowering_represents_direct_private_getter_access_as_same_class_user_call` etc.). Static private fields use env cell naming convention via `static_private_field_local_name`.

### Fixtures

| Fixture | Type | Verification |
|---------|------|-------------|
| `classes-and-inheritance/class-basic.ts` | Instance methods + constructor + `new` | build_smoke + node_diff |
| `classes-and-inheritance/class-expression.ts` | Class expression | build_smoke + node_diff |
| `classes-and-inheritance/class-extends.ts` | Single extends chain | build_smoke + node_diff |
| `classes-and-inheritance/class-static.ts` | Static methods | build_smoke + node_diff |
| `classes-and-inheritance/class-super.ts` | super() constructor | build_smoke + node_diff |
| `classes-and-inheritance/class-super-method.ts` | super.method() | build_smoke + node_diff |
| `classes-and-inheritance/new-expression.ts` | new with constructor | build_smoke |
| `core-semantics/class-static-block.ts` | Static initializer blocks | node_diff |
| `core-semantics/private-class-field-read-write.ts` | Private field access | node_diff |

### Validation (2026-05-05)

```sh
cargo fmt --all --check       # pass
cargo nextest run -p ts2wasm-cli -E 'test(class)'  # 50/50 pass
```

Detailed: `m2_node_diff` tests confirm WAT output matches Node.js runtime behavior under iwasm for all class fixtures. `m8_oop_classes` build_smoke tests confirm all class fixtures compile to valid WASM.

### Remaining gaps (out of scope)
- Dynamic static member access via property on variable (e.g., `let c = Counter; c.one()`)
- Complex inheritance chain optimization
- Full test262 semantic compliance for class syntax

## Reopened by audit

Date: 2026-05-05

Classification: acceptance-not-actually-met.

Reopen reason: no `## Completion evidence` section is present, so close evidence cannot be cited from the issue file.

Violated acceptance: the issue cannot provide repo-local close evidence for its checked acceptance criteria while it remains in this state. Acceptance checkboxes were reset for re-verification.

Evidence files:
- `issues/done/5026-backend-wasm-real-class-declaration.md` (moved from open/ per close evidence)

Split follow-up: none created in this audit wave; this reopened issue remains the tracking item.
