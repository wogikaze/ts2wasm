# IR Contracts

このドキュメントは ts2wasm の各 IR 段階の責務、不変条件、validate 関数の仕様を定める。
各 IR は前段の不変条件を満たした後にのみ構築される。

## IR 段階の概観

```text
Source
  .ts / .js ファイル。文字列。

AST (Abstract Syntax Tree)
  構文構造。Span を持つ。名前解決なし。型情報なし。

HIR (High-level IR) — 未実装、M1 以降
  名前解決済み。JS semantic operation を保持する。
  型情報の一部（typeof, instanceof など）を持ちうる。

MIR (Mid-level IR / Runtime IR) — 未実装、M2 以降
  runtime ABI 呼び出しに寄せた表現。
  RawValue / HeapPtr を直接扱う。

Wasm IR — 未実装、M2 以降
  wasm local / function / import / memory / data segment に直接対応。
  wasm-encoder や WasmFunc builder の入力形式。

LoweredProgram (現在の中間形式、M0)
  Resolver が名前を LocalId / FuncId に解決した後の表現。
  backend が AST を直接参照しないための隔離層。
  HIR / MIR の分割が完了するまでの暫定形式。
```

## AST

### 責務

* Lexer + Parser の出力。
* 構文構造を忠実に表現する。
* 意味論的な変換を含まない（名前解決、型チェック、定数畳み込みをしない）。

### 不変条件

* すべての node は `Span { start, end }` を持つ（M1 以降）。
  M0 では Span なしを許容するが、新規 node は Span を持つこと。
* `Stmt::ConsoleLog` は M0 互換のため存在する。M1 以降は追加しない。
* parser は入力の構文エラーを `Vec<Diagnostic>` で返す。`panic!` しない（M1 以降）。

### validate_ast の仕様（M1 以降）

```rust
pub fn validate_ast(ast: &[Stmt]) -> Result<(), Vec<Diagnostic>>
```

検査内容:

| 検査 | Diagnostic code | M0/M1 |
|---|---|---|
| top-level `return` が `_start` に入らないか | `InvalidTopLevelReturn` | M1 |
| 関数定義の重複 | `DuplicateFunction` | M1 |
| サポート外構文（for, class, try 等） | `UnsupportedSyntax` | M0 |

### AST enum 設計方針

現在の `Stmt::ConsoleLog(Expr)` は M0 互換として保持する。
M1 以降は以下に移行する。

```rust
Expr::Member {
    object: Box<Expr>,
    property: PropertyKey,
}

Expr::Call {
    callee: Box<Expr>,
    args: Vec<Expr>,
}
```

`console.log(x)` → semantic pass → `BuiltinCall(ConsoleLog, [x])`

## HIR — High-level IR（M1 以降）

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

### validate_hir の仕様（M1 以降）

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

## LoweredProgram — M0 中間形式

### 責務

* Resolver が名前を ID に解決した後の表現。
* backend が AST / parser 型を直接インポートしないための隔離層。
* HIR が完成するまでの暫定形式。

### 構造

```rust
pub struct LoweredProgram {
    /// トップレベルの実行文（関数定義を除く）。_start に入る。
    pub top_level_statements: Vec<LoweredStmt>,
    /// _start で使う local 変数の数。
    pub top_level_locals: u32,
    /// 定義されたユーザー関数。
    pub functions: Vec<LoweredFunction>,
}

pub struct LoweredFunction {
    pub id: FuncId,
    pub params: u32,
    pub locals: u32,
    pub body: Vec<LoweredStmt>,
}
```

### 不変条件

* `top_level_statements` に `LoweredStmt::Function` は含まれない。
  関数定義はすべて `functions` に入る。
* `LoweredExpr::Local(LocalId)` の `LocalId.0` は対応する関数の `locals` 以内。
* `LoweredExpr::Call { kind: FunctionCallKind::User(FuncId) }` の `FuncId.0` は `functions` の有効インデックス。
* 名前文字列（`Ident(String)`）は `LoweredExpr` に残らない。

### validate_lowered の仕様

```rust
pub fn validate_lowered(program: &LoweredProgram) -> Result<(), Vec<Diagnostic>>
```

検査内容:

| 検査 | Diagnostic code | 現在の状態 |
|---|---|---|
| top_level_statements に Function が入っていないか | `InvariantViolation` | 未実装 |
| LocalId が範囲内か | `InvariantViolation` | 未実装 |
| FuncId が範囲内か | `InvariantViolation` | 未実装 |
| call arity が params 数と一致するか | `ArityMismatch` | 未実装 |

### Resolver の責務と保証

```rust
pub struct Resolver {
    // 省略: scope stack, function registry
}
```

* `lower_program` を呼んだ後、`LoweredProgram` の不変条件をすべて満たす。
* scope が存在しない名前は `Err(Diagnostic { code: UnresolvedName })` を返す（M1 以降）。
  M0 では panic せず、将来のエラー対応の準備として `None` を返す。

## MIR — Mid-level IR（M2 以降）

### 責務

* runtime ABI と value representation に寄せた表現。
* `RawValue`, `HeapPtr` を直接扱う。
* `CallRuntime(RuntimeFn::Add)`, `AllocString(len)`, `ReadHeap(offset)` など。

### 不変条件（予定）

* HIR の semantic operation はすべて runtime call か wasm primitive に落とされている。
* JS の動的分岐（`typeof`, `instanceof` など）は runtime call に変換済み。
* `RawValue` は `runtime/value.rs` で定義された tagged encoding のみ。

## Wasm IR（M2 以降）

### 責務

* wasm binary の直接表現。
* `WasmFunc`, `WasmLocal`, `WasmInstr`, `WasmImport`, `DataSegment` などの typed node。
* `wasm-encoder` の入力形式、または独自 `ModuleBuilder` の入力形式。

### 不変条件（予定）

* すべての local は型 (`i32` / `i64` / `f32` / `f64`) を持つ。
* function index は `FuncId` の u32 値に対応する。
* 生成された wasm binary は `wasm-tools validate` を通る。

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
6. `docs/12-current-implementation-status.md` を更新する。
