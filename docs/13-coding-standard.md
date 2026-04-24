# Coding Standard

このドキュメントは ts2wasm のコード規約を定める。
compiler path のすべての変更はこの規約に従う。

## 1. パニック禁止ポリシー

compiler path（Lexer / Parser / Resolver / Lowering / Backend）で `panic!`, `unwrap()`, `expect()` を使わない。

```rust
// 禁止
let val = map.get(key).unwrap();
let val = map.get(key).expect("must exist");
panic!("unreachable");

// 許可: 内部不変条件違反のみ bug! マクロ経由
bug!("LocalId {} is out of bounds", id.0);
```

`bug!` マクロは「compiler のバグであり、入力には起因しない」場合にのみ使う。
入力起因の問題は必ず `Diagnostic` を返す。

## 2. Result / Diagnostic ポリシー

Lexer / Parser / Resolver / Lowering / Backend はすべて `Result<T, Diagnostic>` を返す。

```rust
// 禁止
fn emit_expr(expr: &LoweredExpr) -> String

// 推奨
fn emit_expr(expr: &LoweredExpr, f: &mut FunctionBuilder) -> Result<(), Diagnostic>
```

`String` エラーは CLI の境界（`main` / `build_file` の呼び出し元）だけで使う。
内部では `Diagnostic` を構造化して伝播させる。

### Diagnostic の最小構造

```rust
pub struct Diagnostic {
    pub span: Span,
    pub code: DiagCode,
    pub message: String,
    pub notes: Vec<String>,
}

pub enum DiagCode {
    UnresolvedName,
    DuplicateFunction,
    ArityMismatch,
    UnsupportedSyntax,
    InvalidTopLevelReturn,
    // ...
}
```

## 3. Span ポリシー

すべての AST / HIR node は `Span` を持つ。

```rust
pub struct Span {
    pub start: u32,
    pub end: u32,
}

impl Span {
    /// compiler が内部的に生成した node。source 位置なし。
    pub fn generated(reason: &'static str) -> Self { ... }
}
```

synthetic node（lowering が生成する node）は `Span::generated(reason)` を使う。
`Span::generated` は debug / diagnostic 出力で `<generated: reason>` と表示する。

## 4. IR variant 追加ポリシー

IR enum（`LoweredStmt`, `LoweredExpr`, `HirStmt`, `HirExpr` など）に variant を追加する PR は、以下を同時に更新する。

1. validator（`validate_lowered` / `validate_hir` など）
2. debug printer（`Display` / `Debug` impl）
3. snapshot test（`tests/snapshots/`）
4. differential test（`tests/differential/`）
5. `docs/12-current-implementation-status.md`

これらが揃わない PR はマージしない。

## 5. Backend WAT 直書き禁止

backend で WAT 文字列を直接生成しない。

```rust
// 禁止
wat.push_str("(local.get $x)\n");
wat.push_str(&format!("(i32.const {})\n", value));

// 推奨: wasm-encoder か typed module builder を通す
f.instruction(&Instruction::LocalGet(local_id.0));
f.instruction(&Instruction::I32Const(value));
```

例外は WAT snapshot test 用の pretty printer のみ。
production backend は binary emitter を使う。

## 6. Runtime 関数カタログポリシー

runtime 関数は `RuntimeFn` カタログに登録する。

```rust
pub struct RuntimeFn {
    pub name: RuntimeSymbol,
    pub deps: &'static [RuntimeSymbol],
    pub imports: &'static [HostImport],
    pub capabilities: &'static [Capability],
    pub emit: fn(&mut ModuleBuilder),
}
```

raw WAT template へ直接追記しない。
`console.log` を使った時だけ `fd_write`, `$write`, `$log`, `$value_to_string_into` がリンクされるようにする（tree-shaking）。

## 7. Host Import ポリシー

host import は capability manifest から生成する。
backend が `fd_write` などを直接 import 文字列として持たない。

```rust
// 禁止
wat.push_str("(import \"wasi_snapshot_preview1\" \"fd_write\" ...)");

// 推奨: capability manifest から生成
module.import_from_manifest(&program.required_capabilities);
```

## 8. Value Representation ポリシー

value representation は `runtime/value.rs` のみが定義する。
tag mask / heap mask / layout offset を backend に散らさない。

```rust
// 禁止 (backend 側で直接マスク計算)
let tagged = (value << 3) | 4;

// 推奨
let tagged = ValueTag::encode_number(value);
```

## 9. Feature Gate ポリシー

機能追加は必ず feature gate と compatibility level を持つ。
unsupported case は「黙って壊れる」ではなく diagnostic にする。

```rust
// 禁止
Expr::For(_) => {} // silently skip

// 推奨
Expr::For(_) => {
    return Err(Diagnostic {
        span: node.span,
        code: DiagCode::UnsupportedSyntax,
        message: "for loop is not supported in M0".to_string(),
        notes: vec!["planned for M2+".to_string()],
    });
}
```

## 10. Golden Test ポリシー

golden test は 3 段階に分ける。

```text
1. parse snapshot
   入力 source → AST の text repr を snapshot として保存する。
   AST が変わったら必ず更新する。

2. IR snapshot
   入力 source → LoweredProgram / HirProgram の text repr を snapshot として保存する。
   lowering / resolution が変わったら必ず更新する。

3. wasm execution differential
   入力 source → Node.js 実行結果 と wasm/iwasm 実行結果を比較する。
   runtime semantics が変わったら必ず更新する。
```

## 11. M0 Small-Int 制約の明文化

現在の `ValueTag` は M0 small-int tagged value である。

```text
M0 数値表現:
  i32 の範囲内の整数のみ対応。
  tagged representation: (value << 3) | 4

M0 非対応:
  浮動小数点数（double / f64）
  NaN, Infinity, -Infinity
  大整数（BigInt）
  多桁・負数・小数の文字列変換
```

JavaScript の `number` は本来 IEEE 754 double である。
M0 では意図的に small-int subset に限定している。
この制約は `docs/12-current-implementation-status.md` と `docs/05-compatibility-and-semantics.md` にも記載する。

## 12. console.log API 方針

`Stmt::ConsoleLog(Expr)` という parser special form をやめる。
将来的には member access + call expression + semantic pass による resolution に移行する。

```text
段階的移行:
  M0: Stmt::ConsoleLog は互換のため保持するが、新規追加しない。
  M1: Expr::Member + Expr::Call を追加し、semantic pass で BuiltinCall に落とす。
  M2: Stmt::ConsoleLog を削除。

semantic resolution:
  console.log(x) → BuiltinCall(ConsoleLog, [x]) → RuntimeFn::Log + Capability::Stdout
  Math.max(a, b) → BuiltinCall(MathMax, [a, b]) → RuntimeFn::MathMax
```

## 追加設計: Validation Gates

各 IR 段階に validate 関数を置く。

```rust
pub fn validate_ast(ast: &[Stmt]) -> Result<(), Vec<Diagnostic>>;
pub fn validate_hir(hir: &HirProgram) -> Result<(), Vec<Diagnostic>>;
pub fn validate_lowered(program: &LoweredProgram) -> Result<(), Vec<Diagnostic>>;
```

validate が通らない限り次の phase に進まない。
現在 unsupported な検査:

* call arity check
* top-level `return` が `_start` に入らないか
* duplicate function definition
* unresolved name

これらは現在 panic / silent pass しているため、最優先で validate に移行する。
