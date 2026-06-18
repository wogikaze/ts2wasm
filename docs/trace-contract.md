# Trace Contract

Coverage PR は実行結果だけでなく、正しい経路を通った証拠として trace を提出する。

## Trace 種類

| Trace | 生成元 | 内容 | 必須条件 |
|-------|--------|------|---------|
| `SemanticIRTrace` | `semantic-ir` | CFG block 列、Reference 解決、GetValue/PutValue 呼び出し | P1 実装後 |
| `SpecOpTrace` | `spec-kernel` | SpecOp 呼び出し順序、object kind dispatch 結果 | P2 実装後 |
| `RuntimeCoreTrace` | `runtime-core` | Shape 遷移、PropertyDescriptor 操作、Env record 操作 | P0 実装後 |
| `DeoptTrace` | `opt-mir` | Guard 挿入、Guard 破壊、DeoptToBaseline 発火 | P4 実装後 |

## Trace 記法

trace は `->` で操作を連結する。

```
ToPropertyKey -> GetOwnProperty -> OrdinaryGetOwnProperty
```

Proxy 経由の場合は Proxy を明示する。

```
ToPropertyKey -> Get -> ProxyGet -> OrdinaryGet
```

Guard/deopt の場合は guard と fallback を記述する。

```
GuardShape { expected: S1 } -> ShapeLoad(x) -> GuardFailure(S1→S2) -> DeoptToBaseline -> Get -> OrdinaryGet
```

## サンプル: property get

入力 JS:
```js
let o = { x: 1 };
return o.x;
```

期待 trace (SpecOpTrace):
```
ToPropertyKey("x") -> Get(object=o, key="x") -> OrdinaryGet -> return 1
```

期待 trace (RuntimeCoreTrace):
```
ShapeLoad(o, S1) -> ShapeFind(o, "x") -> offset=8 -> Load(inline[8]) -> return 1
```

## サンプル: accessor

```js
let o = { get x() { return 1 } };
o.x;
```

期待 trace:
```
ToPropertyKey("x") -> GetOwnProperty(o, "x") -> accessor descriptor found -> Call(getter, receiver=o) -> return 1
```

## サンプル: Proxy

```js
let p = new Proxy({}, { get(t, k, r) { return 2 } });
p.x;
```

期待 trace:
```
ToPropertyKey("x") -> Get(object=p, key="x") -> ObjectKind::Proxy -> ProxyGet(target={}, handler={get:...}) -> handler.get({}, "x", p) -> return 2
```

## サンプル: deopt

```js
let o = { x: 1 };
o.x; // ShapeLoad 経路
Object.defineProperty(o, "x", { get() { return 2 } });
o.x; // guard 破綻 → DeoptToBaseline → correctness path
```

期待 trace (最初のアクセス):
```
GuardShape(o, S1) -> ShapeLoad(o, x) -> return 1
```

期待 trace (defineProperty 後):
```
GuardShape(o, S1) -> GuardFailure(shape mutated) -> DeoptToBaseline -> Get -> OrdinaryGet -> accessor descriptor -> Call(getter) -> return 2
```

## Coverage PR 必須条件

1. 変更した層に対応する trace を記述する
2. `expected_trace` と `actual_trace` を比較する
3. 差分がある場合は `coverage-classification.py` で分類する
4. `native_lowered.rs` / `typed.rs` 経路の trace は証明として認めない
