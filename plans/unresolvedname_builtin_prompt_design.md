# UnresolvedName builtins burn-down prompt and rough design

## 実装プロンプト

あなたは `ts2wasm` リポジトリで、test262 の `UnresolvedName` 最大バケットを削る P0 作業を担当する。
対象は `eval` / `Function` constructor 以外の ECMAScript global builtin 名である。`eval` の direct/indirect eval 方針、parser の eval 制約、`$262.eval` の既存方針は変更しない。

現在の問題は、`Temporal`、`Float16Array`、`Object`、`Iterator`、`ShadowRealm`、`Atomics`、`Intl`、`WeakRef`、`FinalizationRegistry` などの global builtin 名が、compile pipeline のどこかで `UnresolvedName` として落ち、runtime/lowering に到達できないこと。目的は「全部を完全実装すること」ではなく、未対応 API を `UnresolvedName` ではなく、実装済み runtime call または明示的な `UnsupportedBuiltin` / `UnsupportedSyntax` に分類することである。

作業の優先順位は次の通り。

1. `Object` family を最優先で処理する。`Object.keys`、`values`、`entries`、`fromEntries`、`hasOwn`、`hasOwnProperty`、`getOwnPropertyDescriptor`、`getOwnPropertyDescriptors`、`getOwnPropertyNames`、`getOwnPropertySymbols`、`defineProperty`、`defineProperties`、`assign`、`create`、`is`、`freeze`、`seal`、`preventExtensions`、`isFrozen`、`isSealed`、`isExtensible`、`getPrototypeOf`、`setPrototypeOf` を、既存 runtime 関数があるものは接続し、ないものは意味を壊さない conservative stub または明示 unsupported にする。
2. `Float16Array` を既存 TypedArray family に統合する。`Int8Array` などと同じ constructor/method/static dispatch 経路に通し、最初は内部表現を number array / TypedArrayFromArray として扱う。float16 丸め精度そのものは別 issue に残してよいが、`new Float16Array(...)` や `Float16Array.prototype.*` が `UnresolvedName` で止まらないようにする。
3. `Iterator` / `AsyncIterator` を解決する。`Iterator.from` と iterator helper の build 到達を優先し、既存の `GetIterator`、`IteratorNext`、array iterator、generator iterator の runtime/lowering を再利用する。helper の完全 semantics が難しいものは、feature 名付きの unsupported にする。
4. `WeakRef` / `FinalizationRegistry` は既存 runtime function がある前提で、constructor と主要 prototype method が lowering まで通ることを確認する。
5. `Atomics` と `Intl` は既存実装済み subset を壊さず、未対応 method を `UnresolvedName` ではなく明示 unsupported にする。`Atomics.load/store/add/sub/and/or/xor/exchange/compareExchange/isLockFree/wait/notify/waitAsync`、`Intl.NumberFormat`、`Intl.DateTimeFormat`、`Intl.DurationFormat`、`Intl.ListFormat` の既存経路を優先する。
6. `Temporal` と `ShadowRealm` は巨大なので、今回の P0 では「完全実装」しない。global 名、namespace member、constructor-like reference、`typeof`、`.prototype`、`.name`、基本的な static member 参照を解決し、呼び出しや構築は `issue-436: Temporal API is not implemented` / `issue-436: ShadowRealm API is not implemented` のような明示 diagnostic に落とす。

触るべき主な場所を先に確認すること。

- `crates/resolve/src/name_resolver.rs`
- `crates/ir/src/builtin_resolver.rs`
- `crates/ir/src/lowered/program_builtins.rs`
- `crates/ir/src/lowered/resolver/call/constructor.rs`
- `crates/ir/src/lowered/resolver/call/method.rs`
- `crates/backend-wasm/src/runtime/manifest/all.rs`
- `crates/backend-wasm/src/runtime/object/*`
- `crates/backend-wasm/src/runtime/array/*`
- `crates/backend-wasm/src/runtime/collections/*`
- `crates/backend-wasm/src/runtime/host/*`
- `crates/cli/tests/m6_builtin_methods.rs`
- `docs/26-semantic-feature-matrix.md`
- `issues/I-20260517-9HDRYE.md`

実装方針:

- builtin 名の許可リスト、builtin resolver、lowering、runtime catalog の対応関係を棚卸しし、文字列分岐が散っている場合は小さな central registry を追加する。
- global builtin は次の状態に分類する。
  - `implemented`: runtime call または lowering が存在し、build/runtime まで通せる。
  - `stubbed`: test262 の名前解決・基本 introspection は通すが、semantics は限定的。
  - `explicit_unsupported`: `UnresolvedName` を出さず、機能名付き diagnostic を返す。
- `default_allowed_globals()` に名前を足すだけで終わらせない。後段で `ResolvedExpr::Ident(name)` が local 未解決になって落ちる箇所まで追う。
- `typeof X`、`X.name`、`X.prototype`、`X.constructor`、`X[Symbol.*]`、`new X(...)`、`X.method(...)`、`obj instanceof X` の代表形を smoke test に入れる。
- 実装済みでない API を無理に semantic pass にしない。unsupported の分類改善は合格だが、Node と違う値を返して differential mismatch を増やさない。
- test262 harness 由来の global helper と通常ユーザー input の global builtin を混ぜない。test262 専用 stub は coverage runner / preprocessor scope に限定する。
- `eval` 関連の parser/lowering/runtime は変更禁止。

最低限の acceptance criteria:

- `Temporal`、`ShadowRealm`、`Float16Array`、`Object`、`Iterator`、`Atomics`、`Intl`、`WeakRef`、`FinalizationRegistry` の代表 fixture が `UnresolvedName` で失敗しない。
- `Object` と `Float16Array` は、既存 runtime で自然に処理できる範囲では build pass 以上に進む。
- `Temporal` と `ShadowRealm` は、完全未対応箇所で明示 unsupported diagnostic になる。
- `eval` の既存テスト結果が変わらない。
- `UnresolvedName` 集計で、対象 builtin 名の件数が減る。semantic pass の水増しは禁止。

推奨テスト:

```bash
cargo test -q -p ts2wasm-resolve --test resolver_snapshot
cargo test -q -p ts2wasm-cli --test ir_lowering
cargo test -q -p ts2wasm-cli --test m6_builtin_methods build_smoke_temporal_unsupported_diagnostic
cargo test -q -p ts2wasm-cli --test m6_builtin_methods build_smoke_shadowrealm_unsupported_diagnostic
cargo test -q -p ts2wasm-cli --test m6_builtin_methods build_smoke_iterator_helpers
cargo test -q -p ts2wasm-cli --test m6_builtin_methods build_smoke_iterator_helpers_dedicated
cargo test -q -p ts2wasm-cli --test m6_builtin_methods build_smoke_weakref_finalization
cargo test -q -p ts2wasm-cli --test m6_builtin_methods build_smoke_atomics_wait_async
```

coverage command がある環境では追加で実行する。

```bash
mise run reference-coverage -- test262 --jsonl --sample 100 --jobs 4 --no-dashboard-data
mise run coverage-dashboard-data
```

最後に、変更内容を次の形式で報告する。

- resolved builtin names
- newly explicit unsupported builtin families
- changed runtime/lowering paths
- tests run
- remaining top unresolved symbols
- known semantic gaps

## 大まかな設計

### 1. 目的

`UnresolvedName` は compile pipeline の早い段階で停止するため、runtime 実装や unsupported 分類に到達できない。今回の設計では、global builtin 名を「名前として解決できる対象」として扱い、その後に実装済み subset、stub、明示 unsupported のいずれかへ流す。

成功条件は `UnresolvedName` の削減であり、`Temporal` など巨大 API の完全 semantics 実装ではない。

### 2. 非対象

- direct eval / indirect eval / `Function` constructor の対応拡大
- `Temporal` full semantics
- ShadowRealm の realm 分離 semantics
- Intl の locale 完全実装
- Float16 の完全 IEEE 754 binary16 丸め保証
- Atomics の並行実行 memory model 完全実装

### 3. Pipeline 上の責務分離

| 層 | 責務 | 失敗時の望ましい結果 |
|---|---|---|
| frontend parser | syntax を AST にする | syntax error / unsupported syntax |
| name resolver | local/global 名を識別する | 本当に存在しない名前だけ `UnresolvedName` |
| builtin resolver | builtin call/member/constructor を分類する | 実装済み IR または明示 unsupported |
| lowering | runtime call / object model / constructor に落とす | feature 名付き unsupported |
| runtime catalog/emitter | RuntimeFn を WAT に出す | 未登録 RuntimeFn を作らない |
| coverage runner | test262 harness を注入・分類する | harness 専用 stub と通常 builtin を分離 |

### 4. Builtin registry

分散した文字列 match を減らすため、次のような小さな registry を置く。

```rust
pub enum BuiltinGlobalKind {
    Namespace,
    Constructor,
    Function,
    Value,
}

pub enum BuiltinSupport {
    Implemented,
    Stubbed,
    ExplicitUnsupported { issue: &'static str },
}

pub struct BuiltinGlobalSpec {
    pub name: &'static str,
    pub kind: BuiltinGlobalKind,
    pub support: BuiltinSupport,
    pub feature: &'static str,
}
```

初期登録例:

| name | kind | support | feature |
|---|---|---|---|
| `Object` | Constructor | Implemented | `builtin:object` |
| `Float16Array` | Constructor | Stubbed | `builtin:typedarray` |
| `Iterator` | Constructor/Namespace | Stubbed | `builtin:iterator` |
| `AsyncIterator` | Constructor/Namespace | ExplicitUnsupported or Stubbed | `builtin:iterator` |
| `Temporal` | Namespace | ExplicitUnsupported | `builtin:temporal` |
| `ShadowRealm` | Constructor | ExplicitUnsupported | `builtin:shadowrealm` |
| `Atomics` | Namespace | Stubbed | `builtin:atomics` |
| `Intl` | Namespace | Stubbed | `builtin:intl` |
| `WeakRef` | Constructor | Implemented/Stubbed | `builtin:weakref` |
| `FinalizationRegistry` | Constructor | Implemented/Stubbed | `builtin:weakref` |

`default_allowed_globals()` はこの registry から生成または参照できる形に近づける。すぐに大改造しない場合でも、registry と allowed list の重複を test で検出する。

### 5. Object slice

`Object` は件数が多く、既存 runtime object domain もあるため最初に処理する。

実装方針:

- `Object.*` static method を `RuntimeFn::Object*` に接続する。
- proxy trap と既存 object semantics の分岐を壊さない。
- descriptor 系は既存 descriptor layout に合わせる。
- `Object.getOwnPropertyDescriptors` や `Object.defineProperties` が未実装なら、まずは明示 unsupported にする。
- `Object.prototype.*` は untyped receiver でも runtime method に落とす。

代表テスト:

```ts
Object.keys({ a: 1 });
Object.values({ a: 1 });
Object.entries({ a: 1 });
Object.fromEntries([["a", 1]]);
Object.hasOwn({ a: 1 }, "a");
Object.getOwnPropertyDescriptor({ a: 1 }, "a");
Object.getPrototypeOf({});
Object.setPrototypeOf({}, null);
Object.freeze({});
Object.is(NaN, NaN);
```

### 6. Float16Array slice

`Float16Array` は TypedArray family として扱う。

実装方針:

- `is_typed_array_class` / `is_typed_array_constructor` / typed array method dispatch に `Float16Array` が漏れていないか確認する。
- constructor は `TypedArrayFromArray` に流す。
- static methods は他 TypedArray と同じ扱いにする。
- binary16 丸めは別 issue に残し、今回の smoke tests では名前解決・constructor・基本 method dispatch を見る。

代表テスト:

```ts
new Float16Array();
new Float16Array(2);
new Float16Array([1, 2]);
new Float16Array([1, 2]).at(0);
Float16Array.name;
Float16Array.prototype;
```

### 7. Iterator / AsyncIterator slice

Iterator は protocol と helper の入口だけでも UnresolvedName を大きく減らす可能性がある。

実装方針:

- `Iterator.from(x)` を `GetIterator` に接続する。
- array/generator の既存 iterator facts を再利用する。
- helper は build 到達を優先し、`map/filter/take/drop/toArray/reduce/forEach/some/every/find` を順に扱う。
- unsupported helper は `issue-iterator-helpers: Iterator.prototype.<name> is not implemented` のように明示する。

代表テスト:

```ts
Iterator.from([1, 2, 3]);
Iterator.from([1, 2, 3]).next();
Iterator.from([1, 2, 3]).toArray?.();
AsyncIterator;
typeof Iterator;
```

### 8. Temporal / ShadowRealm slice

この2つは full implementation ではなく、UnresolvedName の分類改善を目的とする。

実装方針:

- `Temporal` と `ShadowRealm` は global として解決する。
- `typeof Temporal`、`Temporal.Now`、`Temporal.PlainDate`、`Temporal.Instant` などの member 参照で resolver が落ちないようにする。
- call/new された場合は明示 unsupported にする。
- test262 の unsupported reason が `UnresolvedName` から `UnsupportedBuiltin` 相当に変わることを確認する。

代表テスト:

```ts
typeof Temporal;
Temporal.Now;
Temporal.PlainDate;
new ShadowRealm();
ShadowRealm.prototype;
```

### 9. Atomics / Intl / WeakRef / FinalizationRegistry slice

実装済み subset を保ちつつ、未対応 API の名前解決落ちを消す。

方針:

- `Atomics.*` は既存 `RuntimeFn::Atomics*` に接続されている method を確認する。
- `Atomics.waitAsync` は実装済みなら smoke test、未実装なら明示 unsupported。
- `Intl.NumberFormat`、`Intl.DateTimeFormat`、`Intl.DurationFormat`、`Intl.ListFormat` は既存 constructor lowering を優先する。
- `Intl.Locale`、`Intl.Segmenter` など未対応 family は明示 unsupported。
- `WeakRef` / `FinalizationRegistry` は constructor と `deref/register/unregister` を runtime に接続する。

### 10. Diagnostics policy

`UnresolvedName` を出してよいのは、ユーザー定義でも builtin でも harness でもない本当の未知名だけ。

builtin family の未実装は次のように分ける。

```text
[UnsupportedBuiltin/builtin-resolver] issue-436: Temporal API is not implemented
[UnsupportedBuiltin/lowering] issue-419: Float16Array binary16 precision is not implemented
[UnsupportedSyntax/lowering] issue-iterator-helpers: Iterator.prototype.map is not implemented
```

diagnostic message には family 名、method 名、issue 番号または tracking label を入れる。

### 11. Regression guard

- `eval` 関連テストは変更しない。
- `Object` / `Reflect` / `Proxy` の既存 static trap dispatch を壊さない。
- unsupported を semantic pass として扱わない。
- test262 preprocessor だけで結果を良く見せない。
- `default_allowed_globals()` に足した名前が lowerer で再度未解決にならないことを smoke test で確認する。

### 12. 実装順

1. 現在の unresolved symbol 集計を保存する。
2. builtin registry またはそれに準じる centralized table を追加する。
3. `Object` static/prototype dispatch の漏れを埋める。
4. `Float16Array` を TypedArray family に完全参加させる。
5. `Iterator` / `AsyncIterator` の入口と helper unsupported を整える。
6. `WeakRef` / `FinalizationRegistry` の smoke test を固定する。
7. `Atomics` / `Intl` の既存 subset と explicit unsupported を整理する。
8. `Temporal` / `ShadowRealm` を explicit unsupported family にする。
9. coverage sample を実行し、`UnresolvedName` の対象 builtin 件数が減ったことを記録する。
10. docs と issue に残りの semantic gaps を書く。
