# eval / `Function` constructor 完全実装計画

Last audited: 2026-05-17 (`archive(14).zip`)
Previous plan: `plans/eval-new-function-implementation-plan.md`
Primary tracking candidate: `issues/I-20260517-WE8P5A.md`
Related tracking: `issues/done/I-20260513-HD4K3Q.md`, `issues/done/I-20260513-B49ZZE.md`, `issues/I-20260513-WBEJBE.md`, `issues/I-20260515-7N7MWQ.md`

## 1. 目的

`eval` と `Function` / `new Function` は、ts2wasm が JavaScript engine wrapper ではなく compiler として成立するかを決める難所である。この計画の目的は、現在進んだ parser-level / resolver-level の対応を土台にしつつ、次の最終形へ到達する実装順序を固定することにある。

1. **compile-time に source が確定する dynamic-code construct は wasm-native AOT lane へ入れる。**
   - static string direct `eval("...")`、static string indirect eval、literal-only `Function("...", "body")` / `new Function(...)` は、可能な限り parse / resolve / lower / backend pipeline に再投入する。
   - static lane は `eval` / `Function` そのものを理由に Node host import を出してはならない。
2. **runtime でしか source が分からない construct は capability-gated host lane へ入れる。**
   - dynamic indirect eval、dynamic `Function` constructor、dynamic direct eval は段階的に `host.eval.*` / `host.function.*` import で扱う。
   - host lane は manifest 上で `standalone: false`、`node_host.imports`、`capability_reasons` が完全一致している場合だけ emit できる。
3. **direct eval の caller-scope mutation は最後に実装する。**
   - local read/write-back、`var` / function declaration landing zone、strict eval lexical environment、TDZ、closure capture が絡むため、不完全な host eval で見かけ上 pass させない。

禁止事項は変わらない。original source 全体を Node.js `eval` / `Function` / `vm` に渡す wrapper 実装は禁止する。host shim が扱うのは、ユーザーコードが runtime に生成した source value に限定する。

## 2. 現在の進展と再評価

`archive(14).zip` では、前回計画時より eval 周辺が明確に進んでいる。ただし進展はまだ **3 つの異なる path に分散**しており、このまま拡張すると semantics の取りこぼしが起きる。

### 2.1 進んだ点

| 領域 | 現在の実装 | 重要度 |
|---|---|---|
| resolver/compiler expression-level literal eval | resolver が unshadowed `eval` を `ResolvedExpr::Eval` に分類し、`crates/compiler/src/stages/eval_expand.rs` が `let r = eval("1 + 2")` や `eval("1; 2;")` を compile-time に展開する | completion value の足場が入った |
| parser statement-level direct eval | `crates/frontend/src/parser/statements_general.rs::direct_eval_literal_statements` が `eval('x = "after";')` を caller statement 群へ展開する | caller local mutation の static slice が動く |
| Annex B block function slice | `static_block_function_eval_expansion` と `crates/ir/src/lowered/program_direct_eval.rs` が supported direct-eval block function fixtures を通す | eval-code function hoist の難所に着手済み |
| indirect eval parser rejection | `globalThis.eval("x")`、`globalThis["eval"]("x")`、`(0, eval)("x")` は parser reject ではなく後段へ流れる | Phase 4/5 の土台 |
| optional eval classification | `eval?.(src)` は parser shape を保持し、unshadowed optional eval を indirect eval として既存 host/static lane へ流す | optional-call nullish/shadowing edge の拡張 |
| resolved eval IR | `ResolvedExpr::Eval { kind, source, caller_is_strict, span }`、`EvalKind::{Direct, Indirect}`、`EvalSource::{StaticLiteral, Runtime}` が存在する | 統一 IR への入口 |
| compiler eval expansion stage | `crates/compiler/src/stages/eval_expand.rs` が static direct / indirect `ResolvedExpr::Eval` を parse / resolve / builtin-resolve して completion expression へ置換し、nested function/class bodies と parameter defaults も再帰的に処理する | parser rewrite から IR rewrite へ移行する素材 |
| literal `Function` constructor | resolver が unshadowed `Function(...)` / `new Function(...)` を internal constructor markers に分類し、compiler eval-expand stage が literal-only args を synthetic `FunctionExpr` に変換する | static `Function` AOT lane の初期 slice |
| build/server dynamic-code stage parity | `pipeline.rs` と `server.rs` はどちらも `expand_static_eval_fragments` を通し、server lowering focused test が host eval runtime call の残存を検出する | batch/server path でも static AOT lane を維持 |
| runtime catalog symbols | `RuntimeFn::EvalDirectHost` / `EvalIndirectHost` がある | host lane の名前だけはある |

### 2.2 まだ危険な点

| Gap | 現状 | 必要な修正 |
|---|---|---|
| eval path が 3 つある | parser expression rewrite、parser statement rewrite、compiler `ResolvedExpr::Eval` rewrite が併存 | canonical path を `DynamicCodePlan` / `EvalFragment` に一本化する |
| parser が binding-sensitive 判断をしている | statement-level direct eval expansion still uses the `possible_eval_shadowing` token count heuristic. `Function` constructor classification has moved to resolver lexical binding checks | remaining eval intrinsic / shadowed decisions should move fully to resolver facts |
| `EvalKind` が lowering で無視される | `ResolvedExpr::Eval` は direct/indirect に関係なく `RuntimeFn::EvalDirectHost` へ lower される | direct / indirect / host-global-only / full-direct を別 lowering にする |
| host runtime fn が host import ではない | `EvalDirectHost` / `EvalIndirectHost` は `NO_IMPORTS` / `NO_CAPS` で、string source は runtime WAT で `unreachable` | runtime catalog に real `HostImport` / `Capability` を追加し、stub trap を事前 diagnostic に置換する |
| compiler server path parity の regression 化 | `pipeline.rs` と `server.rs` はどちらも `expand_static_eval_fragments` を呼ぶ | parity test を維持し、今後の `DynamicCodePlan` 移行時にも両 path で同じ stage を通す |
| `caller_is_strict` が未接続 | builtin resolver で `false` 固定 | parser/resolver から strict context を伝搬する |
| static eval declaration completion が不安定 | `compiler/src/stages/eval_expand.rs::extract_completion_value` は statement を expression に潰すだけで、declaration environment を caller へ接続しない | eval-code statement lowering と completion slot を導入する |
| `Function` constructor grammar が簡易 | parameter strings を `join(", ")` して synthetic function を parse するのみ | ECMAScript Function constructor の parameter/body parse rules、strict restrictions、SyntaxError timing を明示実装する |
| tests / fixtures / issues が古い | `function-constructor-unsupported` という fixture 名や comment が build-success と矛盾。`I-20260517-WE8P5A` は dropped のまま | Phase 0 で tracking と naming を更新する |

この計画では、既に入った parser-level 実装を無駄にしない。ただし、それを最終形とは見なさず、**短期的には regression guard、長期的には resolver/lowering 主導の canonical implementation へ移行**する。

## 3. 現在の support matrix

| Feature | 例 | 現在 | 完全実装の着地点 |
|---|---|---|---|
| static direct eval expression | `let x = eval("1 + 2")` | parser post-parse rewrite で一部対応 | `EvalFragment` + completion slot で対応 |
| static direct eval statement mutation | `eval('x = "after"')` | statement rewrite で一部対応 | caller-scope eval-code lowering で対応 |
| static direct eval block function | `eval('{ function f(){} }')` | Annex B supported slice あり | hoist plan / mutable binding env を validation 付きで対応 |
| direct eval with declarations | `eval('var x=1; x')` | expression completion と environment 接続が未完成 | eval-code environment + completion record |
| indirect eval static literal | `(0, eval)("1+2")` | resolver が direct/indirect を分類し、supported literal subset は AOT eval expansion で host import なし | global `EvalFragment` AOT |
| indirect eval dynamic | `(0, eval)(src)` | `host.eval.indirect` manifest / host-deny slice と primitive-return / string-keyed primitive object property node-shim 実行 regression は実装済み | `host.eval.indirect` capability |
| direct eval dynamic | `eval(src)` | `host.eval.direct` manifest / host-deny slice と primitive-return node-shim 実行 regression は実装済み。initialized env-cell descriptor 経由の primitive number caller-local / parameter write-back、未初期化 caller env binding の TDZ-unsafe host 実行拒否、plain object result と string-keyed primitive property bridge も focused node-shim guarded。declaration landing / lexical env / object-identity/nested/error bridge は未完 | env descriptor + mutation ledger + write-back |
| optional eval | `eval?.(src)` | unshadowed optional eval は indirect eval として resolver が分類し、dynamic source は `host.eval.indirect` Node shim 実行 regression 済み | optional-call nullish/shadowing edge の拡張 |
| `new eval` | `new eval("x")` | unsupported / TypeError 境界が未整理 | eval is not constructor の TypeError parity |
| literal `Function` | `Function("a", "return a")` | resolver/compiler synthetic `FunctionExpr` slice; nested function/class body traversal, zero-arg, caller-local non-capture, non-simple duplicate bound-name and strict-body duplicate/non-simple/eval/arguments parameter early errors, and direct `.name` / `.length` / `.prototype` metadata are guarded for static constructor locals | first-class static `FunctionConstructorPlan` / generated function object |
| literal `new Function` | `new Function("a", "return a")` | resolver/compiler synthetic `FunctionExpr` slice; zero-arg and call output are Node differential guarded | generated function object + metadata |
| dynamic `Function` | `new Function(body)` | `host.function.compile` manifest / host-deny slice と statically visible returned host function handle の `host.function.call` / `host.function.construct` manifest/lowering slice は実装済み。primitive/string/object-return call、discarded construct、string-keyed primitive object property reads、`.length` / `.name` / `.prototype` metadata reads の node-shim regression も実装済み | `host.function.compile` + host function handle |
| shadowed `eval` / `Function` | `let eval = f; eval("x")` | resolver が shadowed eval を ordinary call として保持し、parser は shadowing risk のある Function rewrite を避ける。shadowed `Function` ordinary-call fixture は Node differential guarded | ordinary user binding semantics |
| `$262.evalScript` | `$262.evalScript(src)` | runtime helper exists but dynamic eval body未実装 | harness/global eval lane として別分類 |

## 4. 最終設計

### 4.1 two-lane execution

| Lane | Source certainty | Scope semantics | Output | Capability |
|---|---|---|---|---|
| AOT direct eval | static string | caller lexical / variable env | wasm-native eval-code lowering | eval 自体では不要 |
| AOT indirect eval | static string | global env | wasm-native global eval-code lowering | eval 自体では不要 |
| AOT Function constructor | all params/body static strings | global env, no caller capture | generated function object | eval 自体では不要 |
| Dynamic indirect eval | runtime maybe-string | global env | Node host shim | `host.eval.indirect` |
| Dynamic Function constructor | runtime params/body | global env, no caller capture | host function handle | `host.function.compile` / call / construct |
| Dynamic direct eval | runtime maybe-string | caller env with read/write-back | Node host shim + env descriptor | `host.eval.direct` |

### 4.2 parser の責務

parser は以下だけを行う。

- syntactic shape を壊さず AST に残す。
- literal source text と span を保持する。
- optional chaining / comma / member / call / new の構文構造を失わない。
- nested eval source を parse する場合でも、その結果を最終 semantic として確定しない。

parser が行ってはいけないこと:

- `eval` や `Function` が user binding で shadowed されていないと仮定すること。
- caller scope mutation の landing zone を決めること。
- Function constructor が intrinsic であると syntactic name だけで決めること。

現行 parser rewrite は一時的な compatibility shim として残してよいが、新しい coverage は resolver/lowering path に寄せる。

### 4.3 resolver の責務

resolver は lexical binding table を使って intrinsic dynamic-code construct を分類する。

```rust
pub enum DynamicCodeKind {
    DirectEval,
    IndirectEval,
    FunctionConstructor { called_with_new: bool },
    EvalScriptHostHook,
}

pub enum DynamicCodeSource<T> {
    StaticString { value: String, span: Span },
    RuntimeExpr(Box<T>),
    NonStringStatic(Box<T>),
}

pub enum EvalScopeMode {
    CallerScope { scope_id: ScopeId, strict_caller: bool },
    GlobalScope,
}

pub struct EvalFragmentPlan {
    pub kind: DynamicCodeKind,
    pub source: DynamicCodeSource<ResolvedExpr>,
    pub scope_mode: EvalScopeMode,
    pub source_span: Span,
    pub call_span: Span,
}

pub struct FunctionConstructorPlan {
    pub called_with_new: bool,
    pub params_and_body: Vec<DynamicCodeSource<ResolvedExpr>>,
    pub static_parse: Option<StaticFunctionParsePlan>,
    pub call_span: Span,
}
```

`eval` / `Function` が user binding で shadowed されている場合は、専用 IR にせず通常の identifier / call / new として扱う。

### 4.4 AOT eval-code lowering

static eval source は ordinary source file と同じ pipeline へ単純に投げるだけでは足りない。eval-code には caller scope / global scope への接続が必要である。

必要な構造:

- `EvalFragmentId`
  - source text、source span、strict flag、scope mode、parse goal を保持。
- `EvalCompletionSlot`
  - eval expression の戻り値を statement lowering から取り出す。
- `EvalDeclarationPlan`
  - `var` / function declaration の landing zone。
- `EvalLexicalEnvPlan`
  - strict eval / lexical declaration の閉じ込め。
- `EvalBlockFunctionPlan`
  - Annex B block-level function declaration の initial binding / mutable binding / var binding 接続。

static direct eval の completion は `docs/22-completion-records.md` の Completion Record model と接続する。

### 4.5 `Function` constructor lowering

literal-only `Function` / `new Function` は eval ではなく **global function compilation** として扱う。

必要な contract:

- caller lexical environment を capture しない。
- global environment で resolve する。
- parameter strings と body string は Function constructor grammar で parse する。
- duplicate parameter、default/rest/destructuring、strict mode directive、body-level `use strict` の early error を Node と揃える。
- resulting function object は callable かつ constructable。
- `.name === "anonymous"`、`.length`、own `.prototype` を metadata として持つ。
- static generated function は host import を出さない。

現行の `Expr::FunctionExpr` への parser rewrite は、短期的な build slice としては有用だが、metadata / constructability / non-capture validation を表現しづらい。最終的には `ResolvedExpr::FunctionConstructor` または `LoweredExpr::FunctionConstructorStatic` に移す。

### 4.6 dynamic host lane

runtime source の実行は pure wasm では実装できない。host lane は runtime catalog からのみ import を出す。

追加する host imports:

| HostImport | manifest import | 用途 |
|---|---|---|
| `EvalIndirect` | `host.eval.indirect` | dynamic indirect eval |
| `EvalDirectGlobalOnly` | `host.eval.directGlobalOnly` | local write-back 不要と証明できる direct eval subset |
| `EvalDirect` | `host.eval.direct` | env descriptor / write-back 付き full direct eval |
| `FunctionCompile` | `host.function.compile` | dynamic Function constructor compile |
| `FunctionCall` | `host.function.call` | host function handle call |
| `FunctionConstruct` | `host.function.construct` | host function handle construct |

対応する capability:

```rust
Capability::HostEvalIndirect
Capability::HostEvalDirectGlobalOnly
Capability::HostEvalDirect
Capability::HostFunctionCompile
Capability::HostFunctionCall
Capability::HostFunctionConstruct
```

現在の `RuntimeFn::EvalDirectHost` / `EvalIndirectHost` は名前に反して host import を持たず、string source で `unreachable` する。完全実装では次のどちらかに直す。

1. dynamic host lane を有効化し、RuntimeSpec に HostImport / Capability を付ける。
2. host lane 未有効時は lowering / runtime-gate で `UnsupportedEval` diagnostic を出し、WAT に `unreachable` stub を混入させない。

## 5. 実装 phase

### Phase 0: tracking / docs / tests の整合

目的: 既存の進展を誤って diagnostic-only / unsupported と扱わないようにする。

作業:

- `issues/I-20260517-WE8P5A.md` の dropped 状態を再検討し、runtime eval 全拒否ではなく「AOT lane + host lane 完全実装」issue に再編する。
- `issues/done/I-20260513-B49ZZE.md` は diagnostic acceptance だったことを明示し、literal Function constructor の現状とは分ける。
- fixture 名・comment の矛盾を直す。
  - `function-constructor-call-static.ts`
  - `new-function-constructor-static.ts`
  - `builtins-and-io/function-constructor.ts`
- name resolver tests で `dynamic Function constructor` rejection を期待する古い test を、static literal / dynamic runtime / shadowed の分類 test に置き換える。
- `current-state.md` と `docs/language-reference/javascript-features.md` を現状に合わせる。

Exit criteria:

- eval / Function の support matrix が docs、fixtures、tests、issues で一致する。
- 「Function constructor は unsupported」という古い表現が、literal-only AOT slice と dynamic host lane の区別に置き換わる。

### Phase 1: current AOT literal eval を regression guard 化する

目的: 既に動く static direct eval slice を壊さず、canonical path へ移行できるようにする。

作業:

- parser tests を以下に分ける。
  - expression-only eval completion
  - statement mutation eval
  - block function eval
  - shadowed eval ordinary-call
  - indirect eval shape preservation
  - optional eval indirect classification
- `pipeline.rs` と `server.rs` の stage parity を直す。
  - `expand_static_eval_fragments` を両方で呼ぶ、または canonical resolver/lowering path へ移して両方から不要にする。
- `compiler/src/stages/eval_expand.rs` の責務を明確化する。
  - 一時 stage なら current limitations を test で固定する。
  - canonical stage にするなら caller scope context を渡す。
- static direct eval が host import を出さない manifest test を追加する。

Exit criteria:

- `eval("1; 2;")`、`let x = 1; let y = eval("x + 2")`、`eval('x = "after"')` の supported subset が Node differential で一致する。
- build path と server path で同じ結果になる。
- static eval を含む standalone artifact に `host.eval.*` import がない。

### Phase 2: resolver-owned `EvalFragment` へ移行

目的: parser rewrite 依存を減らし、declaration / completion / scope effect を扱える IR にする。

作業:

- `ResolvedExpr::Eval` を `EvalFragmentPlan` 付きに拡張する。
- unshadowed direct eval 判定を resolver に移す。
- `possible_eval_shadowing` heuristic による parser rejection を削る。
- direct eval source を caller scope context で name resolve する。
- eval fragment 内の declarations を caller env / eval lexical env に正しく接続する。
- eval expression result は `EvalCompletionSlot` 経由で戻す。
- strict caller / strict eval code を伝搬する。

追加 fixtures:

```ts
let x = 1;
console.log(eval("x + 2"));
```

```ts
function f() {
  let x = "before";
  let r = eval('x = "after"; x');
  console.log(r);
  return x;
}
console.log(f());
```

```ts
function f() {
  eval('var x = 1; function g(){ return x; }');
  return g();
}
console.log(f());
```

Exit criteria:

- static direct eval with expression completion and supported declarations is not implemented by parser-only statement splicing.
- shadowed `eval` is ordinary user call/new behavior, not intrinsic eval.
- unsupported eval-code syntax fails with `UnsupportedEval` / issue-linked diagnostic before backend trap.

### Phase 3: Annex B / strict mode direct eval completion

目的: direct eval の最も壊れやすい block function / strictness / binding interaction を正す。

作業:

- Annex B block-level function declarations in eval-code を `EvalBlockFunctionPlan` に移す。
- sloppy direct eval の function declaration landing zone を caller function env に接続する。
- strict eval code の lexical declarations を eval lexical env 内へ閉じ込める。
- `arguments` / `this` / `new.target` / function self-reference の read semantics を caller context と合わせる。
- eval-code abrupt completion (`throw`, `return` invalidity, break/continue invalidity) を Completion Record と接続する。

Exit criteria:

- 既存 fixtures `direct-eval-block-function*` が parser special-case ではなく eval lowering path で通る。
- strict/sloppy の代表 test262 direct eval cases が Node differential で一致する。

### Phase 4: static `Function` / `new Function` AOT lane

目的: literal-only Function constructor を generated wasm function object として完全化する。

作業:

- parser rewrite ではなく resolver で unshadowed intrinsic `Function` を分類する。
- `ResolvedExpr::FunctionConstructor` を追加する。
- eval-expand traversal は nested function/class bodies と parameter defaults の current regression を維持する。
- static args の parse rule を実装する。
  - 0 args: empty body。
  - 1 arg: body only。
  - 2+ args: all but last are parameter strings, last is body。
- parameter strings は FormalParameters parse goal として parse する。
- body は FunctionBody parse goal として parse する。
- non-simple duplicate bound names plus body-level `"use strict"` duplicate /
  non-simple / `eval` / `arguments` params の early error を test で固定する。compiler-stage focused tests cover the current synthetic FunctionExpr path.
- generated function は global scope で resolve し、caller capture list が空であることを validate する。
- function object metadata を作る。
  - `.name`
  - `.length`
  - `.prototype`
  - callable / constructable
- `new Function(...)` と `Function(...)` の結果 object identity / call behavior を揃える。

追加 fixtures:

```ts
let f = new Function("a", "b", "return a + b");
console.log(f(1, 2));
console.log(f.length);
console.log(f.name);
```

```ts
let f = Function();
let g = new Function();
console.log(f());
console.log(g());
```

```ts
let x = 1;
let f = Function("return typeof x");
console.log(f()); // caller local を capture しない
```

```ts
const Function = (x) => x;
console.log(Function("return 1")); // ordinary call
```

Exit criteria:

- literal-only `Function` / `new Function` は host import なしで Node differential pass。
- metadata fixtures が pass。
- shadowed `Function` は ordinary user binding として扱われる。

### Phase 5: static indirect eval AOT lane

目的: indirect eval の static literal source を global eval-code として wasm-native 実行する。

作業:

- `(0, eval)("...")`、`globalThis.eval("...")`、`globalThis["eval"]("...")` を resolver で `EvalKind::Indirect` に分類する。
- source static string の場合は global scope context で parse / resolve / lower する。
- caller local は参照しない。global lookup / ReferenceError path を使う。
- `eval?.("...")` は spec に従い direct eval ではなく optional call / indirect-like global eval として扱う。

追加 fixtures:

```ts
let x = 1;
globalThis.x = 10;
console.log((0, eval)("x"));
```

```ts
let y = "local";
console.log(globalThis.eval("typeof y"));
```

Exit criteria:

- static indirect eval は host import なし。
- caller local non-capture が Node differential で一致する。
- `EvalKind::Indirect` が lowering/backend で direct に潰れない。
- unshadowed optional eval は parser diagnostic ではなく indirect eval として分類される。

### Phase 6: dynamic indirect eval host lane

目的: runtime source の indirect eval を capability-gated Node host shim で実装する。

作業:

- `HostImport::EvalIndirect` を追加する。
- `Capability::HostEvalIndirect` を追加する。
- `RuntimeFn::EvalIndirectHost` spec を `imports: &[HostImport::EvalIndirect]`、`capability: &[Capability::HostEvalIndirect]` にする。
- backend import emission は `RuntimeLinkPlan` 経由に限定する。
- capability manifest に `host.eval.indirect` と reason を出す。
- Node shim は RawValue string decode / non-string return / JS exception bridge を持つ。
- host-deny では compile または runtime-gate で明確に fail する。

Manifest example:

```json
{
  "standalone": false,
  "node_host": {
    "required": true,
    "imports": ["host.eval.indirect"]
  },
  "capability_reasons": {
    "host.eval.indirect": ["dynamic indirect eval source"]
  }
}
```

Exit criteria:

- dynamic indirect eval node-shim target passes。
- same fixture under host-deny fails with explicit diagnostic。
- static indirect eval remains standalone。

進捗:

- `host.eval.indirect` manifest / host-deny checks are implemented, and a
  focused Node WebAssembly shim regression covers primitive-return dynamic
  indirect eval plus string-keyed primitive object property reads.

### Phase 7: dynamic `Function` / `new Function` host lane

目的: runtime-generated params/body を host で compile し、host function handle として扱う。

作業:

- `HostImport::FunctionCompile` / `FunctionCall` / `FunctionConstruct` を追加する。
- `Capability::HostFunctionCompile` / `HostFunctionCall` / `HostFunctionConstruct` を追加する。
- `RuntimeFn::FunctionCompileHost` などを runtime catalog に追加する。
- host function handle representation を決める。
  - wasm heap object wrapping host external id。
  - or RawValue external-host tag + table id。
- host function call / construct の bridge を追加する。
- `.length`, `.name`, `.prototype` の minimum property bridge を実装する。
- thrown SyntaxError / runtime error を caller へ伝搬する。

進捗:

- `host.function.compile`, `host.function.call`, and
  `host.function.construct` manifest / host-deny lowering slices are
  implemented for statically visible dynamic Function handles. A focused Node
  WebAssembly shim regression covers primitive/string/object-return calls,
  string-keyed primitive object property reads, discarded constructor calls, and
  returned constructed objects with string-keyed primitive property reads. The
  focused shim preserves stable identity when the same host object is returned
  repeatedly, refreshes existing string-keyed primitive properties on repeated
  returns, tracks string-keyed primitive shape changes beyond the first host
  record capacity, bridges nested JS arrays with length/index reads for
  primitive elements and later array growth, exposes `.length`, `.name`, and
  `.prototype` metadata, bridges function-valued object properties as
  metadata-bearing callable host handles including aliased property calls and
  receiver-bound method calls, and carries handles as wasm object cells backed
  by host-side handle maps instead of number values; remaining work is
  identity-stable growth for
  already-returned wasm references, error bridging, and a runtime-wide host
  external object contract beyond the focused shim.

Exit criteria:

- dynamic `new Function(body)` passes on node-shim target。
- literal-only `new Function` remains standalone。
- host-deny fails clearly。

### Phase 8: dynamic direct eval with env descriptor

目的: caller local read/write を必要とする dynamic direct eval を安全に実装する。

作業:

- `EvalEnvDescriptor` を lower する。
  - readable bindings。
  - writable bindings。
  - `var` / function declaration landing zone。
  - strict eval lexical declaration isolation。
  - module live binding / imported binding exclusion。
- host shim に env snapshot を渡す。
- host eval 後、mutation ledger を wasm env cells / locals へ write back する。
- direct eval を含む function の locals を必要に応じて env-cell 化する。
- TDZ、private name、module live binding、nested closure mutation と競合する binding は diagnostic にする。

Exit criteria:

- `eval(src)` が caller local read/write を Node differential で一致させる。
- write-back missing は validation で検出される。
- dynamic direct eval import は `host.eval.direct` として manifest に出る。

進捗:

- `host.eval.direct` manifest / host-deny checks are implemented, and a
  focused Node WebAssembly shim regression covers primitive-return dynamic
  direct eval. Lowering now passes an initialized env-cell descriptor to the
  host import, and the Node shim regression covers primitive number
  caller-local, parameter, and shadowed block-local write-back plus string
  result/write-back bridging, object result default stringification,
  string-keyed primitive object property reads, and strict lexical shadow
  isolation for local `let`. Lowering also rejects dynamic direct eval before
  not-yet-initialized caller env bindings so the current host lane cannot bypass
  TDZ semantics. Full declaration landing, full TDZ modeling, object identity,
  nested/non-primitive properties, and error bridge semantics remain open.

### Phase 9: `$262.evalScript` / test262 ramp / cleanup

目的: language eval と harness eval を分けて coverage を伸ばす。

作業:

- `$262.evalScript(source)` は direct eval ではなく test262 host hook として分類する。
- `scripts/lib/test262_harness.py` / `compiler/src/test262_preprocessor.rs` の `new Function` stripping を見直す。
- reference coverage の eval filter を以下に分類する。
  - static direct eval AOT。
  - static indirect eval AOT。
  - static Function constructor AOT。
  - dynamic indirect eval host。
  - dynamic Function constructor host。
  - dynamic direct eval env write-back。
  - realm / cross-realm / `$262.evalScript` harness gap。
- `UnsupportedEval` burn-down を artifact で確認する。

Exit criteria:

- eval / Function constructor による expected-fail は実装不能扱いではなく、realm/harness/security など明確な理由だけに残る。

## 6. File-by-file task map

| File / area | 変更方針 |
|---|---|
| `crates/frontend/src/parser/eval_expand.rs` | Removed for `Function` constructor expression rewriting; remaining parser-side dynamic-code compatibility is statement-level direct eval in `statements_general.rs`. |
| `crates/frontend/src/parser/statements_general.rs` | `direct_eval_literal_statements` の parser semantic rewrite を `EvalFragment` lowering へ移す。 |
| `crates/frontend/src/parser/expressions_main.rs` | `eval?.()` は optional-call shape を保持し、resolver で indirect eval classification へ渡す。 |
| `crates/resolve/src/name_resolver.rs` | intrinsic / shadowed eval・Function 判定、strict context、scope id を管理する。未使用 diagnostic helper を整理する。 |
| `crates/ir/src/builtin_resolved.rs` | `EvalFragmentPlan` / `FunctionConstructorPlan` を追加または既存 `Eval` を拡張する。 |
| `crates/ir/src/builtin_resolver.rs` | direct / indirect / Function constructor classification を resolver facts に基づいて行う。 |
| `crates/compiler/src/stages/eval_expand.rs` | canonical stage にする場合は caller/global scope context を受ける。そうでなければ削除する。 |
| `crates/compiler/src/pipeline.rs` / `server.rs` | eval expansion / dynamic-code lowering stage parity を保証する。 |
| `crates/ir/src/lowered/program_direct_eval.rs` | Annex B block function support を `EvalBlockFunctionPlan` として一般化する。 |
| `crates/ir/src/lowered/resolver/expr/mod.rs` | `EvalKind` を無視せず、static / host / unsupported を分岐する。 |
| `crates/runtime-catalog/src/host_import.rs` | eval / function host imports を追加する。 |
| `crates/runtime-catalog/src/capability.rs` | host eval/function capabilities を追加する。 |
| `crates/runtime-catalog/src/runtime_fn.rs` | runtime specs に imports / capabilities を接続する。 |
| `crates/backend-wasm/src/runtime/host/emit.rs` | string source `unreachable` stubs を廃止し、host import or diagnostic にする。 |
| `crates/backend-wasm/src/capability_manifest.rs` | `host.eval.*` / `host.function.*` reasons を manifest に出す。 |
| `fixtures/core-semantics/*eval*` | static/dynamic/direct/indirect/optional/shadowed を命名で分ける。 |
| `fixtures/builtins-and-io/function-constructor.ts` | comment と expected behavior を literal AOT slice に合わせる。 |
| `scripts/lib/test262_harness.py` / `compiler/src/test262_preprocessor.rs` | `new Function` stripping を phase ごとの分類に置き換える。 |

## 7. Acceptance gates

| Gate | 条件 |
|---|---|
| G0 tracking | docs / fixtures / issue status が current support matrix と一致する |
| G1 static direct eval | expression completion + caller local mutation + no host import |
| G2 direct eval declarations | supported `var` / function declaration landing zone + completion record |
| G3 Annex B block function | existing block-function fixtures pass through canonical eval lowering |
| G4 static Function constructor | literal-only `Function` / `new Function` Node differential + metadata + no host import |
| G5 static indirect eval | global-scope semantics + no host import |
| G6 dynamic indirect eval | `host.eval.indirect` exact manifest + primitive-return node-shim pass + host-deny fail |
| G7 dynamic Function constructor | `host.function.*` exact manifest + host function call/construct pass |
| G8 dynamic direct eval | env descriptor + mutation ledger + write-back validation |
| G9 coverage | `UnsupportedEval` / Function-constructor expected-fail count decreases with categorized artifact |

全 gate の共通条件:

- backend WAT に silent `unreachable` eval stub を混ぜない。
- parser-only rewrite による semantic pass を新規 acceptance としない。
- host import 追加時は RuntimeCatalog、RuntimeLinkPlan、CapabilityManifest、Node shim の名前を完全一致させる。
- standalone artifact に Node host eval import を混入させない。

## 8. 推奨 PR 分割

### PR 1: tracking and regression guard

- この計画を更新。
- current-state / language reference / fixture comments を現状に合わせる。
- stale `unsupported Function constructor` tests を分類し直す。
- static eval / literal Function current behavior の focused tests を追加する。

### PR 2: resolver-owned dynamic-code classification

- shadowed / unshadowed eval and Function を resolver で判定する。
- indirect eval shape を `EvalKind::Indirect` にする。
- parser heuristic rejection を縮小する。

### PR 3: static direct eval `EvalFragment`

- `EvalFragmentPlan`、completion slot、caller-scope resolution。
- parser rewrite 依存を段階的に削減。

### PR 4: direct eval declarations and Annex B

- var/function hoist plan。
- block function mutable env plan。
- strict eval lexical env。

### PR 5: static Function constructor AOT

- `FunctionConstructorPlan`。
- Function constructor grammar。
- generated function object metadata。

### PR 6: static indirect eval AOT

- global eval fragment。
- caller local non-capture tests。

### PR 7: dynamic indirect eval host catalog

- HostImport / Capability / RuntimeFn / manifest。
- Node shim。
- host-deny tests。

### PR 8: dynamic Function constructor host lane

- host function handle representation。
- compile / call / construct bridge。
- metadata bridge。

### PR 9+: dynamic direct eval env descriptor and test262 ramp

- env snapshot。
- mutation ledger。
- write-back。
- `$262.evalScript` / realm classification。

## 9. 直近の最小作業単位

次に着手すべき最小単位は **PR 1 + PR 2 の前半**である。

1. `plans/eval-new-function-implementation-plan.md` をこの版へ更新する。
2. `current-state.md` の eval paragraph を archive(14) の実装に合わせる。
3. stale fixture comments と stale test names を直す。
4. `server.rs` が `pipeline.rs` と同じ eval expansion stage を通るか確認し、差があるなら focused test を追加する。
5. `RuntimeFn::EvalDirectHost` / `EvalIndirectHost` の host import と capability reason を明示しつつ、static eval が host import を出さない regression test を残す。

これで「既に進んだ static lane」と「これから実装する dynamic host lane」の境界が明確になる。その後に resolver-owned `EvalFragment` へ移行する。

## 10. リスクと対策

| Risk | 影響 | 対策 |
|---|---|---|
| parser rewrite が shadowed eval / Function を誤展開する | user binding semantics が壊れる | intrinsic 判定を resolver へ移す |
| eval completion value が declaration env と切り離される | `eval('let x=1; x')` などが壊れる | statement lowering + completion slot で扱う |
| indirect eval が direct eval として lower される | caller local capture の spec 違反 | `EvalKind` を lowering/backend validation で必須分岐にする |
| runtime stub `unreachable` が semantic failure を隠す | backend trap が diagnostic を置き換える | dynamic eval 未実装時は lowering/runtime-gate diagnostic |
| Function constructor が caller scope を capture する | spec 違反 | global resolve + empty capture validation |
| host eval capability が広すぎる | sandbox / manifest の信用低下 | exact import names + host-deny + reason validation |
| direct dynamic eval write-back が漏れる | observable mutation loss | env descriptor validation と mutation ledger |
| test262 `$262.evalScript` と language eval を混同する | coverage 分析が濁る | harness hook を別 label に分類 |

## 11. 完了時の状態

完了時には以下が成立する。

- static direct eval は expression completion、caller local mutation、supported declarations、Annex B block functions を wasm-native に実行する。
- static indirect eval は global semantics で wasm-native に実行する。
- literal-only `Function` / `new Function` は generated wasm function object として実行し、metadata を持つ。
- dynamic indirect eval と dynamic `Function` constructor は exact capability manifest を伴う Node host lane で動く。
- dynamic direct eval は env descriptor / mutation ledger / write-back により caller scope mutation を検証可能に扱う。
- `UnsupportedEval` は「未実装だから大量に残る」状態ではなく、realm / harness / unsupported binding kind など明確な理由に分類される。
