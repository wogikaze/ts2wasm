# Coverage Driver Protocol

## 原則

test262 coverage は**実装方針ではなく結果指標**です。
coverage を直接上げようとすると、旧 backend に helper を積んで破綻します。
正しい構造は「coverage を上げようとすると、必ず semantic floor、spec kernel、
runtime core、correctness backend が育つ」ものです。

coverage rate より重要な指標: **unclassified failure count**

## Failure Taxonomy

test262 の各 failing test は必ず以下のいずれかに分類されます。

| Kind | 意味 | 所有者 | 例 |
|------|------|--------|-----|
| `ParseGap` | 構文が読めない | `frontend` | 新しい構文、TypeScript 以外の JS 構文 |
| `ResolveGap` | binding/scope/module が解決できない | `resolve` | 新しい import 形式、private name |
| `SemanticIRGap` | 意味操作が IR で表現できない | `semantic-ir` | Reference、Completion、Iterator、Async |
| `SpecOpGap` | Internal method 実装がない | `spec-kernel` | `[[Get]]`、`[[Set]]`、`[[Construct]]`、Proxy |
| `RuntimeCoreGap` | Engine substrate がない | `runtime-core` | Shape、Descriptor、Realm、GC |
| `CorrectnessBackendGap` | IR→SpecOp への lowering がない | `backend-correctness` | 正しい層に IR はあるが wasm call が出せない |
| `LegacyBackendLeak` | 旧 `native_lowered`/`typed.rs` に仕様判断が漏れている | **設計違反** | 旧経路に新規 helper を追加 |
| `OptimizationGap` | correctness path は通るが遅い | `opt-mir` | coverage の blocker にしない |

## Coverage Driver フロー

```
test262 run
  → extract failures
  → classify each failure → {Parse,Resolve,SemanticIR,SpecOp,RuntimeCore,CorrectnessBackend,Legacy,Optimization}Gap
  → group by first missing capability
  → pick largest group (most impact per fix)
  → identify the owning layer's missing primitive
  → implement in owning layer ONLY
  → add trace test proving correct path
  → update coverage dashboard
  → (optional) file opt-mir issue
```

## PR Protocol

coverage increase PR には以下が必須:

1. **failure classification** — どの Gap か
2. **owning layer** — 変更した crate
3. **trace test** — 正しい経路を通った証拠 (semantic-ir dump, SpecOp trace, 等)
4. **no legacy escape** — `native_lowered.rs`, `typed.rs` 未変更
5. **no RuntimeFn addition** — 新しい SpecOp として定義
6. **no unclassified failures increase** — 分類不能が増えていない

## Reject Conditions

| 条件 | 理由 |
|------|------|
| `native_lowered.rs` LOC 増加 | legacy freeze 違反 |
| `typed.rs` LOC 増加 | legacy freeze 違反 |
| `RuntimeFn` variant 追加 | SpecOp を使うべき |
| 実行結果のみ合っている (trace なし) | 経路を証明できない |
| Legacy backend only の coverage 改善 | 設計を壊す |
| `test262 expected` 更新のみ | 真の改善ではない |
| unclassified failure 増加 | 分類が追いついていない |

## Dashboard Metrics

```
overall pass rate
parse pass rate
resolved pass rate
semantic-ir lowered rate
spec-kernel executed rate
correctness-backend executed rate
runtime crash count
semantic mismatch count
legacy path pass count
new path pass count
unclassified failure count       ← 最重要指標
```

## 開発順

1. P0a: Value/Heap/Object (Value, GC header, shape, descriptor, ordinary object)
2. P0b: Execution Context (realm, env, function, closure, call frame)
3. P0c: Runtime Services (job queue, baseline VM container, GC)
4. P0d: Trace/Inspection (heap dump, SpecOp trace, FrameState dump)
5. P1: Semantic IR (CFG-based, Reference, GetValue, PutValue, Call, Construct)
6. P2: Spec Kernel (internal method dispatch, object kind vtable)
7. P3: Correctness Backend (SemIR → SpecOp → wasm runtime call)
8. P4: Opt MIR (Guard, Deopt, SlowPathCall)
9. P5: RuntimeFn 整理

## 最初の縦スライス

```js
function f(o) {
  o.x = 1;
  return o.x;
}
f({});
```

必要なもの: ordinary object allocation, shape transition, property set,
property get, function call, return edge, SpecOp trace,
backend-correctness runtime call。旧 `native_lowered.rs` なしで通す。

## 次のスライス

```js
let o = { get x() { return 1 } };
o.x;
```

必要なもの: GetOwnProperty, accessor descriptor, Call getter, receiver。

## Proxy スライス

```js
let p = new Proxy({ x: 1 }, { get(t, k, r) { return 2 } });
p.x;
```

必要なもの: object kind dispatch, Proxy trap。

## Deopt スライス

```js
function f(o) { return o.x + o.x; }
let o = { x: 1 };
f(o);
Object.defineProperty(o, "x", { get() { return 2 } });
f(o);
```

必要なもの: ShapeLoad → guard → prototype mutation → correctness path 復帰。
