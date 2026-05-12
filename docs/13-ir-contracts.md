# IR Contracts

このドキュメントは ts2wasm の各 IR 段階の責務、不変条件、validate 関数の仕様を定める。
各 IR は前段の不変条件を満たした後にのみ構築される。

## IR contracts validation summary

IR contracts は各 phase の入口で検証する。現在は `LoweredProgram` の `validate_lowered`、初期 Semantic HIR slice の `validate_hir`、および `MirProgram` boundary の `validate_mir` wrapper を持つ。`MirProgram` は現時点では `LoweredProgram` alias であり、`Validated<MirProgram>` は native MIR 完成前の bridge boundary として使う。build pipeline は HIR lowering が対応済みの subset では `validate_hir` を実行し、未対応構文は既存 `LoweredProgram` pipeline を維持する。HIR が対応した構文で invariant violation が出た場合は compiler bug として `InvariantViolation` を返す。

## IR 段階の概観

```text
Source
  .ts / .js ファイル。文字列。

AST (Abstract Syntax Tree)
  構文構造。Span を持つ。名前解決なし。型情報なし。

BuiltinResolved AST (現在の semantic 前段)
  BuiltinResolver pass の出力。
  `console.log` / `.length` / property / computed index の意味付けを持つ。
  まだ LocalId / FuncId には解決されていない。

HIR (High-level IR) — 初期 slice 実装済み
  名前解決済み。JS semantic operation を保持する。
  `crates/ir::semantic` が対応済み subset の lowering と validation を提供する。
  `ts2wasm dump --tir` はこの phase を structural debug output として表示する。
  `ts2wasm dump --tir --unparse` は local/function ID を保持した pseudo source を表示する。

Optimized HIR
  HIR に optimization level と safety mode に応じた optimizer pass を適用した診断用 IR。
  `ts2wasm dump --optimize -O0..-O3` はこの phase を structural debug output として表示する。
  `ts2wasm dump --optimize --unparse -O2` は optimizer 後の HIR を pseudo source として表示する。

MIR (Mid-level IR / Runtime IR) — alias/bridge 実装済み、native MIR は未完
  現在は `MirProgram = LoweredProgram` alias として runtime ABI intent の boundary を提供する。
  `Validated<MirProgram>` と optional `emit_mir_wat` path は存在するが、native MIR data model と native MIR emitter は次段の作業である。

Wasm IR — backend-core typed IR 実装済み、main pipeline 全面移行は未完
  wasm local / function / import / memory / data segment に直接対応する typed node。
  `WasmInstr` / `WasmModule`、`WatWriter::emit_module`、feature-gated wasm-encoder path が存在する。

LoweredProgram（現在の default lowering 出力 / MIR compatibility bridge）
  NameResolver + Lowering が Resolved 表現を LocalId / FuncId に解決した後の表現。
  backend が AST を直接参照しないための隔離層。
  native MIR が完成するまで、default backend input かつ `MirProgram` alias の実体として使う。
```

## AST

### 責務

* Lexer + Parser の出力。
* 構文構造を忠実に表現する。
* 意味論的な変換を含まない（名前解決、型チェック、定数畳み込みをしない）。

### 不変条件

* すべての node は `Span { start, end }` を持つことを目標とする。
  現状では Span なしを許容するが、新規 node は Span を持つこと。
* parser は入力の構文エラーを `Vec<Diagnostic>` で返す。`panic!` しない。

### validate_ast の仕様（追加予定）

```rust
pub fn validate_ast(ast: &[Stmt]) -> Result<(), Vec<Diagnostic>>
```

検査内容:

| 検査 | Diagnostic code | 状態 |
|---|---|---|
| top-level `return` が `_start` に入らないか | `InvalidTopLevelReturn` | 予定 |
| 関数定義の重複 | `DuplicateFunction` | 予定 |
| サポート外構文（for, class, try 等） | `UnsupportedSyntax` | 現状 |
| サポート外 builtin / Date / RegExp / module / eval / TS 構文 / runtime subset | `UnsupportedBuiltin` / `UnsupportedDate` / `UnsupportedRegExp` / `UnsupportedModule` / `UnsupportedEval` / `UnsupportedTypeScriptSyntax` / `UnsupportedRuntimeSubset` | `UnsupportedSyntax` に集約せず、表示診断と coverage 集計で原因を分ける |

### BuiltinResolved AST

BuiltinResolver は Parser AST を以下へ写像する。

```rust
ResolvedExpr::BuiltinCall { builtin: BuiltinId, args: Vec<ResolvedExpr> }
ResolvedExpr::BuiltinProperty { builtin: BuiltinPropertyId, object: Box<ResolvedExpr> }
ResolvedExpr::PropertyAccess { object: Box<ResolvedExpr>, key: String }
ResolvedExpr::ComputedIndex { object: Box<ResolvedExpr>, index: Box<ResolvedExpr> }
```

```text
console.log(x) -> BuiltinCall(ConsoleLog, [x])
obj.length -> BuiltinProperty(Length, obj)
obj.key -> PropertyAccess(obj, "key")
obj[index] -> ComputedIndex(obj, index)
```

Lowering はこの Resolved 表現を入力に取り、Parser AST を直接解釈しない。

### AST enum 設計方針

Parser AST は構文だけを保持し、builtin の意味判定を持たない。

```rust
Expr::Member { object: Box<Expr>, property: PropertyKey }
Expr::Call { callee: Box<Expr>, args: Vec<Expr> }
Expr::Index { object: Box<Expr>, index: Box<Expr> }
```

`console.log(x)` かどうかは BuiltinResolver だけが判断する。

### Array literal elisions

Array literals preserve elisions as syntax. The parser represents array literal
slots with an array-element enum rather than `Vec<Expr>`:

```rust
enum ArrayElement {
    Present(Expr),
    Hole { span: Span },
    Spread { expr: Expr, span: Span },
}

Expr::Array {
    elements: Vec<ArrayElement>,
    span: Span,
}
```

A comma without an expression contributes one `Hole`. A trailing comma after a
present element or spread element is only a separator and does not add a hole;
a trailing comma after an elision terminates the already-counted hole. The AST
does not lower a hole to `Expr::Undefined`, because `undefined` is a present
value while a hole is an absent indexed property.

## IR design

Generic JavaScript semantic IR は、AST の構文構造でも backend の runtime ABI でもない中間層として設計する。目的は、JavaScript の observable semantics を `backend-wasm` から切り離し、lowering/validation/optimization が同じ semantic operation を共有できるようにすることである。

### Semantic instruction set

HIR は次の命令群を持つ。命令名は設計上の契約であり、実際の Rust enum 名は 020b で確定する。

| Group | Instructions | Semantics |
|---|---|---|
| Value | `ConstUndefined`, `ConstNull`, `ConstBool`, `ConstNumber`, `ConstString`, `ConstObject`, `ConstArray` | JS value を作る。backend の tagged layout はここでは露出しない |
| Local / builtin receiver | `LoadLocal`, `StoreLocal`, `LoadBuiltin` | locals は ID 参照。`LoadBuiltin` は HIR 初期 slice で許可済み global receiver 名を保持する暫定表現 |
| Conversion | `ToBoolean`, `ToNumber`, `ToString`, `ToPropertyKey`, `ToPrimitive` | JS の抽象操作を表す。最適化 hint があっても意味論を変えない |
| Operator | `JsAdd`, `JsStrictEqual`, `JsAbstractEqual`, `JsRelational`, `JsUnaryNot`, `JsTypeOf`, `JsInstanceOf` | operator token ではなく JS semantics。`JsAdd` は number add と string concat の両方を保持する |
| Property | `GetProp`, `SetProp`, `GetIndex`, `SetIndex`, `HasProp`, `OwnKeys`, `ArrayLength` | prototype chain / property key conversion / array length semantics を backend から隠す |
| Call | `CallFunction`, `CallMethod`, `CallBuiltin`, `Construct` | receiver と callee を明示し、method call の `this` を失わない |
| Control | `Block`, `BranchIfTruthy`, `Loop`, `Break`, `Continue`, `Return`, `Throw`, `TryCatchFinally` | condition は `ToBoolean` または truthy branch として明示する |
| Metadata | `TypeHint`, `SourceSpan`, `CapabilityHint` | 診断・最適化・manifest の補助情報。metadata だけで observable semantics を変更しない |

### Design decisions

* `JsAdd` は HIR では単一命令のまま保持する。TypeScript type hints が `number-add-fast-path` を示しても、runtime guard または証明済み typed lowering がない限り static number add に置き換えない。
* property access は `GetProp` / `SetProp` に集約し、computed index は `ToPropertyKey` を通して同じ property model に入れる。array index fast path は MIR/backend の最適化で扱う。
* `CallMethod` は receiver を明示フィールドとして持つ。`obj.method()` を `CallFunction(GetProp(obj, "method"))` に潰すと `this` binding が失われるため禁止する。
* TypeScript checker 由来の型情報は `TypeHint` metadata として保持する。hint は最適化候補を示すだけで、semantic validation の代替にはしない。
* HIR validation は unresolved name、invalid local/function ID、arity mismatch、top-level return、receiver loss を検出する。backend は validate 済み HIR/MIR だけを受け取る。

## HIR — High-level IR（初期 slice 実装済み）

### 責務

* 名前解決済みの表現。
* JS semantic operation を保持する（`JsAdd`, `JsStrictEqual`, `ToBoolean`, `ToString`, `GetProp`, `SetProp`, `Call`）。
* operator の意味論的な分岐（number add vs string concat）をここで保持する。
  backend で分岐するのではなく、semantic lowering で `JsAdd` に落とす。

### 不変条件

* すべての名前参照は `LocalId` / `FuncId` / `BuiltinId` に解決済み。
* 未解決名 `Ident(String)` は HIR に残らない。
* `JsAdd` は number add と string concat の両方を表す。
  runtime lowering で `RuntimeFn::Add` に落とす（静的分岐しない）。
* すべての node は `Span` を持つ。

### dump --tir の仕様

`dump --tir` は `BuiltinResolved AST` から `HirProgram` を構築し、`validate_hir` に成功した結果だけを出力する。これは backend 用の runtime lowering である `LoweredProgram` の別名ではない。

構造出力は Rust の debug representation を使い、`HirProgram` / `HirStmt` / `HirExpr` をそのまま表示する。`--unparse` は source の識別子名ではなく HIR の `local$N` / `fn$N` ID を表示し、`JsAdd`、`ToBoolean`、`CallBuiltin` 相当の semantic operation を残した pseudo source にする。

### dump --optimize の仕様

`dump --optimize -O0..-O3` は `HirProgram` に optimizer pipeline を適用し、`validate_hir` に成功した optimized HIR だけを出力する。これは `LoweredProgram` の別名ではなく、optimizer pass の実行結果を `OptimizedHirProgram` として表示する。

構造出力は optimization level、`applied_passes`、optimized `HirProgram` を含む。`-O0` は pass を適用しない基準出力であり、`-O1` 以上の pass は observable JavaScript semantics を壊さない場合にだけ IR を変更する。`--unparse` は optimizer 後の HIR を pseudo source として表示し、source の識別子名ではなく `local$N` / `fn$N` ID を保持する。

### validate_hir の仕様

```rust
pub fn validate_hir(program: &HirProgram) -> Result<(), Vec<Diagnostic>>
```

検査内容:

| 検査 | Diagnostic code |
|---|---|
| 未解決名の残留 | `UnresolvedName` |
| call arity mismatch | `ArityMismatch` |
| top-level return | `InvalidTopLevelReturn` |
| duplicate function | `DuplicateFunction` |
| local / function ID の範囲外参照 | `InvariantViolation` |
| truthy branch condition が `ToBoolean` を通っていない | `InvariantViolation` |
| function table index と `HirFunctionId` の不一致 | `InvariantViolation` |

## LoweredProgram — 現在の lowering 中間形式

### 責務

* Resolver が名前を ID に解決した後の表現。
* backend が AST / parser 型を直接インポートしないための隔離層。
* native MIR が完成するまでの default backend input / MIR compatibility bridge。

### 構造

```rust
pub struct LoweredProgram {
    /// トップレベルの実行文（関数定義を除く）。_start に入る。
    pub top_level_statements: Vec<LoweredStmt>,
    /// _start で使う local 変数 ID。
    pub top_level_locals: Vec<LocalId>,
    /// 定義されたユーザー関数。
    pub functions: Vec<LoweredFunction>,
    /// module graph lowering information。
    pub modules: Vec<ModuleInfo>,
}

pub struct LoweredFunction {
    pub id: FuncId,
    pub params: Vec<LocalId>,
    pub uses_receiver: bool,
    pub min_required_params: usize,
    pub rest_param_index: Option<usize>,
    pub locals: Vec<LocalId>,
    pub body: Vec<LoweredStmt>,
    pub recursion_depth: usize,
    pub is_async: bool,
}
```

Array construction in `LoweredProgram` carries the same present-vs-absent
distinction as the AST:

```rust
enum LoweredArrayElement {
    Present(LoweredExpr),
    Hole,
}

LoweredExpr::ArrayNew {
    elements: Vec<LoweredArrayElement>,
}
```

Lowering preserves literal holes into `LoweredArrayElement::Hole`. A dense array
literal is the special case where every element is `Present`. Array spread is
not represented as a hole-preserving copy operation: spreading an array reads
each source index through the array get/iterator contract, so a source hole
lowers to a present `undefined` in the destination expression list.

### 不変条件

* `top_level_statements` に `LoweredStmt::Function` は含まれない。
  関数定義はすべて `functions` に入る。
* `LoweredExpr::Local(LocalId)` の `LocalId.0` は対応する関数の `locals` 以内。
* `LoweredExpr::Call { kind: FunctionCallKind::User(FuncId) }` の `FuncId.0` は `functions` の有効インデックス。
* 名前文字列（`Ident(String)`）は `LoweredExpr` に残らない。
* `LoweredArrayElement::Hole` は `LoweredExpr::Undefined` に置換されない。
  `array_get` は hole を `undefined` として読むが、`array_has_index` / `in`
  / `Array.prototype.map` は hole の absence を観測する。

### validate_lowered の仕様

```rust
pub fn validate_lowered(program: &LoweredProgram) -> Result<(), Vec<Diagnostic>>
```

検査内容:

| 検査 | Diagnostic code | 現在の状態 |
|---|---|---|
| top_level_statements に Function が入っていないか | `InvariantViolation` | 実装済み (variant 不在のため構造的に保証) |
| LocalId が範囲内か | `InvariantViolation` | 実装済み |
| FuncId が範囲内か | `InvariantViolation` | 実装済み |
| call arity が params 数と一致するか | `ArityMismatch` | 実装済み |
| top_level_locals が 0 から連続しているか | `InvariantViolation` | 実装済み (5227) |

### Resolver の責務と保証

```rust
pub struct Resolver {
    // 省略: scope stack, function registry
}
```

* `lower_program` を呼んだ後、`LoweredProgram` の不変条件をすべて満たす。
* scope が存在しない名前は `Err(Diagnostic { code: UnresolvedName })` を返すことを目標とする。
  現状では panic せず、将来のエラー対応の準備として `None` を返す場合がある。

## MIR — Mid-level IR（alias/bridge 実装済み、native MIR 未完）

### 現在の実体

現時点の MIR は独立した data model ではなく、`crates/ir/src/lowered/mir.rs` の type alias である。

```rust
pub type MirProgram = LoweredProgram;
pub type MirExpr = LoweredExpr;
pub type MirStmt = LoweredStmt;
```

この alias により、`Validated<MirProgram>`、`validate_mir`、`emit_mir_wat` の phase boundary は operational になっている。一方で、これは native MIR が完成したことを意味しない。

### 責務

* HIR の semantic operation を runtime ABI intent に寄せる。
* `RuntimeFn`、locals、functions、modules、classes、closures、control-flow invariants を backend に渡せる形で保持する。
* parser syntax と WAT string を持たない。

### 現在の bridge path

* `lower_hir_to_mir` は `HirProgram` を `LoweredProgram` 互換の MIR に変換する。
* `Validated<MirProgram>::new_mir` は `validate_mir` を通して invariant violation を fatal にする。
* backend の `emit_mir_wat` は `Validated<MirProgram>` を受け取るが、現状は standard `LoweredProgram` emitter への委譲 bridge である。

### Native MIR の完成条件

* `MirProgram` / `MirStmt` / `MirExpr` が alias ではなく独立型になる。
* `validate_mir` が native MIR の local/function/module/control-flow invariants を直接検査する。
* `lower_hir_to_mir` が native MIR を直接返す。
* `emit_mir_wat` が selected subset から native MIR を直接 emit し、bridge 委譲を段階的に縮小する。

## Wasm IR（backend-core typed IR 実装済み、全面移行未完）

### 現在の実体

`crates/backend-core/src/wasm_ir.rs` に typed wasm module model が存在する。主な型は以下である。

```rust
pub enum WasmInstr { /* typed wasm instructions */ }
pub struct WasmFunction { /* params/results/locals/body */ }
pub struct WasmModule { /* imports/globals/memory/data/functions/exports */ }
```

`backend-wasm` の `WatWriter` は `WasmInstr` / `WasmModule` を WAT に出力できる。feature `wasm-encoder-backend` では `WasmModule` から binary を生成する path もある。

### 責務

* wasm module structure の typed representation。
* imports / globals / memory / data segment / functions / exports を保持する。
* WAT writer と binary encoder の共有入力になる。

### 不変条件

* すべての local は型 (`i32` / `i64` など) を持つ。
* function/import/global/export は module 内で解決可能な symbol / index に対応する。
* host import は RuntimeLinkPlan / manifest と一致する。
* 生成された WAT / wasm binary は validation tool を通る。

### 未完了範囲

* default backend path はまだ `Validated<LoweredProgram>` → WAT emitter が中心である。
* `WasmInstr::Raw` は migration 中の escape hatch であり、完成形では audited legacy helper に限定する。
* main pipeline の MIR → WasmIR → WAT/binary 全面移行は `docs/27-ir-layer-completion.md` の gate で進める。

## 禁止パターン

| パターン | 理由 |
|---|---|
| backend が `Stmt` / `Expr` を直接参照する | AST と backend が結合する |
| backend で名前文字列をスキャンして local を収集する | Resolver の責務を侵害する |
| `LoweredExpr::Ident(String)` が backend に到達する | 名前解決漏れ |
| runtime 関数名を文字列リテラルで backend に持つ | `RuntimeFn` catalog を使うべき |
| validate を通さずに次の phase に渡す | 不変条件が検査されない |

## IR 変更手順

IR に variant を追加・変更する際は以下の順番で行う。

1. 不変条件のドキュメント（このファイル）を更新する。
2. validate 関数に検査を追加する。
3. debug / Display impl を更新する。
4. snapshot test を更新する。
5. differential test が通ることを確認する。
6. `current-state.md` を更新する。
