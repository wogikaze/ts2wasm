# Architecture Decoupling and LLM-Friendly Sizing

この文書は、現状の結合ポイントを分析し、phase boundary × semantic domain boundary × capability boundary
の三重分離を段階的に達成する設計を定義する。

目標は、LLM のコンテキストウィンドウに収まる粒度で各 module を独立させ、
1 機能追加に必要な読み取り範囲を最小化することである。

## 1. 現状の主要な結合ポイント

| 症状 | 根拠 | リスク |
|---|---|---|
| `backend-wasm` が `frontend` に依存 | `backend-wasm/Cargo.toml`, `src/lib.rs` | backend が構文層の型・診断型に引きずられる |
| `ir` が `frontend` に依存 | `ir/Cargo.toml`, `ir/src/lowered/types.rs` | IR が parser/syntax 表現から独立しきれない |
| `compiler/src/lib.rs` が driver/I/O/module rewrite/validation/emit を抱える | `compiler/src/lib.rs:84-167` | 1 変更で広範囲を読む必要がある |
| `Resolver` の状態が巨大 | `ir/src/lowered/resolver.rs:5-50` | array/class/module/capture/private field 等が密結合 |
| `lower_expr` が 2700 行級 | `ir/src/lowered/resolver_expr.rs` | 1 機能追加が全式 lowering の文脈を要求 |
| `RuntimeFn` が 288 variant | `backend-wasm/src/runtime_fn.rs` | runtime catalog 追加が巨大 match/spec に波及 |
| `runtime_builder.rs` に 300 行超の巨大 dispatch | `backend-wasm/src/runtime_builder.rs:5-319` | runtime domain ごとの独立性が低い |
| `LoweredExpr::RuntimeCall` が `String` | `ir/src/lowered/types.rs:358-361` | docs/13 の「runtime 関数名を文字列リテラルで持つな」に反する |
| `include!` による物理分割 | `ir/src/lowered.rs:1-4` | ファイルが分かれても module 境界になっていない |
| line limit が 4100 と緩い | `scripts/check/architecture-rules.py:20-21` | LLM コンテキスト基準では巨大ファイルを許容しすぎる |

特に重要な Anti-pattern:

```rust
RuntimeCall {
    runtime_fn: String,  // typo・未登録 runtime・capability 漏れが起きる
    args: Vec<LoweredExpr>,
    span: Span,
}
```

runtime は `RuntimeFn` catalog と `RuntimeLinkPlan` を正本にしているのに、
IR 側で runtime function が `String` になっている。ここは `RuntimeIntrinsic` /
`RuntimeOp` / `RuntimeFnId` のような型にすべき。

## 2. 三重分離の設計

普通の compiler なら frontend → semantic → lowering → backend で十分に見えるが、
このプロジェクトは TypeScript/JavaScript → WASM であるため、実際には以下の複数軸が絡む。

- 構文
- 名前解決
- TypeScript 消去
- JavaScript 意味論
- builtin API
- runtime ABI
- WASM layout
- host capability
- manifest/security
- differential testing

したがって必要なのは、単なる layer architecture ではなく、
**phase boundary × semantic domain boundary × capability boundary** の三重分離である。

### 2.1. Phase boundary（「いつ決めるか」の境界）

```
Parser:
  構文だけ読む

NameResolver:
  名前・scope・binding だけ解決する

BuiltinResolver:
  console.log / Math.* / fs.* など API 意味だけ決める

HIR:
  JavaScript の意味論を表す

MIR:
  runtime ABI 呼び出しへ落とす

RuntimeLinkPlan:
  必要 RuntimeFn / import / capability / runtime string を決める

Backend:
  validate 済み IR を wasm/WAT に符号化する
```

### 2.2. Semantic domain boundary（「何の意味論か」の境界）

- array semantics
- object semantics
- function / closure semantics
- class / private field semantics
- module semantics
- builtin / host API semantics
- async / completion record semantics
- string / regexp semantics
- number / bigint semantics

現在の `Resolver` と `RuntimeFn` はこの semantic domain が横に全部入っている。
domain ごとに module を切り、独立したファイルで管理する。

### 2.3. Capability boundary（「外部能力を要求するか」の境界）

- WASI stdout / stdin / env / args
- filesystem read/write
- clock / random
- Node host shim

これはすでに `RuntimeLinkPlan` と manifest に寄せる方向が正しい。伸ばす。

## 3. 目標アーキテクチャ

最終的な依存方向:

```
ts2wasm-source          Span / SourceId / FileId / SourceMap
ts2wasm-diagnostic      Diagnostic / DiagCode / DiagnosticOrigin
ts2wasm-syntax          Token / AST / parser-owned syntax model
ts2wasm-frontend        Lexer / Parser
ts2wasm-resolve         NameResolved AST / symbol table
ts2wasm-semantics       BuiltinResolved / TypeScript erasure / JS semantic HIR
ts2wasm-ir              HIR / MIR / validators / typed ids
ts2wasm-runtime-abi     RawValue / Layout / logical ABI
ts2wasm-runtime-catalog RuntimeFn / RuntimeSpec / HostImport / Capability
ts2wasm-backend-core    Wasm module model / writer traits / validated input contracts
ts2wasm-backend-wasm    WAT / binary emission
ts2wasm-compiler        pipeline orchestration only
ts2wasm-cli             args / path / stdout / stderr / exit code only
```

ただし、いきなり crate を増やす必要はない。最初は module と API で分け、
安定したら crate に昇格する。

現実的な移行順:

```
1. shared/source/diagnostic を先に分離
2. IR から frontend 依存を剥がす
3. backend から frontend 依存を剥がす
4. Runtime catalog を backend-wasm 内で domain 分割
5. Runtime catalog を独立 crate 化
6. HIR/MIR/Validated wrapper を導入
7. backend は ValidatedMIR or ValidatedLoweredProgram だけ受ける
```

## 4. LLM コンテキストに収まる粒度の基準

ファイル分割の基準を LOC ではなく**読み込み単位**で決める。

### 4.1. 1 module の上限

| 段階 | 上限 |
|---|---|
| 理想 | 300〜800 LOC |
| 許容 | 1200 LOC |
| 一時的上限 | 1500 LOC |
| 危険 | 2000 LOC 超 |
| 禁止 | 3000 LOC 超 |

現在の `DEFAULT_MAX_FILE_LINES = 4100`（architecture-rules.py）は緩すぎる。
段階的に引き下げる:

```
Phase 1: 4100 → 3000
Phase 2: 3000 → 2000
Phase 3: 2000 → 1500
Phase 4: 1500 → 1200
```

既存巨大ファイルは allowlist に入れ、**新規巨大化を禁止**する。

### 4.2. 1 function の上限

| 範囲 | 上限 |
|---|---|
| 理想 | 20〜80 LOC |
| 許容 | 120 LOC |
| 危険 | 200 LOC 超 |
| 禁止 | 300 LOC 超 |

現在危険域にある関数:

- `lower_expr` — 2711 lines
- `RuntimeFn::spec` — 2318 lines
- `emit_json_parse` — 1357 lines
- `emit_expr` — 921 lines
- `Lexer::tokenize` — 863 lines
- `lower_variable_array_callback_method` — 842 lines
- `emit_statement_with_label` — 756 lines

### 4.3. 1 feature slice の読み取り範囲

例: `Array.prototype.find` を追加する場合。

読むべきファイル:
- docs/feature/array-find.md（存在する場合）
- builtin resolver の array domain
- HIR/MIR の array op 定義
- runtime catalog array domain
- runtime emitter array domain
- link plan test
- differential fixture

読まないで済むべきファイル:
- class private field
- module graph
- bigint
- object descriptor
- regexp
- async
- CLI

これができていないなら、境界が足りない。

## 5. compiler/src/lib.rs の分離

現在の `build_file_with_host_deny` は以下をすべて抱えている:

- ファイル読み込み
- test262 preprocessing
- parser
- module graph
- static import rewrite
- name resolver
- builtin resolver
- semantic validation
- lowering
- manifest 書き出し
- WAT emit
- wasm 書き出し

目標構成:

```
compiler/src/
  lib.rs
  pipeline.rs
  session.rs
  input.rs
  stages/
    parse.rs
    ast_validate.rs
    module_graph.rs
    static_imports.rs
    name_resolve.rs
    builtin_resolve.rs
    semantic_validate.rs
    lower.rs
    lowered_validate.rs
    runtime_gate.rs
    emit.rs
  io/
    read_source.rs
    write_output.rs
```

`compile_source` を純粋関数寄りにする:

```rust
pub fn compile_source(
    source: &str,
    options: CompileOptions,
) -> Result<CompileReport<CompiledModule>, Diagnostic>
```

ファイル I/O は外側に分離:

```rust
pub fn build_file(input: &Path, output: &Path) -> Result<CompileReport<()>, Diagnostic> {
    let source = read_source(input)?;
    let compiled = compile_source(&source, options)?;
    write_output(output, compiled)?;
    Ok(...)
}
```

## 6. Validated\<T\> の導入

現在は `emit_wat(program: &LoweredProgram)` の中で `validate_lowered` を
呼んでいる。backend が validate 済みしか受けないことを型で表す。

```rust
pub struct Validated<T> {
    inner: T,
}

impl Validated<LoweredProgram> {
    pub fn new(program: LoweredProgram) -> Result<Self, Vec<Diagnostic>> {
        validate_lowered(&program)?;
        Ok(Self { inner: program })
    }

    pub fn as_ref(&self) -> &LoweredProgram {
        &self.inner
    }
}
```

backend API:

```rust
pub fn emit_wat(program: &Validated<LoweredProgram>) -> Result<String, Diagnostic>
```

将来の拡張:

```rust
Validated<Ast>
Validated<NameResolvedProgram>
Validated<HirProgram>
Validated<MirProgram>
Validated<RuntimeLinkPlan>
```

## 7. Resolver context の分割

現在の `Resolver` は scope / function / captures / class / private fields /
module / array facts / bigint facts / regexp facts / string literal facts が
同じ struct に入っている。

```rust
pub struct LoweringCtx<'a> {
    pub symbols: SymbolEnv<'a>,
    pub locals: LocalAllocator,
    pub functions: FunctionRegistry<'a>,
    pub captures: CaptureEnv<'a>,
    pub classes: ClassEnv,
    pub modules: ModuleEnv,
    pub facts: StaticFacts,
    pub diagnostics: DiagnosticSink,
}
```

domain module ごとに関数として切り出す。いきなり trait object に逃げず、
`Resolver` の method ではなく domain module の関数にする:

```rust
pub(crate) fn lower_array_literal(
    ctx: &mut LoweringCtx,
    elements: &[ResolvedArrayElement],
) -> Result<LoweredExpr, Diagnostic>
```

`lower_expr` の branch を domain module に移す:

```
crates/ir/src/lowered/
  mod.rs
  types.rs
  validate.rs
  lower.rs
  ctx.rs
  local_alloc.rs
  symbols.rs
  functions.rs
  captures.rs
  facts.rs
  modules.rs
  class/
    mod.rs
    private_fields.rs
    heritage.rs
    constructors.rs
  array/
    mod.rs
    literal.rs
    callbacks.rs
    spread.rs
  object/
    mod.rs
    literal.rs
    property.rs
    descriptors.rs
  call/
    mod.rs
    user_call.rs
    builtin_call.rs
    method_call.rs
  control/
    mod.rs
    loops.rs
    try_finally.rs
    completion.rs
```

## 8. include! を real module に置き換える

現在:

```rust
include!("lowered/types.rs");
include!("lowered/program.rs");
include!("lowered/resolver.rs");
include!("lowered/validate.rs");
```

これでは `super::*` 的に全部が同じ namespace に沈む。

```rust
pub mod types;
pub mod program;
pub mod validate;

mod lower;
mod ctx;
mod array;
mod object;
mod class;
mod call;
mod control;

pub use types::{
    LoweredProgram,
    LoweredStmt,
    LoweredExpr,
    LocalId,
    FuncId,
};

pub use validate::validate_lowered;
pub use lower::lower_program;
```

`pub use` するものを絞り、外から見えるものを contract として明示する。

## 9. Runtime catalog の domain 分割

`RuntimeFn` が 288 variant あるのはプロジェクトの性質上ある程度仕方ない。
問題は spec / emission_order / all / manifest_name / runtime builder dispatch
が全部巨大 match になっていること。

### 9.1. Domain の導入

```rust
pub enum RuntimeDomain {
    Core,
    Array,
    Object,
    String,
    BigInt,
    Date,
    Host,
    Promise,
}

impl RuntimeFn {
    pub const fn domain(self) -> RuntimeDomain { ... }
}
```

### 9.2. Domain dispatch の builder

```rust
pub(super) fn emit_runtime(&mut self, wat: &mut String) {
    self.emit_utf8_helpers(wat);

    for runtime_fn in RuntimeFn::emission_order() {
        if !self.link_plan.required_runtime_functions().contains(runtime_fn) {
            continue;
        }

        match runtime_fn.domain() {
            RuntimeDomain::Core => core::emit(self, *runtime_fn, wat),
            RuntimeDomain::Array => array::emit(self, *runtime_fn, wat),
            RuntimeDomain::Object => object::emit(self, *runtime_fn, wat),
            RuntimeDomain::String => string::emit(self, *runtime_fn, wat),
            RuntimeDomain::BigInt => bigint::emit(self, *runtime_fn, wat),
            RuntimeDomain::Host => host::emit(self, *runtime_fn, wat),
            RuntimeDomain::Promise => promise::emit(self, *runtime_fn, wat),
        }
    }
}
```

### 9.3. Domain 側の小さい match

```rust
pub(super) fn emit(emitter: &mut WatEmitter, f: RuntimeFn, wat: &mut String) {
    match f {
        RuntimeFn::ArrayGet => emitter.emit_array_get(wat),
        RuntimeFn::ArrayMap => emitter.emit_array_map(wat),
        RuntimeFn::ArrayFilter => emitter.emit_array_filter(wat),
        _ => unreachable!("non-array runtime fn routed to array emitter"),
    }
}
```

### 9.4. ファイル構成

```
crates/backend-wasm/src/runtime/
  mod.rs
  catalog.rs
  domain.rs
  core/
    mod.rs
    catalog.rs
    emit.rs
  array/
    mod.rs
    catalog.rs
    emit.rs
  object/
    mod.rs
    catalog.rs
    emit.rs
  string/
    mod.rs
    catalog.rs
    emit.rs
  bigint/
    mod.rs
    catalog.rs
    emit.rs
  date/
    mod.rs
    catalog.rs
    emit.rs
  host/
    mod.rs
    catalog.rs
    emit.rs
  promise/
    mod.rs
    catalog.rs
    emit.rs
```

## 10. RuntimeCall の String を typed intrinsic に置き換える

```rust
// Before
RuntimeCall {
    runtime_fn: String,
    args: Vec<LoweredExpr>,
    span: Span,
}

// After
pub enum RuntimeIntrinsic {
    ArrayConcat,
    ArrayMap,
    ObjectKeys,
    GetIterator,
    IteratorNext,
    HeapClosureCall,
    // ...
}
```

選択肢:

| 案 | 内容 | 評価 |
|---|---|---|
| A: `RuntimeFn` を `runtime-catalog` crate に移す | IR から直接参照可能 | 長期で最もきれい |
| B: IR 用に `RuntimeIntrinsic` を作る | mapping layer が必要 | 短期向け |

`RuntimeFn` は capability/import/link plan の正本であるため、長期的には案 A が良い。

## 11. HIR / MIR / Wasm IR の責務明確化

docs/13 の HIR/MIR/Wasm IR 構想を具体化する。

### 11.1. HIR — JS の意味論を表す

```rust
enum HirExpr {
    Const(Value),
    LoadLocal(LocalId),
    StoreLocal(LocalId, Box<HirExpr>),

    JsAdd(Box<HirExpr>, Box<HirExpr>),
    JsStrictEqual(Box<HirExpr>, Box<HirExpr>),
    ToBoolean(Box<HirExpr>),

    GetProp { object: Box<HirExpr>, key: PropertyKey },
    SetProp { object: Box<HirExpr>, key: PropertyKey, value: Box<HirExpr> },

    CallMethod { receiver: Box<HirExpr>, key: PropertyKey, args: Vec<HirExpr> },
}
```

ここでは WASM layout を知らない。

### 11.2. MIR — runtime ABI に寄せる

```rust
enum MirExpr {
    RawConst(RawValue),
    Local(MirLocal),
    CallRuntime { func: RuntimeFn, args: Vec<MirExpr> },
    WasmPrimitive { op: WasmPrimOp, args: Vec<MirExpr> },
}
```

ここでは `RuntimeFn` が出てよい。

### 11.3. Wasm IR — wasm 命令列

```rust
enum WasmInstr {
    I32Const(i32),
    LocalGet(WasmLocal),
    Call(WasmFunc),
    Block(Vec<WasmInstr>),
}
```

WAT 文字列は最後だけ。

## 12. Backend を符号化器にする

docs にも raw WAT 直書き禁止方針はあるが、WAT 文字列生成は壊れやすい。
括弧・stack discipline・indent・型・call signature を文字列で管理するから。

短期: `WatWriter` を強化する。

```rust
writer.call_runtime(RuntimeFn::Add);
writer.local_get(local);
writer.i32_const(1);
writer.block(|w| {
    ...
});
```

中期: stack effect を持たせる。

```rust
RuntimeFn::Add.signature()
// [JsVal, JsVal] -> [JsVal]
```

長期:

```
HIR → MIR → Wasm IR → wasm-encoder → .wasm
```

WAT は debug dump に下げる。

## 13. Diagnostic を frontend から分離する

現在は `Diagnostic`, `DiagCode`, `Span` が frontend にあり、
backend と IR が frontend に依存している。これは層の逆流。

分離先:

```
crates/source
  Span
  SourceId
  SourceMap

crates/diagnostic
  DiagCode
  Diagnostic
  DiagnosticOrigin
```

または既存 `shared` に一時的に入れる。

```rust
pub enum DiagnosticOrigin {
    Source(Span),
    Generated(&'static str),
    Internal,
}
```

`Option<Span>` を減らす:

```rust
pub struct Diagnostic {
    pub code: DiagCode,
    pub message: String,
    pub origin: DiagnosticOrigin,
    pub phase: Phase,
}
```

## 14. 機能追加の vertical slice checklist

機能追加は必ず以下を checklist にする。

```
Feature: <name>

1. Syntax impact
   - parser 変更が必要か？
   - AST variant が必要か？

2. Builtin resolution
   - source pattern
   - arity
   - receiver semantics
   - unsupported diagnostics

3. HIR
   - semantic op
   - validation
   - snapshot

4. MIR / Lowered IR
   - RuntimeFn mapping
   - local/call invariants

5. Runtime catalog
   - RuntimeFn variant
   - RuntimeSpec
   - deps
   - imports
   - capabilities
   - runtime strings

6. Runtime emission
   - domain emitter
   - WAT/wasm validation

7. Link plan test
   - required runtime functions
   - no unnecessary imports

8. Differential test
   - Node vs iwasm

9. Negative tests
   - unsupported callback shape
   - unsupported receiver
   - arity mismatch

10. Docs/current state
```

## 15. Architecture fitness functions

docs に原則を書くより CI で破らせない方が強い。

### 追加すべき check

```text
1. backend-wasm must not depend on frontend
2. ir must not depend on frontend except temporary allowlist
3. no include! in crates/ir/src/lowered.rs
4. no RuntimeCall { runtime_fn: String }
5. no use super::* outside tests
6. no function > 200 lines
7. no file > staged threshold
8. no new Diagnostic { span: None } for source diagnostics
9. no backend module imports Stmt/Expr from frontend
10. no raw "$runtime_symbol" string outside runtime catalog
11. no wat.push_str in new runtime helper files
12. RuntimeFn::all contains every RuntimeFn variant
13. RuntimeFn::emission_order contains every emitted function exactly once
14. every RuntimeFn has RuntimeSpec
15. every RuntimeFn with imports has capability or explicit no-capability marker
16. every host import appears in manifest tests
17. every LoweredExpr variant is covered by validate_lowered
18. every HIR/MIR variant is covered by dump/snapshot printer
```

## 16. Coupling の指標

感覚ではなく数値で管理する。

| 指標 | 目標 | 危険 |
|---|---|---|
| fan-out（1 module の import 数） | 5 以下 | 10 超 |
| public API count（1 module の公開 item 数） | 5〜15 | 30 超 |
| function length | 120 行以下 | 200 行超 |
| match size | 20 arms 以下 | 50 arms 超 |
| change amplification（1 feature 追加の触るファイル数） | 5〜8 files | 20 files 超 |
| context load（1 作業に必要な読み込み LOC） | 3000 LOC 以下 | 10000 LOC 超 |

`RuntimeFn` のように巨大 enum が避けられない場合は、domain dispatch で match を分散する。

## 17. 分割の原則

良い分割:

```text
同じ理由で変わるものを同じ場所に置く。
違う理由で変わるものを離す。
```

このプロジェクトでは変更理由はおおよそ以下に分類される:

- syntax grammar が変わる
- name resolution が変わる
- builtin API contract が変わる
- JS semantics が変わる
- runtime ABI が変わる
- WASM layout が変わる
- host capability が変わる
- test oracle が変わる

例えば `Array.prototype.map` は array semantics、`private field` は class/private semantics、
`fs.readFileSync` は host API + capability。同じ lowering だからといって同じ巨大 resolver に
入れ続けるべきではない。

## 18. Object Semantics Kernel と Completion Records

`docs/21-object-semantics-kernel.md` と `docs/22-completion-records.md` の方向性は良い。

この 2 つは複雑な JS semantics を散らさないための semantic kernel になる。

### Object kernel

`obj.x`, `obj[x]`, `delete obj.x`, `"x" in obj`, `Object.keys`, class prototype などを、
最終的に共通の internal operation に寄せる。

- OrdinaryGet
- OrdinarySet
- OrdinaryHasProperty
- OrdinaryDelete
- OrdinaryDefineOwnProperty
- OrdinaryGetOwnProperty
- OrdinaryOwnPropertyKeys
- OrdinaryGetPrototypeOf
- OrdinarySetPrototypeOf

### Completion Record

`return`, `throw`, `break`, `continue`, `try/finally`, async を `status/value/target`
で統一する。特に JS の `return` を WASM の `return` に即変換すると `finally` が壊れる。
Completion Record はこの事故を防ぐ。

これらの semantic kernel は IR に早めに反映すべき。

## 19. テスト戦略

Node vs iwasm differential は重要だが、それだけでは内部構造の結合を防げない。

必要なテスト層:

- Parser snapshot: source → AST
- Name resolver snapshot: AST → named representation
- Builtin resolver contract: source pattern → BuiltinId / arity / diagnostics
- HIR snapshot: resolved → semantic operations
- HIR validator negative tests: invalid IDs / receiver loss / unresolved name
- MIR / Lowered snapshot: semantic op → runtime call / local IDs
- RuntimeLinkPlan structure test: RuntimeFn / imports / capabilities / strings
- WAT / wasm validation: emitted module is structurally valid
- Differential execution: Node stdout/stderr/exit code vs iwasm
- Manifest snapshot: capability manifest is canonical and minimal

特に `RuntimeLinkPlan structure test` が重要。「実行結果が同じ」でも不要な host import
が入っていたら security model と standalone 目標に反する。

## 20. Refactor の進め方

### 20.1. Branch by abstraction

既存実装を壊さず新しい抽象を横に作る:

```
old direct RuntimeCall string
→ RuntimeIntrinsic enum を追加
→ 一部だけ RuntimeIntrinsic に移す
→ String path を deprecated にする
→ architecture check で新規 String path 禁止
→ 残りを移す
→ String path 削除
```

### 20.2. Strangler fig

巨大ファイルを一気に割らず、機能単位で周辺から置き換える:

```
resolver_expr.rs の lower_expr 巨大 match は残す
→ array literal branch だけ array/literal.rs へ移す
→ class branch だけ class/lower.rs へ移す
→ call branch だけ call/lower.rs へ移す
→ 最後に lower_expr は dispatcher だけにする
```

### 20.3. Characterization tests

リファクタ前に現状の振る舞いを固定する:

```
before:
  existing fixture output
  link plan snapshot
  lowered IR snapshot
refactor:
  output must not change
```

### 20.4. Mikado method

大目標を小依存タスクに分解する:

```
Goal: backend-wasm no longer depends on frontend

Requires:
  Diagnostic moved to shared/diagnostic
  Span moved to shared/source
  DiagCode moved to shared/diagnostic
  frontend re-exports for compatibility
  ir imports shared diagnostic
  backend imports shared diagnostic
  cargo dependency removed
  architecture check added
```

## 21. 直近でやるべき 10 項目（優先度順）

1. **`Span`, `DiagCode`, `Diagnostic` を frontend から外す**
   短期は `shared` に移す。backend-wasm → frontend 依存消去、ir → frontend 依存削減

2. **`Validated<LoweredProgram>` を導入する**
   backend API を `Validated` だけ受ける形にする

3. **`RuntimeCall { runtime_fn: String }` を型に置き換える**
   `RuntimeIntrinsic` を作るか、`RuntimeFn` を catalog crate に移す

4. **`runtime_fn_impl.rs` を domain 分割する**
   まず spec: `runtime/spec/core.rs`, `runtime/spec/array.rs`, ...

5. **`runtime_builder.rs` を domain dispatch にする**
   巨大 match を domain module に逃がす

6. **`compiler/src/lib.rs` から pipeline stage を切り出す**
   まず `compile_source` の純粋関数化

7. **`Resolver` の context を分解する**
   内部 field group を struct 化: `symbols: SymbolEnv, classes: ClassEnv, facts: StaticFacts, ...`

8. **`lower_expr` の branch を domain module に移す**
   最初は array/call/class/object の 4 つ

9. **`include!` を real module に置き換える**
   `pub mod types; mod lower; pub use ...`

10. **architecture checks を強化する**
    新規違反禁止: no new file > 2000 lines, no function > 300 lines, no RuntimeCall string, ...

## 22. 設計スローガン

迷ったらこれに従う:

```
Parser は知らない。
Resolver は決めない。
BuiltinResolver は emit しない。
HIR は layout を知らない。
MIR は syntax を知らない。
RuntimeLinkPlan は WAT を知らない。
Backend は semantics を決めない。
CLI は compiler を知らない。
```

具体化:

```
名前文字列は NameResolver 以降に残さない。
runtime 関数名文字列は IR に残さない。
host import 文字列は backend に直書きしない。
source 起因 error は必ず span を持つ。
compiler bug は InvariantViolation にする。
backend は validate 済み IR 以外を受けない。
capability は RuntimeSpec → RuntimeLinkPlan → Manifest のみで決まる。
```

## 23. 参照

- `docs/12-coding-standard.md`: コード規約
- `docs/13-ir-contracts.md`: IR 契約（HIR/MIR/Wasm IR）
- `docs/14-runtime-abi.md`: RuntimeFn catalog
- `docs/21-object-semantics-kernel.md`: Object semantics kernel
- `docs/22-completion-records.md`: Completion record design
- `scripts/check/architecture-rules.py`: 現行 architecture check
- `crates/shared/`: 現行共有定義（schema/manifest）
