# Architecture Decoupling and LLM-Friendly Sizing

<!-- Status: Implementation tracker for all sections -->
<!--
  ✅ = Done (P9/P10/P11/P13 batch, last audited 2026-05-12)
  🚧 = In progress
  ❌ = Not started
-->

この文書は、現状の結合ポイントを分析し、phase boundary × semantic domain boundary × capability boundary
の三重分離を段階的に達成する設計を定義する。

目標は、LLM のコンテキストウィンドウに収まる粒度で各 module を独立させ、
1 機能追加に必要な読み取り範囲を最小化することである。

## 1. 現状の主要な結合ポイント (✅ 一部改善)

| 症状 | 根拠 | リスク | 状態 |
|---|---|---|---|
| `backend-wasm` が `frontend` に依存 | `backend-wasm/Cargo.toml`, `src/lib.rs` | backend が構文層の型・診断型に引きずられる | ✅ P13: dependency 削除済み。`check_backend_frontend_import` で監視 |
| `ir` が `frontend` に依存 | `ir/Cargo.toml`, `ir/src/lowered/types.rs` | IR が parser/syntax 表現から独立しきれない | ✅ P9: [dev-dependencies] のみ, 全 import 移行済み |
| `compiler/src/lib.rs` が driver/I/O/module rewrite/validation/emit を抱える | `compiler/src/lib.rs` → **82 lines**, `pipeline.rs` → **107 lines** | 1 変更で広範囲を読む必要がある | ✅ P13: `pipeline.rs`, `io/`, `stages/` に分割済み |
| `Resolver` の状態が巨大 | `ir/src/lowered/resolver.rs:5-50` → **分割済み** | array/class/module/capture/private field 等が密結合 | 🚧 P10: `LoweringCtx`, `SymbolEnv`, `ClassEnv`, `StaticFacts` 作成。`extra.rs` から `function.rs`/`string.rs`/`module.rs` 抽出済み |
| `lower_expr` が 2700 行級 | `ir/src/lowered/resolver/expr/mod.rs` → **200 lines** | 1 機能追加が全式 lowering の文脈を要求 | ✅ P13: `resolver/expr/*` と domain modules に分割済み |
| `RuntimeFn` が 288+ variant | `runtime-catalog/src/runtime_fn.rs` | runtime catalog 追加が巨大 match/spec に波及 | ✅ P13: `ts2wasm-runtime-catalog` crate に抽出済み。巨大 registry は catalog 境界に限定 |
| `runtime_builder.rs` に 300 行超の巨大 dispatch | `backend-wasm/src/runtime_builder.rs:5-319` → **55 lines** | runtime domain ごとの独立性が低い | ✅ P10: `RuntimeDomain` enum で domain dispatch 化 |
| `LoweredExpr::RuntimeCall` が `String` | `ir/src/lowered/types.rs:365-369` → `intrinsic: RuntimeFn` | docs/13 の「runtime 関数名を文字列リテラルで持つな」に反する | ✅ P13: runtime-catalog の `RuntimeFn` を直接参照 |
| `include!` による物理分割 | `ir/src/lowered.rs:1-4` | ファイルが分かれても module 境界になっていない | ✅ P9: 全 include! 削除, real module 化完了 |
| line limit が 4100 と緩い | `scripts/check/architecture-rules.py` | LLM コンテキスト基準では巨大ファイルを許容しすぎる | ✅ P11: staged 2000/1500 line checks と 200-line function warning を追加 |

特に重要な Anti-pattern:

```rust
RuntimeCall {
    intrinsic: RuntimeFn,  // 元は runtime_fn: String → RuntimeIntrinsic → RuntimeFn
    args: Vec<LoweredExpr>,
    span: Span,
}
```

`RuntimeFn` は `crates/runtime-catalog/src/runtime_fn.rs` で定義される。
IR は runtime 関数名を文字列として保持せず、runtime-catalog の typed catalog を直接参照する。

→ ✅ **P13 で `RuntimeIntrinsic` 中間層も削除済み。**

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

- array semantics ✅ 一部対応 (P9: builtin_domain/array, resolver/call.rs, array/iteration 等)
- object semantics ❌ (builtin_domain/object は作成済み, resolver/object.rs は stub)
- function / closure semantics ✅ P10: `resolver/function.rs` 抽出済み
- class / private field semantics ✅ 一部対応 (P9: builtin_domain/class, resolver/class.rs)
- module semantics ✅ P10: `resolver/module.rs` 抽出済み
- builtin / host API semantics ✅ 一部対応 (P9: builtin_domain 全5 domain 作成)
- async / completion record semantics 🚧 P10: `lowered/completion.rs` stub 作成済み
- string / regexp semantics ✅ P10: `resolver/string.rs` 抽出済み
- number / bigint semantics ✅ 一部対応 (P9: builtin_domain/number)

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
ts2wasm-source          Span / SourceId / SourceMap
ts2wasm-diagnostic      Diagnostic / DiagCode / DiagnosticOrigin
ts2wasm-syntax          Token / AST / parser-owned syntax model
ts2wasm-frontend        Lexer / Parser
ts2wasm-resolve         NameResolved AST / symbol table
ts2wasm-semantics       BuiltinResolved / TypeScript erasure / JS semantic HIR policy
ts2wasm-ir              HIR / MIR / Lowered IR / validators / typed ids
ts2wasm-runtime-abi     RawValue / Layout / logical ABI
ts2wasm-runtime-catalog RuntimeFn / RuntimeSpec / HostImport / Capability / RuntimeLinkPlan
ts2wasm-backend-core    Wasm module model / writer traits / validated input contracts
ts2wasm-backend-wasm    WAT / binary emission
ts2wasm-compiler        pipeline orchestration only
ts2wasm-cli             args / path / stdout / stderr / exit code only
```

既に crate 化された境界は crate 境界を正本にする。まだ crate 化していない境界は
module と API で分け、安定したら crate に昇格する。

現実的な移行順:

```
1. shared/source/diagnostic を先に分離       ✅ P9: shared に移動済み
2. IR から frontend 依存を剥がす             ✅ P9: [dev-dependencies] のみ, 全 import 移行
3. backend から frontend 依存を剥がす         ✅ P13: dependency 削除済み
4. Runtime catalog を backend-wasm 内で domain 分割  ✅ P10: spec/catalog/emit 分割済み
5. Runtime catalog を独立 crate 化           ✅ P11/P13: `crates/runtime-catalog`
6. HIR/MIR/Validated wrapper を導入          ✅ P13: HIR/MIR validators + wrappers 追加済み
7. backend は ValidatedMIR or ValidatedLoweredProgram だけ受ける ✅ P13: public backend API は `Validated<LoweredProgram>`
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

`architecture-rules.py` は P10/P11 で段階的な line limit を持つ。
段階的に引き下げる:

```
Phase 1: 4100 → 3000     ✅ P10: 改定済み, arch check 追加
Phase 2: 3000 → 2000     🚧 P11: allowlist 付き check 追加
Phase 3: 2000 → 1500     🚧 P11: allowlist 付き check 追加
Phase 4: 1500 → 1200     ❌ 未実施
```

`check_rust_file_length(2000)` と `check_rust_file_length_1500(1500)` は
allowlist を使って既存巨大ファイルと新規巨大化を分ける。

既存巨大ファイルは allowlist に入れ、**新規巨大化を禁止**する。

### 4.2. 1 function の上限

| 範囲 | 上限 |
|---|---|
| 理想 | 20〜80 LOC |
| 許容 | 120 LOC |
| 危険 | 200 LOC 超 |
| 禁止 | 300 LOC 超 |

現在危険域にある関数:

- `lower_expr` — ~~2711 lines~~ ✅ P13: dispatcher は `resolver/expr/mod.rs` 配下で **200 lines**。branch は `resolver/expr/*` に分割
- `lower_method_call_expr` — ~~1223 lines~~ ✅ P13: `resolver/call/method.rs` に分離済み
- `emit_json_parse` — ~~1357 lines~~ ✅ P13: `runtime/json/{parser,value,string,serializer,error}.rs` に分割
- `RuntimeFn::spec` — 巨大 registry は `runtime-catalog/src/runtime_fn.rs` と generated-style `runtime/spec/all.rs` に隔離
- `emit_expr` — 🚧 `backend-wasm/src/expr_emit.rs` は legacy allowlist。新規 semantic branch は domain helper へ切る
- `Lexer::tokenize` — ~~863 lines~~ ✅ P10: **268 lines** (4 の sub-method に分割)
- `lower_variable_array_callback_method` — ~~842 lines~~ ✅ P10: **250 lines** (8 の sub-method に分割)
- `emit_statement_with_label` — ~~756 lines~~ ✅ P10: **258 lines** (10 の sub-function に分割)

### 4.3. 1 feature slice の読み取り範囲

例: `Array.prototype.find` を追加する場合。

読むべきファイル:
- docs/feature/array-find.md（存在する場合）
- builtin resolver の array domain   ✅ P9: builtin_domain/array.rs に分離済み
- HIR/MIR の array op 定義           ❌ 未分離
- runtime catalog array domain        ✅ P9: array の emit は 4 ファイルに分割済み
- runtime emitter array domain        ✅
- link plan test                      ❌
- differential fixture                ⚪ 既存

読まないで済むべきファイル:
- class private field                ✅ P9: resolver/call.rs に分離, expr.rs では読まない
- module graph                        ✅
- bigint                              ✅
- object descriptor                   ✅
- regexp                              ✅
- async                               ✅
- CLI                                 ✅

これができていないなら、境界が足りない。

## 5. compiler/src/lib.rs の分離 ✅ P13: pipeline/io/stages 分割

以前の `build_file_with_host_deny` は以下をすべて抱えていた:

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

現在の分割:

```
compiler/src/
  lib.rs                          ← 82 lines (API surface/re-export)
  pipeline.rs                     ← build flow orchestration
  io/
    read_source.rs
    write_manifest.rs
    write_output.rs
  stages/
    parse.rs                      ← parse + collect_diagnostics
    module_graph.rs               ← module graph stage facade
    static_imports.rs             ← static import lowering facade
    name_resolve.rs               ← name resolution
    builtin_resolve.rs            ← builtin resolution
    semantic_validate.rs          ← semantic validation
    lower.rs                      ← lowering
    validate.rs                   ← validation + Validated wrapper
    lowered_validate.rs           ← lowered validation
    runtime_gate.rs               ← runtime/capability gate
    emit.rs                       ← wasm emission + I/O
```

`lib.rs` の行数: ~~906~~ → **82 lines**。pipeline と I/O は分離済み。

**まだ残る設計上の注意:**

```
compiler crate は orchestration と diagnostic aggregation の責務を持つ。
semantic decision は ir/semantics、encoding decision は backend-wasm/backend-core に置く。
```

`pipeline.rs` は flow を読ませるための薄い orchestration に留める。各 stage の
input/output contract を広げる場合は、その stage の focused test を先に増やす。

## 6. Validated\<T\> の導入 ✅ P13: Lowered/HIR/MIR/RuntimeLinkPlan

`crates/ir/src/lowered/types.rs` で定義:

```rust
pub struct Validated<T> {
    inner: T,
    non_fatal: Vec<Diagnostic>,
}
```

backend API は `emit_wat(program: &Validated<LoweredProgram>)` で強制されている。
P10 で `Validated<T>` struct 本体を実装し、arch check (`check_validated_backend_contract`) で
新規 `pub fn emit*` が `Validated<LoweredProgram>` を強制する。

現在の validated boundary:

```rust
Validated<LoweredProgram>       // backend public emit API
Validated<HirProgram>           // HIR invariant validation
Validated<MirProgram>           // MIR invariant validation
Validated<RuntimeLinkPlan>      // runtime fn / import / capability consistency
```

`Validated<RuntimeLinkPlan>` は `crates/runtime-catalog/src/link_plan.rs` にあり、
`ValidatedRuntimeLinkPlan` として manifest emission に渡す。

削除した候補:
- ~~`Validated<Ast>`~~ — AST は parser 直後で validation 不要 (parse error は別 system)。parser が構文エラーを返す。
- ~~`Validated<NameResolvedProgram>`~~ — name resolution の validation は `ResolvedStmt` の
  invariant として表現済み。個別 wrapper は過剰。

## 7. Resolver context の分割 🚧 P10: LoweringCtx + domain modules 完了

現在の `Resolver` は scope / function / captures / class / private fields /
module / array facts / bigint facts / regexp facts / string literal facts が
同じ struct に入っているが、P10 で以下の分割が完了。

```rust
// 既存 (crates/ir/src/lowered/resolver/mod.rs)
pub struct Resolver<'a> { ... }

// P10 追加 (crates/ir/src/lowered/ctx.rs, symbols.rs, classes.rs, facts.rs)
pub struct LoweringCtx<'a> { ... }
pub struct SymbolEnv<'a> { ... }
pub struct ClassEnv { ... }
pub struct StaticFacts { ... }
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
  hir.rs               ✅ P10: 新規作成 (type stub)
  mir.rs               ✅ P10: 新規作成 (type stub)
  ctx.rs               ✅ P10: 新規作成 (LoweringCtx)
  local_alloc.rs
  symbols.rs           ✅ P10: 新規作成 (SymbolEnv)
  classes.rs           ✅ P10: 新規作成 (ClassEnv)
  facts.rs             ✅ P10: 新規作成 (StaticFacts)
  object_kernel.rs     ✅ P10: 新規作成 (OrdinaryGet/Set stubs)
  completion.rs        ✅ P10: 新規作成 (CompletionRecord stubs)
  resolver/
    mod.rs              — Resolver struct, domain module 宣言
    expr/               — Unary/Binary/Ternary/制御構文/property/facts など
    call/               — Call/MethodCall/New/optional/spread/receiver/user/builtin
    array.rs            — ArrayLiteral + callback lowering
    object.rs           — ObjectLiteral lowering (stub)
    class.rs            — ClassExpr lowering (stub)
    function.rs         ✅ P10: arrow/closure/capture 抽出済み
    string.rs           ✅ P10: string/regexp 抽出済み
    module.rs           ✅ P10: module 抽出済み

**なぜ目標より緩いか**: `expr/` は分割済みだが、いくつかの branch は依然として
`&mut Resolver` に密結合しており、pure domain function にはなっていない。
理由:
- `expr/` の branch 間には共通 helper (resolve_local, alloc_temp 等) への依存があり、
  関数 signature がまだ `&mut Resolver` に密結合している。
- `LoweringCtx` struct の field 移行 (現在の Resolver struct の field を ctx に移動) が
  完了しないと、domain module の関数が context なしで呼べない。
```

**現在の resolver/ のファイル構成:**
- `mod.rs` — Resolver struct, 全 field, lower_expr の match dispatch
- `expr/` — assignment/binary/binding/control/facts/literal/property/ternary/unary
- `call/` — builtin/callback/constructor/method/optional/receiver/spread/user
- `array.rs` — ArrayLiteral + callback lowering
- `object.rs` — ObjectLiteral lowering (stub)
- `class.rs` — ClassExpr lowering (stub)
- `function.rs` ✅ — arrow/closure/capture (581 lines)
- `string.rs` ✅ — string literal/regexp
- `module.rs` ✅ — module_id_for_specifier

**残課題**: `Resolver` field 群の `LoweringCtx` への完全移行、`expr/` の branch を
より狭い context function に寄せること、backend-facing LoweredExpr と HIR/MIR の責務整理。

## 8. include! を real module に置き換える ✅ P9: 完了

Before:

```rust
include!("lowered/types.rs");
include!("lowered/program.rs");
include!("lowered/resolver.rs");
include!("lowered/validate.rs");
```

After (現在の実装):

```rust
pub mod program;
pub mod resolver;
pub mod types;
pub mod validate;
```

`pub use` するものを絞り、外から見えるものを contract として明示する。
(現在の `lowered.rs` は `// Replaced include! with real module boundaries` と
コメントあり。include! は 0 個。)

## 9. Runtime catalog の domain 分割 ✅ P13: runtime-catalog crate 抽出済み

`RuntimeFn` が 288+ variant あるのはプロジェクトの性質上ある程度仕方ない。
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

現在の実ファイル構成:

```
crates/runtime-catalog/src/
  lib.rs
  runtime_fn.rs                    ✅ RuntimeFn / RuntimeSpec / signature / registry
  domain.rs                        ✅ RuntimeDomain
  capability.rs                    ✅ Capability
  host_import.rs                   ✅ HostImport / HostImportSpec
  link_plan.rs                     ✅ RuntimeLinkPlan / ValidatedRuntimeLinkPlan
  runtime/
    spec/all.rs                    ✅ generated-style RuntimeSpec registry
    manifest/all.rs                ✅ generated-style manifest registry

crates/backend-wasm/src/runtime/
  mod.rs                          ✅ 存在 (core/array/collections/host/json/object/regexp/string + spec を宣言)
  spec/
    all.rs                        ✅ compatibility shim to runtime-catalog
  manifest/
    all.rs                        ✅ compatibility shim to runtime-catalog
  core/
    mod.rs
    catalog.rs                    ✅ P10
    emit.rs                       ✅ P10: 378 行 (domain submodules に分割)
    arithmetic.rs                 ✅ P10
    bigint.rs                     ✅ P10
    comparison.rs                 ✅ P10
    control.rs                    ✅ P10
    conversion.rs                 ✅ P10
    memory.rs                     ✅ P10
  array/
    mod.rs
    catalog.rs                    ✅ P10
    emit.rs                       ✅ P9: dispatch のみ
    iteration.rs                  ✅ P9
    mutator.rs                    ✅ P9
    accessor.rs                   ✅ P9
    iterator.rs                   ✅ P9
  object/
    mod.rs
    catalog.rs                    ✅ P10
    emit.rs
  string/
    mod.rs
    catalog.rs                    ✅ P10
    emit.rs
  bigint/
    mod.rs
    catalog.rs                    ✅ P10
    emit.rs
  date/
    mod.rs
    catalog.rs                    ✅ P10
    emit.rs
  host/
    mod.rs
    catalog.rs                    ✅ P10
    emit.rs
  promise/
    mod.rs
    catalog.rs                    ✅ P10
    emit.rs
```

`backend-wasm/src/runtime_fn.rs` と `runtime_fn_impl.rs` は互換 shim であり、
正本は `ts2wasm-runtime-catalog` にある。

## 10. RuntimeCall の String を typed catalog に置き換える ✅ P13

```rust
// Before (P9)
RuntimeCall {
    runtime_fn: String,
    args: Vec<LoweredExpr>,
    span: Span,
}

// Intermediate (P10)
RuntimeCall {
    intrinsic: RuntimeIntrinsic,
    args: Vec<LoweredExpr>,
    span: Span,
}

// Current
RuntimeCall {
    intrinsic: RuntimeFn,
    args: Vec<LoweredExpr>,
    span: Span,
}
```

`RuntimeFn` は `crates/runtime-catalog/src/runtime_fn.rs` で定義。
IR は `String` も `RuntimeIntrinsic` 中間層も経由せず、runtime-catalog の
typed catalog を参照する。

arch check `check_no_new_string_runtime_call` で新規 `RuntimeCall { runtime_fn: String }` を禁止。

選択肢比較:

| 案 | 内容 | 評価 | 選ばなかった理由 |
|---|---|---|---|
| A: `RuntimeFn` を `runtime-catalog` crate に移す | IR から直接 `RuntimeFn` 参照 | 長期で最もきれい | ✅ **P13 で採用。** `RuntimeIntrinsic` を廃止し、catalog を正本化。 |
| B: IR 用に `RuntimeIntrinsic` を作る | mapping layer が必要 | 短期向け | ✅ P10 で採用済み、P13 で撤去済み。 |

採用判断理由:
- P10 の 1 sprint で String 排除まで持っていくには、`RuntimeIntrinsic` の追加が最小手順。
- P13 では runtime-catalog crate 抽出が完了したため、`RuntimeIntrinsic` を残す理由がなくなった。
- `program_builtins.rs` は `RuntimeFn` を直接返し、IR/backend/runtime catalog の variant drift を避ける。

→ ✅ **P13 で案 A へ移行済み。**

## 11. HIR / MIR / Wasm IR の責務明確化 🚧 P13: type + validator + lowering skeleton

docs/13 の HIR/MIR/Wasm IR 構想を具体化する。
`crates/ir/src/lowered/hir.rs`, `mir.rs`, `hir_validate.rs`, `mir_validate.rs`,
`hir_dump.rs`, `mir_dump.rs`, `hir_to_mir.rs` が境界を固定する。

### 11.1. HIR — JS の意味論を表す

```rust
// crates/ir/src/lowered/hir.rs
pub enum HirExpr {
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
// crates/ir/src/lowered/mir.rs
pub enum MirExpr {
    RawConst(RawValue),
    Local(MirLocal),
    CallRuntime { func: RuntimeFn, args: Vec<MirExpr> },
    WasmPrimitive { op: WasmPrimOp, args: Vec<MirExpr> },
}
```

ここでは `RuntimeFn` が出てよい。
HIR→MIR の lowering pass は `crates/ir/src/lowered/hir_to_mir.rs` にあり、
現時点では LoweredExpr を完全置換する migration path の足場として扱う。

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

## 12. Backend を符号化器にする 🚧 P13: backend-core crate 抽出済み

docs にも raw WAT 直書き禁止方針はあるが、WAT 文字列生成は壊れやすい。
括弧・stack discipline・indent・型・call signature を文字列で管理するから。

短期: `WatWriter` と `backend-core` の typed Wasm IR を強化する。

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

WAT は debug dump に下げる。`crates/backend-core/src/wasm_ir.rs` はこの方向の
typed module model であり、`backend-wasm` は concrete WAT/binary backend として扱う。

## 13. Diagnostic を frontend から分離する ✅ P13: source/diagnostic crate 分離済み

現在は `Diagnostic`, `DiagCode`, `Span` が frontend にあり、
backend と IR が frontend に依存していた。 → **P13 で crate 境界まで整理済み。**

移動先:

- `crates/source/src/lib.rs` — `Span`, source identity
- `crates/diagnostic/src/lib.rs` — `Diagnostic`, `DiagCode`

- ✅ `Diagnostic` struct — diagnostic crate に移動, frontend は互換 re-export
- ✅ `DiagCode` enum — diagnostic crate に移動, frontend は互換 re-export
- ✅ `Span` struct — source crate に移動, frontend は互換 re-export
- ✅ `crates/shared/src/lib.rs` — `pub mod diagnostic;`, `pub use diagnostic::{...}`
- ✅ ir/backend/compiler crate — shared/source/diagnostic-oriented import に移行中
- ✅ frontend — 後方互換のため再エクスポート維持

将来の拡張 (オプション):
- `DiagnosticOrigin` の導入 (Source(Span) / Generated(&str) / Internal)
- `Option<Span>` を減らす

分離先 (長期構想):

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

## 15. Architecture fitness functions 🚧 P13: checks 拡張済み

docs に原則を書くより CI で破らせない方が強い。

### 追加すべき check (状態)

```text
1. backend-wasm must not depend on frontend                      ✅ P13: dependency 削除 + check 監視
2. ir must not depend on frontend except temporary allowlist     ✅ P9: [dev-dependencies] のみ
3. no include! in crates/ir/src/lowered.rs                       ✅ P9: 0 include!
4. no RuntimeCall { runtime_fn: String }                         ✅ P10/P13: `RuntimeFn` typed path
5. no use super::* outside tests                                 ✅ P9: 既存 (preexisting)
6. no function > 200 lines                                       🚧 P11: warning + allowlist 運用
7. no file > staged threshold                                    🚧 P11: 2000/1500 staged checks
8. no new Diagnostic { span: None } for source diagnostics       ✅ P10: check_diagnostic_span_none (WARN)
9. no backend module imports Stmt/Expr from frontend             ✅ P10: check_backend_frontend_import でカバー
10. no raw "$runtime_symbol" string outside runtime catalog      ✅ P10/P13: runtime-catalog 正本化
11. no wat.push_str in new runtime helper files                  ✅ 既存 check (WARN only)
12. RuntimeFn::all contains every RuntimeFn variant              🚧 runtime-catalog registry check
13. RuntimeFn::emission_order contains every emitted function    🚧 runtime-catalog registry check
14. every RuntimeFn has RuntimeSpec                              🚧 runtime-catalog registry check
15. every RuntimeFn with imports has capability marker           🚧 HostImport/Capability catalog contract
16. every host import appears in manifest tests                  🚧 manifest snapshot tests
17. every LoweredExpr variant is covered by validate_lowered     ✅ P10: check_lowered_expr_validate_coverage
18. every HIR/MIR variant is covered by dump/snapshot printer    🚧 HIR/MIR dump + validate modules
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

## 18. Object Semantics Kernel と Completion Records 🚧 P10: stubs 追加

`docs/21-object-semantics-kernel.md` と `docs/22-completion-records.md` の方向性は良い。
P10 で `crates/ir/src/lowered/object_kernel.rs` と `crates/ir/src/lowered/completion.rs` に type stub を作成。

- ✅ `object_kernel.rs` — `OrdinaryGet`, `OrdinarySet`, `OrdinaryHasProperty`, `OrdinaryDelete` の stub
- ✅ `completion.rs` — `CompletionRecord`, `CompletionKind` enum の型定義
- ❌ 実際の lowering pass への統合は未着手

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
→ RuntimeIntrinsic enum を追加 (P10)
→ 一部だけ RuntimeIntrinsic に移す
→ String path を deprecated にする
→ architecture check で新規 String path 禁止
→ 残りを移す
→ String path 削除
→ RuntimeFn を runtime-catalog crate に移す
→ RuntimeIntrinsic を削除して IR から RuntimeFn を直接参照する (P13)
```

### 20.2. Strangler fig

巨大ファイルを一気に割らず、機能単位で周辺から置き換える:

```
resolver_expr.rs の lower_expr 巨大 match は残す
→ array literal branch だけ array/literal.rs へ移す    ✅ P9: resolver/array.rs
→ class branch だけ class/lower.rs へ移す              ✅ P9: resolver/class.rs
→ call branch だけ call/lower.rs へ移す                ✅ P9: resolver/call.rs
→ 最後に lower_expr は dispatcher だけにする           ✅ P9: 1122 lines
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
  Diagnostic moved to diagnostic crate      ✅ P13
  Span moved to source crate                ✅ P13
  DiagCode moved to diagnostic crate        ✅ P13
  frontend re-exports for compatibility     ✅ P13
  ir imports source/diagnostic contracts    ✅ P13
  backend imports source/diagnostic contracts ✅ P13
  cargo dependency removed                  ✅ P13 (backend-wasm → frontend)
  architecture check added                  ✅ P13
```

## 21. 直近でやるべき 10 項目（優先度順）— P13 実績反映

1. **`Span`, `DiagCode`, `Diagnostic` を frontend から外す** ✅ **P13 → done**
   `crates/source` と `crates/diagnostic` に分離。frontend は互換 re-export を維持。

2. **`Validated<LoweredProgram>` を導入する** ✅ **P13 → done**
   backend API を `Validated` だけ受ける形にする
   → `Validated<T>` struct (`types.rs`) 実装済み。public backend emit API は `Validated<LoweredProgram>`。

3. **`RuntimeCall { runtime_fn: String }` を型に置き換える** ✅ **P13 → done**
   `RuntimeIntrinsic` 中間層を経て、現在は `runtime-catalog::RuntimeFn` を直接参照。

4. **`runtime_fn_impl.rs` を domain 分割し runtime-catalog に抽出する** ✅ **P13 → done**
   `crates/runtime-catalog` が RuntimeFn / RuntimeSpec / HostImport / Capability / RuntimeLinkPlan の正本。

5. **`runtime_builder.rs` を domain dispatch にする** ✅ **P10 → done**
   `RuntimeDomain` enum による domain dispatch に置換。55 行。

6. **`compiler/src/lib.rs` から pipeline stage を切り出す** ✅ **P13 → done**
   `pipeline.rs`, `io/`, `stages/parse.rs`, `module_graph.rs`, `static_imports.rs`,
   `semantic_validate.rs`, `lowered_validate.rs`, `runtime_gate.rs` まで分割。

7. **`Resolver` の context を分解する** 🚧 **P10 → partial**
   `LoweringCtx`, `SymbolEnv`, `ClassEnv`, `StaticFacts` struct (`ctx.rs`, `symbols.rs`, `classes.rs`, `facts.rs`) 作成済み。
   `resolver/expr/` と `resolver/call/` の directory 分割済み。Resolver struct の field 群の完全移行は継続。

8. **`lower_expr` の branch を domain module に移す** ✅ **P13 → done (継続縮小対象あり)**
   `resolver/array.rs`, `resolver/call.rs`, `resolver/class.rs`, `resolver/object.rs` 作成済み。
   P13 では `resolver/call/` と `resolver/expr/` にさらに分割。

9. **`include!` を real module に置き換える** ✅ **P9 → done**
   `crates/ir/src/lowered.rs` の include! 全削除。real module 宣言に置換。

10. **architecture checks を強化する** 🚧 **P13 → warnings/checks 拡張**
    新規: `check_no_new_string_runtime_call`, `check_backend_frontend_import`,
    `check_runtimefn_spec_gap`, `check_rust_file_length(2000)` / `check_rust_file_length_1500(1500)`,
    `check_diagnostic_span_none`, `check_raw_runtime_symbol_outside_catalog`,
    `check_lowered_expr_validate_coverage`。
    継続: warning allowlist の縮小、host import / manifest checks、HIR/MIR coverage。

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
- `crates/source/`: source/span 定義
- `crates/diagnostic/`: diagnostic 定義
- `crates/runtime-catalog/`: RuntimeFn / RuntimeSpec / HostImport / Capability / RuntimeLinkPlan
- `crates/backend-core/`: typed Wasm IR / writer traits
