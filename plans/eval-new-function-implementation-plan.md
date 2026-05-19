# eval / `Function` constructor 完全実装計画（archive15 反映版）

Last audited: 2026-05-18 (`archive(15).zip`)

Previous plan: `plans/eval-new-function-implementation-plan.md`

Primary tracker: `issues/I-20260517-WE8P5A.md`

Related trackers: `issues/done/I-20260513-HD4K3Q.md`, `issues/done/I-20260513-B49ZZE.md`, `issues/I-20260513-WBEJBE.md`, `issues/I-20260515-7N7MWQ.md`

Audit note: この版は `archive(15).zip` のファイル差分・実装ファイル・fixture/test 名の読み取りに基づく。audit 環境では Rust test suite を再実行していないため、本文中の「guarded」は該当 fixture / test / issue note が tree 内に存在するという意味で使い、ここで再実行済みという意味ではない。

## 1. 目的

`eval` と `Function` / `new Function` は、ts2wasm が JavaScript engine wrapper ではなく compiler として成立するかを決める要所である。`archive(15)` では、前回計画後に parser semantic rewrite の削除、resolver-owned plan の導入、static AOT expansion、dynamic host lane、direct eval env descriptor が大きく進んだ。したがって、この計画は「未実装の eval をどう始めるか」ではなく、**すでに入った focused slices を canonical implementation に統合し、完全実装へ収束させるための更新版**とする。

最終方針は変えない。

1. **compile-time に source が確定する dynamic-code construct は wasm-native AOT lane へ入れる。**
   - static string direct `eval("...")`、static string indirect eval、literal-only `Function("...", "body")` / `new Function(...)` は parse / resolve / builtin-resolve / lower / backend pipeline に再投入する。
   - static lane は `eval` / `Function` そのものを理由に Node host import を出してはならない。
2. **runtime でしか source が分からない construct は capability-gated host lane へ入れる。**
   - dynamic indirect eval、dynamic direct eval、dynamic `Function` constructor は `host.eval.*` / `host.function.*` import で扱う。
   - host lane は manifest 上で `standalone: false`、`node_host.required: true`、`node_host.imports`、`capability_reasons` が完全一致している場合だけ emit する。
3. **direct eval の caller-scope mutation は descriptor / ledger / validation で扱う。**
   - local read/write-back、`var` / function declaration landing、strict eval lexical env、TDZ、closure capture、host external object facts が絡むため、focused shim の成功を最終形と見なさない。
4. **wrapper 化は禁止する。**
   - original source 全体を Node.js `eval` / `Function` / `vm` に渡す実装は禁止する。
   - host shim が扱ってよいのは、ユーザーコードが runtime に生成した source value と、それを実行するために必要な監査済み env descriptor に限定する。

## 2. `archive(15)` で確認できる重要な進展

### 2.1 parser rewrite から resolver/compiler plan へ移行が進んだ

`crates/frontend/src/parser/eval_expand.rs` は存在しなくなり、parser-side の direct eval statement splice と `possible_eval_shadowing` heuristic は削除されている。`eval("...")` statement は AST call shape を保持し、name resolver / builtin resolver / compiler `expand_static_eval_fragments` に流れる。parser が binding-sensitive semantics を決める構造を避けられるため、shadowed `eval` / `Function` の ordinary binding semantics を壊しにくくなった。

現在の IR には次の入口がある。

```rust
ResolvedExpr::Eval { plan: EvalFragmentPlan }
ResolvedExpr::FunctionConstructor { kind, args, span }

pub struct EvalFragmentPlan {
    pub kind: EvalKind,
    pub source: EvalSource,
    pub caller_is_strict: bool,
    pub span: Span,
}

pub enum EvalKind { Direct, Indirect }
pub enum EvalSource { StaticLiteral(String), Runtime(Box<ResolvedExpr>) }
pub enum FunctionConstructorKind { Call, New }
```

ただし、これはまだ最終の semantic plan ではない。scope id、eval declarations、completion slot、lexical environment、host capability reason、TDZ policy、host external facts は別 path に分散している。次の主作業は、この `EvalFragmentPlan` / `FunctionConstructor` を「分類ノード」から「実行計画ノード」へ育てることになる。

### 2.2 static eval expansion が大幅に広がった

`crates/compiler/src/stages/eval_expand.rs` は、static direct / indirect eval と literal-only Function constructor を compile-time に展開する中心 stage になっている。`pipeline.rs`、`server.rs`、dump pipeline でも `expand_static_eval_fragments` が通るようになり、build / server / dump の stage parity は前進している。

static eval completion は `EvalCompletionStep` へ拡張されており、単純な「最後の expression だけを返す」段階を超えている。現在 tree で確認できる step は、少なくとも次を含む。

- `HoistVars`
- `HoistFunctions`
- `Value`
- `Empty`
- `VarLet`
- `FunctionDecl`
- `ClassDecl`
- `Block`
- `If`
- `While`
- `DoWhile`
- `For`
- `ForOf`
- `ForIn`
- `Switch`
- `TryCatch`
- `Labeled`
- `Throw`
- `Break`
- `Continue`
- `LexicalLet`
- `DestructureLet`

Fixture 名からも、`if` / `while` / `do while` / `for` / `for-of` / `for-in` / `switch` / `try` / labeled completion、destructuring completion、class declaration completion、declaration empty / previous completion preservation が追加されている。旧計画の「statement を expression に潰すだけで不安定」という評価は古い。今は **EvalCompletionStep による completion plan が始まっているが、まだ canonical eval-code environment とは言えない**、という評価に更新する。

### 2.3 static direct eval の caller-scope slice が拡張された

static direct eval は、caller local read、assignment side effect、`var` / function declaration landing、eval-defined function が eval-defined `var` を読む slice、strict caller での `var` locality、Annex B block function、class/private/`this`/`arguments`/`new.target` 関連 fixture まで広がっている。

一方で、現在の実装は compiler expansion、lowered direct-eval helpers、env-cell planning、fixture-specific guards が組み合わさった状態であり、完全な eval-code lexical environment model にはまだ届いていない。特に、lexical declarations、TDZ、runtime-created bindings の wasm-native 後続参照、abrupt completion の一般化は残る。

### 2.4 static indirect / optional eval の AOT lane が入った

`(0, eval)("...")`、`globalThis.eval("...")`、`globalThis["eval"]("...")` は parser reject ではなく resolver classification へ流れ、supported static literal subset は AOT eval expansion で host import なしになっている。`eval?.("...")` も unshadowed optional eval として indirect/global semantics 側に分類される slice が入っている。

注意点として、static indirect eval は caller binding collision を `globalThis` property access へ rewrite する補助を持つ。これは focused slice として有効だが、global lexical binding、global object binding、realm semantics を表現する canonical global eval environment ではない。完全実装では `EvalScopeMode::Global` と global env model に置き換える。

### 2.5 dynamic host lane が manifest / host-deny まで進んだ

runtime catalog / backend / manifest / tests に以下が入っている。

- `HostImport::EvalDirect` -> `host.eval.direct`
- `HostImport::EvalIndirect` -> `host.eval.indirect`
- `HostImport::FunctionCompile` -> `host.function.compile`
- `HostImport::FunctionCall` -> `host.function.call`
- `HostImport::FunctionCallMethod` -> `host.function.callMethod`
- `HostImport::FunctionConstruct` -> `host.function.construct`
- corresponding `Capability::{HostEvalDirect, HostEvalIndirect, HostFunctionCompile, HostFunctionCall, HostFunctionCallMethod, HostFunctionConstruct}`
- runtime specs / link plan / capability registry tests
- host-deny tests in `crates/cli/tests/m11_host_deny.rs`
- focused Node shim in `crates/cli/tests/node_shim_host.rs`

旧計画の「runtime catalog symbols の名前だけはある」「host import を追加する」は古い。今の残作業は、host import の存在ではなく、**host external object/function handle contract を runtime-wide に一般化し、focused shim 依存から脱すること**である。

### 2.6 dynamic direct eval env descriptor が入った

dynamic direct eval は、runtime source を `host.eval.direct(source, env)` に渡すだけではなく、lowering が initialized env-cell descriptor を作るようになっている。descriptor には caller strict metadata も含まれ、Node shim 側では以下の focused semantics が入っている。

- caller local / parameter / catch binding write-back
- block shadow write-back
- string result / write-back bridge
- plain object result、object identity
- nested object / array primitive property bridge
- string-keyed primitive property bridge
- thrown Error-like object bridge into wasm `try/catch`
- class/object method receiver
- class method `arguments`
- arrow lexical `this` / `arguments` / outer lexical write-back
- existing caller `var` landing zone への `var` / function declaration write-back
- host lane 内で新規に作られた `var` / function declaration の later eval persistence
- destructuring `var` binding names extraction for later direct eval
- strict caller metadata propagation
- strict caller `delete Identifier` SyntaxError bridge
- strict caller restricted binding guard for `arguments` / `eval` in `var` / `let` / `const` / function / async function / destructuring forms
- env-cell result assignment が host external object / host function handle facts を保持し、function-valued properties を `host.function.call` / `host.function.callMethod` へ流せる slice

ただし、これはまだ full direct eval ではない。現行 lowering は not-yet-initialized caller env binding を見つけた場合に `UnsupportedEval` diagnostic を出す。これは正しい安全策だが、full TDZ modeling の未完了を示す。また、host lane 内で作られた binding は後続 dynamic eval には見えるが、通常 wasm code からの direct access はまだ一般化されていない。

### 2.7 literal-only Function constructor の AOT slice が拡張された

`ResolvedExpr::FunctionConstructor` が導入され、static literal args は `eval_expand` 内で synthetic `FunctionExpr` に展開される。fixture / issue note から、以下の focused coverage がある。

- zero args
- call and construct behavior
- caller local non-capture
- parameter defaults
- rest parameters, including `.length === 0`
- destructuring parameters
- non-simple duplicate bound-name early error
- strict body duplicate / non-simple / `eval` / `arguments` parameter early errors
- `arguments` object reads
- `.name`, `.length`, `.prototype` metadata
- constructor object-return behavior vs primitive return fallback
- sloppy / strict `this` binding
- construct-call `new.target` typeof / `.name` / `.length`
- shadowed `Function` ordinary-call behavior

残りは、synthetic `FunctionExpr` から first-class `FunctionConstructorPlan` へ移し、ECMAScript Function constructor の parameter/body parse goals、SyntaxError timing、global realm / global environment semantics、metadata/toString/prototype identity を体系化すること。

### 2.8 `$262.evalScript` は indirect eval host lane に接続済み

`$262.evalScript(source)` は `Dollar262Eval` に lower され、`host.eval.indirect` lane に委譲されている。focused Node-shim / host-deny regression が入っている。残りは realm / cross-realm / test262 harness 分類であり、language direct eval と混同しないことが重要である。

## 3. 現在の support matrix

| Feature | 例 | `archive(15)` 現状 | 完全実装の着地点 |
|---|---|---|---|
| static direct eval expression | `let y = eval("x + 2")` | `ResolvedExpr::Eval` -> `expand_static_eval_fragments` -> `EvalCompletionStep`。caller binding read と expression completion fixture あり | `EvalFragmentPlan` + explicit caller scope + completion slot |
| static direct eval side effect | `eval('x = "after"; x')` | `Sequence` / `EvalCompletionStep` 経由で focused caller-local write/read が guarded | eval-code statement lowering と mutation validation |
| static direct eval declarations | `eval('var x=1; function g(){return x}')` | `HoistVars` / `HoistFunctions`、eval-created var/function landing slice あり | `EvalDeclarationPlan` で var/function/class/lexical/destructuring を統一 |
| static direct eval lexical/class completion | `eval('class C{}; typeof C')` | class / lexical / destructuring completion fixtures あり | eval lexical env、TDZ、class binding lifecycle を canonical 化 |
| Annex B block function in eval | `eval('{ function f(){} }')` | parser splice なしで `EvalCompletionStep` + direct-eval helpers により focused fixtures が残る | `EvalBlockFunctionPlan` と Annex B validation |
| static direct eval abrupt completion | `eval('throw e')`, invalid `return` | throw/break/continue step は存在。`return` は static eval source validation で reject | Completion Record と invalid abrupt completion diagnostics の体系化 |
| static indirect eval | `(0, eval)("x")` | supported static literal subset は AOT で host import なし。caller collision rewrite helper あり | global eval environment plan |
| optional eval | `eval?.(src)` | unshadowed optional eval は indirect/global lane へ流れる slice あり | optional nullish / shadowed / call-reference semantics の拡張 |
| dynamic indirect eval | `(0, eval)(src)` | `host.eval.indirect` manifest / host-deny / focused Node shim | audited host lane + broader object/error/realm coverage |
| dynamic direct eval | `eval(src)` | `host.eval.direct` env descriptor、strict metadata、write-back、declaration persistence、object/function handle bridge の focused slice | full env descriptor + mutation ledger + TDZ + wasm-visible runtime bindings |
| `new eval` | `new eval("x")` | catchable TypeError parity fixture あり | broader non-constructor builtin audit |
| literal `Function` | `Function("a", "return a")` | `ResolvedExpr::FunctionConstructor` -> synthetic `FunctionExpr` AOT。params/default/rest/destructuring/metadata/this/new.target/arguments fixtures あり | first-class static `FunctionConstructorPlan` と generated function object |
| literal `new Function` | `new Function("return 1")` | static construct call and metadata fixtures あり | constructable generated function object with prototype identity |
| dynamic `Function` compile | `Function(body)` where `body` runtime | `host.function.compile` manifest / host-deny / focused Node shim | host function handle contract |
| dynamic host function call | `f(1)`, `new f()` | `host.function.call` / `host.function.construct` slice あり | general callable/constructable host external handle model |
| dynamic host method call | `obj.m()` where `m` is host function | `host.function.callMethod` import/capability/lowering slice あり | receiver-preserving method bridge across host external objects |
| host external object bridge | dynamic eval / Function returns object/array/function | focused Node shim preserves identity, growth, nested primitive leaves, function-valued props | runtime-wide `HostExternalObject` / `HostExternalFunction` contract |
| shadowed `eval` / `Function` | `let eval = f; eval("x")` | resolver keeps ordinary call for shadowed cases; fixture coverage exists | ordinary binding semantics in all call/new/optional/member forms |
| `$262.evalScript` | `$262.evalScript(src)` | `Dollar262Eval` delegates to `host.eval.indirect`; manifest/host-deny guarded | separate harness/realm classification and test262 ramp |

## 4. 更新後の設計原則

### 4.1 parser の責務を固定する

parser は syntactic shape を残すだけにする。

- `eval`, `Function`, `new Function`, optional call, comma call, member call の AST shape を壊さない。
- literal source text と span を保持する。
- direct eval / indirect eval / Function intrinsic の binding-sensitive 判定をしない。
- caller scope mutation、declaration landing、global env rewrite を決めない。

`archive(15)` で parser splice が削除されたため、この原則はほぼ満たされている。今後 parser に戻してよいのは構文 parse と source span preservation だけで、semantic shortcut は戻さない。

### 4.2 resolver / builtin resolver の責務を強める

resolver は lexical binding facts を使って dynamic-code construct を分類する。既存の `EvalFragmentPlan` は以下へ拡張する。

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
    GlobalScope { realm_id: RealmId },
}

pub struct EvalFragmentPlan {
    pub kind: EvalKind,
    pub source: EvalSource,
    pub scope_mode: EvalScopeMode,
    pub caller_is_strict: bool,
    pub eval_source_is_strict: Option<bool>,
    pub declaration_plan: Option<EvalDeclarationPlan>,
    pub completion_plan: Option<EvalCompletionPlan>,
    pub host_plan: Option<EvalHostPlan>,
    pub span: Span,
}

pub struct FunctionConstructorPlan {
    pub kind: FunctionConstructorKind,
    pub args: Vec<DynamicCodeSource<ResolvedExpr>>,
    pub static_parse: Option<StaticFunctionConstructorParse>,
    pub host_plan: Option<FunctionHostPlan>,
    pub span: Span,
}
```

既存 `EvalFragmentPlan { kind, source, caller_is_strict, span }` は良い入口である。次は、scope/declaration/completion/host policy を別 stage の暗黙 state ではなく plan に載せる。

### 4.3 static AOT lane は eval-code environment を持つ

static eval source を ordinary source file と同じ pipeline に投げるだけでは不十分である。eval-code には caller/global environment との接続が必要になる。

必要な plan:

- `EvalFragmentId`: source text、source span、strict flag、scope mode、parse goal。
- `EvalCompletionSlot`: `eval(...)` expression の戻り値を statement lowering から取り出す。
- `EvalDeclarationPlan`: `var` / function / class / lexical / destructuring declaration の landing zone。
- `EvalLexicalEnvPlan`: strict eval code と lexical declarations の閉じ込め。
- `EvalBlockFunctionPlan`: Annex B block-level function declaration の initial binding / mutable binding / var binding 接続。
- `EvalAbruptCompletionPlan`: `throw`、invalid `return`、invalid `break` / `continue`、caught exception を Completion Record と接続。

`EvalCompletionStep` はこの最終 plan の素材として残す。短期的には current stage を維持しつつ、step lowering が canonical semantics に近づくようにする。

### 4.4 static indirect eval は global environment を明示する

static indirect eval の caller collision rewrite は regression を増やすためのよい interim 実装だが、完全形ではない。

完全形では次を分ける。

- global object property binding
- global lexical binding
- module top-level lexical binding
- realm-specific global object
- `typeof` / ReferenceError / assignment semantics

`EvalKind::Indirect` は backend まで失われてはいけない。direct eval と同じ caller env descriptor に流れることを validation error とする。

### 4.5 Function constructor は synthetic function から first-class plan へ移す

現行の literal-only Function constructor は `function anonymous(params) { body }` source を合成し、`FunctionExprOrigin::FunctionConstructor` の `ResolvedExpr::FunctionExpr` へ展開する。これは広い fixture を早く通すには有効だが、以下を表現しづらい。

- Function constructor 専用の FormalParameters / FunctionBody parse goal
- source text と `Function.prototype.toString()` の表現
- call/new 共通の function object identity
- `.prototype` identity と descriptor
- global realm / no caller capture の validation
- SyntaxError timing と diagnostic classification
- dynamic host lane への exact fallback reason

したがって最終形では `FunctionConstructorPlan` を作り、static AOT の generated function object と dynamic host handle を同じ semantic model で扱う。

### 4.6 dynamic direct eval は env descriptor v2 にする

現行 descriptor は focused slice として大きく進んでいる。次は `EvalEnvDescriptor` を formal data model にする。

必要な fields:

- caller strict flag
- eval call site id
- caller activation id or stable descriptor key
- readable bindings
- writable bindings
- uninitialized / TDZ bindings
- `var` / function declaration landing zone
- eval lexical environment allocation policy
- synthetic compatibility bindings: `this`, `arguments`, `new.target` where supported
- host external object/function fact retention policy
- module imported/live binding exclusion
- private name / super / class home object exclusion

host shim の戻り値は単なる result value ではなく、mutation ledger を含める。

```rust
pub struct EvalMutationLedger {
    pub result: RawValue,
    pub writes: Vec<EvalBindingWrite>,
    pub created_var_bindings: Vec<EvalCreatedBinding>,
    pub created_function_bindings: Vec<EvalCreatedBinding>,
    pub abrupt: Option<EvalAbruptCompletion>,
}
```

現行 Node shim は wrapper `Function(...)` を使って focused direct eval を実行する。これは runtime source value の host lane として許容できるが、restricted binding guard が scanner/heuristic に寄っているため、full implementation では parser-backed validation または host-side precise parse validation へ置き換える。

### 4.7 host external object/function handle を runtime-wide contract にする

dynamic eval / Function host lane が増えると、host object, array, function, Error-like object が wasm heap と host heap を跨ぐ。`archive(15)` では focused Node shim が identity, growth, nested primitive leaves, function-valued property calls, method receiver を扱っているが、これは test shim に強く依存する。

完全形では runtime ABI と lowering facts に次を持つ。

- `HostExternalObject` RawValue tag
- `HostExternalFunction` RawValue tag
- stable host handle table id
- object identity and aliasing rules
- property get/set/delete contract
- array length/index growth contract
- callable / constructable / method-call receiver contract
- exception object bridge contract
- manifest/capability derivation rules

`host.function.callMethod` はこの contract の一部として扱い、`host.function.call` の派生ではなく receiver-sensitive import として残す。

## 5. 更新後の implementation phases

### Phase 0: tracking / docs / naming consistency

Status: **mostly done, but keep as guard.**

`issues/I-20260517-WE8P5A.md` は `doing` に戻っており、dynamic eval / dynamic Function は「全体 unsupported」ではなく supported host-lane slices として再分類されている。`current-state.md` と language reference も partial implementation として更新されている。

残作業:

- `plans/eval-new-function-implementation-plan.md` をこの版へ更新する。
- docs の support matrix で `host.function.callMethod` と dynamic direct eval strict/restricted-binding slice を明記する。
- fixture 名に `unsupported` が残るものは、実際に unsupported boundary か historical name かを分ける。
- `direct-eval-return-unsupported.ts` と `direct-eval-block-function-shadowed-unsupported.ts` は expected diagnostic の理由をコメントで明確化する。
- 旧 `direct-eval-dynamic-tdz-conflict-unsupported.ts` 境界は `direct-eval-dynamic-tdz-conflict-node-shim.ts` へ移行済みで、dynamic direct eval host descriptor lane の catchable `ReferenceError` fixture として扱う。

Exit criteria:

- docs / fixtures / issue notes が `archive(15)` の support matrix と一致する。
- 「Function constructor は unsupported」「indirect eval は parser reject」という古い表現が残らない。

### Phase 1: parser semantic rewrite removal を regression guard 化

Status: **implemented in archive15; regression guard を維持する。**

完了していること:

- parser-side direct eval statement splicing は削除済み。
- `possible_eval_shadowing` heuristic は削除済み。
- statement position の `eval("...")` は AST call shape を保持する。
- build / server / dump path は `expand_static_eval_fragments` を通る。

残作業:

- parser tests に「eval call shape preservation」を explicit に残す。
- shadowed `eval` / `Function` が ordinary call/new/optional/member semantics に流れることを resolver tests で固定する。
- future contributor が parser rewrite を戻さないよう、`frontend/parser` の comment に禁止事項を置く。

Exit criteria:

- parser output だけでは intrinsic eval と user binding eval を区別しない。
- parser changes で static eval fixture が通るようにする PR は reject できる。

### Phase 2: `EvalFragmentPlan` を canonical eval execution plan に拡張

Status: **partially implemented; next immediate target.**

現状:

- `ResolvedExpr::Eval { plan }` は kind/source/strict/span を持つ。
- static expansion は `eval_expand.rs` が担う。
- dynamic lowering は `ResolvedExpr::Eval` を runtime call へ lower する。
- declaration/completion/TDZ/env descriptor は plan 外に分散している。

作業:

- `EvalFragmentPlan` に `scope_mode`, `declaration_plan`, `completion_plan`, `host_policy` を追加する。
- `EvalCompletionStep` の生成を plan-owned に移す。
- `eval_expand.rs` を「temporary compiler rewrite」ではなく「AOT eval fragment expansion stage」として名前・責務を整理する。
- direct eval source resolution に caller scope id を渡し、visible binding vector だけに頼らない。
- indirect eval source resolution に global scope id / realm id を渡し、caller collision rewrite を段階的に削る。
- unsupported eval source syntax は backend trap ではなく issue-linked diagnostic にする。

Exit criteria:

- static direct eval expression, statement, declaration landing, and completion are represented by `EvalFragmentPlan` / `EvalCompletionPlan`, not hidden in parser or ad-hoc visible binding lists.
- lowering can reject malformed direct/indirect/static/dynamic combinations by inspecting the plan.

### Phase 3: static direct eval declaration / lexical environment 完全化

Status: **focused slices implemented; canonical env still open.**

現状の強み:

- `HoistVars` / `HoistFunctions` がある。
- `VarLet`, `FunctionDecl`, `ClassDecl`, `LexicalLet`, `DestructureLet` がある。
- `if` / loop / switch / try / labeled completion step がある。
- strict caller `var` locality slice がある。

残作業:

- sloppy direct eval の `var` / function declaration landing zone を caller variable environment として formalize する。
- strict eval code と strict caller eval code の lexical declarations を eval lexical environment に閉じ込める。
- lexical declarations の TDZ と duplicate declaration checks を eval code と caller env の両方で validation する。
- eval-created `var` / function を後続 normal wasm code から読む場合の binding materialization を一般化する。
- `class` declaration completion と class binding lifecycle を `EvalDeclarationPlan` へ接続する。
- invalid `return`, invalid `break` / `continue`, uncaught `throw`, caught exception を Completion Record と統合する。
- Annex B block function の block-scoped initial binding と function-scope mutable binding を plan に移す。

Exit criteria:

- static direct eval with `var` / function / class / lexical / destructuring declarations が eval-code environment model で説明できる。
- fixture が「compiler rewrite の偶然」ではなく、declaration / completion / environment plan によって通る。
- static direct eval は host-deny standalone のまま。

### Phase 4: static indirect / optional eval AOT lane の global env 化

Status: **supported literal subset implemented; global env model still open.**

作業:

- `EvalKind::Indirect` を `EvalScopeMode::Global` と結びつける。
- `(0, eval)(...)`, `globalThis.eval(...)`, `globalThis["eval"](...)`, optional eval の分類を plan に記録する。
- static source は global eval fragment として parse / resolve / lower する。
- caller lexical binding collision rewrite を global env lookup に置き換える。
- global lexical vs global object property vs module top-level lexical の違いを fixture 化する。
- optional eval について、nullish callee, shadowed eval, optional chaining short-circuit の edge を追加する。

Exit criteria:

- static indirect / optional eval literal source は host import なし。
- caller local non-capture が plan validation で保証される。
- global env semantics が `globalThis` property rewrite ではなく model として表現される。

### Phase 5: static `Function` / `new Function` AOT lane を first-class plan 化

Status: **broad focused coverage implemented; plan migration open.**

現状:

- resolver emits `ResolvedExpr::FunctionConstructor`。
- static literal args は synthetic `FunctionExpr` へ展開される。
- default/rest/destructuring params、early errors、metadata、construct returns、`this`, `arguments`, `new.target`, no caller capture の fixtures がある。

作業:

- `FunctionConstructorPlan` を追加し、synthetic `FunctionExpr` への即時変換を段階的に置き換える。
- parameter strings は FormalParameters parse goal、body string は FunctionBody parse goal として扱う。
- source concatenation ではなく parse-goal based validation にする。
- body-level strict directive と parameter grammar の early error を plan 内で表現する。
- generated function object に `.name === "anonymous"`, `.length`, own `.prototype`, constructability を metadata として持たせる。
- `Function.prototype.toString()` の source representation を audit する。
- no caller capture validation を generated function の capture analysis に組み込む。
- `new Function(...)` と `Function(...)` の returned object identity / prototype / construct behavior を統合する。

Exit criteria:

- literal-only `Function` / `new Function` は host import なしで AOT。
- `FunctionConstructorPlan` から generated function object が作られる。
- synthetic source string hack は internal fallback か削除候補になる。

### Phase 6: dynamic indirect eval host lane の runtime-wide audit

Status: **host import / manifest / host-deny implemented; broader audit open.**

作業:

- host shim の non-string source behavior を spec と合わせる。
- dynamic indirect eval の result value が host external object/function の場合、runtime-wide bridge へ接続する。
- realm/global object semantics を `$262.evalScript` と language eval で分ける。
- manifest reason を exact に保つ。
- host-deny error message を user-facing diagnostic として整える。

Exit criteria:

- dynamic indirect eval は `host.eval.indirect` のみを要求する。
- static indirect eval は引き続き standalone。
- object/error/function return が host external contract で扱われる。

### Phase 7: dynamic `Function` / `new Function` host lane の host function contract 化

Status: **compile/call/callMethod/construct imports implemented; focused shim broadening open.**

現状:

- `host.function.compile` returns a host function handle。
- `host.function.call`, `host.function.callMethod`, `host.function.construct` がある。
- focused Node shim は primitive/string/object returns、metadata、construct calls、object/array growth、function-valued properties、receiver-bound methods、thrown compile/call errors を扱う。

作業:

- host function handle representation を runtime ABI と object model に固定する。
- callable / constructable / method-call receiver の three-way bridge を Lowered IR の facts に持たせる。
- `.length`, `.name`, `.prototype` property access を host function metadata plan にする。
- host function value が object property、array element、env-cell local、eval result、catch binding を通っても facts を失わないようにする。
- host compile SyntaxError / call error / construct error を catchable exception model に接続する。
- host-deny で compile/call/callMethod/construct が必要な fixture を全て拒否する。

Exit criteria:

- dynamic Function constructor compile-only fixture は `host.function.compile` のみ。
- returned handle call fixture は `host.function.compile` + `host.function.call`。
- method call fixture は `host.function.callMethod` を exact に要求する。
- construct fixture は `host.function.construct` を exact に要求する。

### Phase 8: dynamic direct eval env descriptor v2 / mutation ledger

Status: **focused env descriptor implemented; full direct eval open.**

現状:

- initialized env-cell descriptor がある。
- not-yet-initialized env binding は `UnsupportedEval` で拒否する。
- strict caller metadata と restricted binding guards がある。
- new `var` / function declaration persistence と destructuring var name extraction がある。
- host external object/function facts の一部保持がある。

作業:

- `EvalEnvDescriptor` を typed lowered object にする。
- descriptor に caller activation identity と writable/readable/TDZ binding table を持たせる。
- host shim result を `EvalMutationLedger` として受け取り、wasm env cells / locals / created binding table に write back する。
- runtime-created var/function bindings を後続 normal wasm code から読めるようにするか、読めない場合は明示 diagnostic にする。
- lexical declarations を eval-local に閉じ込め、TDZ / duplicate checks を parser-backed validation で実装する。
- `arguments`, `this`, `new.target`, `super`, private names, class home object, module imports/live bindings の support / diagnostic boundary を plan にする。
- current Node shim の restricted binding guard を parser-backed validation へ移す。

Exit criteria:

- dynamic direct eval の caller local read/write-back が descriptor + ledger で説明できる。
- TDZ を bypass しない。
- declaration persistence と normal code visibility の仕様が決まる。
- manifest は `host.eval.direct` plus derived host function/object imports only when actually needed.

### Phase 9: host external object/function runtime contract

Status: **focused shim implemented; runtime-wide contract open.**

作業:

- RawValue tag / host handle table / wasm object wrapper の single source of truth を作る。
- backend runtime support と Node shim test support の差をなくす。
- object identity、array growth、property get/set/delete、function-valued property、method receiver、exception object を common bridge にする。
- lowering facts:
  - `host_external_object_locals`
  - `host_external_function_locals`
  - env-cell facts
  - property-derived facts
  - catch-binding facts
  を centralize する。
- host external bridge が必要な場合だけ exact capabilities を manifest に出す。

Exit criteria:

- dynamic eval と dynamic Function が同じ host external object model を共有する。
- focused Node shim 専用 behavior が production runtime contract に昇格する。

### Phase 10: `$262.evalScript` / test262 ramp / cleanup

Status: **host hook connected; realm/test262 classification open.**

作業:

- `$262.evalScript(source)` を language eval ではなく harness host hook として分類し続ける。
- test262 preprocessor / harness の `new Function` stripping を見直す。
- reference coverage を以下に分類する。
  - static direct eval AOT
  - static indirect eval AOT
  - static Function constructor AOT
  - dynamic indirect eval host
  - dynamic direct eval host descriptor
  - dynamic Function constructor host
  - host external object/function bridge
  - realm / cross-realm / `$262.evalScript` gap
- `UnsupportedEval` burn-down artifact を作る。
- old expected-fail が semantic gap か harness gap か security/capability policy gap かを分ける。

Exit criteria:

- eval / Function constructor による expected-fail は「未実装だから大量に残る」状態ではなく、明確な category に分類される。

## 6. File-by-file task map

| File / area | 現状 | 次の変更方針 |
|---|---|---|
| `crates/frontend/src/parser/eval_expand.rs` | removed | 復活させない。parser semantic rewrite 禁止を docs/comment 化 |
| `crates/frontend/src/parser/statements_general.rs` | eval statement splice removed | call shape preservation の regression test を維持 |
| `crates/frontend/src/parser/expressions_main.rs` | optional/member/comma eval shape を保持 | optional nullish / shadowed eval edge を追加 |
| `crates/syntax/src/ast.rs` | eval/function call shape helpers | semantic helper ではなく syntactic helper に限定 |
| `crates/resolve/src/name_resolver.rs` | strict context / lexical facts を供給 | scope id / strict metadata / shadowing facts を `EvalFragmentPlan` へ接続 |
| `crates/ir/src/builtin_resolved.rs` | `EvalFragmentPlan`, `FunctionConstructor`, `EvalCompletionStep` あり | plan fields を scope/declaration/completion/host policy へ拡張 |
| `crates/ir/src/builtin_resolver.rs` / `builtin_resolver_host.rs` | eval/function host classification あり | direct/indirect/optional/member/new classification を plan-owned にする |
| `crates/compiler/src/stages/eval_expand.rs` | static eval/function expansion の中心 | canonical `EvalFragment` AOT expansion stage として整理し、ad-hoc visible binding/collision logic を plan 化 |
| `crates/compiler/src/pipeline.rs` / `server.rs` / `dump/mod.rs` | eval expansion stage parity あり | future plan migration 後も parity test を残す |
| `crates/ir/src/lowered/program_direct_eval.rs` | block function env / dynamic direct eval env-cell collection | `EvalEnvDescriptor` v2 と declaration landing plan に統合 |
| `crates/ir/src/lowered/resolver/expr/mod.rs` | `ResolvedExpr::Eval` lower、env descriptor、TDZ diagnostic | descriptor typed lowering、mutation ledger、full TDZ modeling |
| `crates/ir/src/lowered/resolver/call/method.rs` | host external object method call -> `FunctionCallMethodHost` slice | method receiver bridge を host external contract に統合 |
| `crates/ir/src/lowered/facts.rs` / `ctx.rs` | host external/env-cell facts | fact propagation の single source of truth を作る |
| `crates/runtime-catalog/src/host_import.rs` | eval/function imports exist, including `FunctionCallMethod` | names/params/result を manifest / backend / shim と exact に保つ |
| `crates/runtime-catalog/src/capability.rs` | eval/function capabilities exist | reason derivation と host-deny coverage を維持 |
| `crates/runtime-catalog/src/link_plan.rs` | imports -> capabilities mapping exists | host external bridge imports の derived capability を audit |
| `crates/backend-wasm/src/runtime/host/emit.rs` | eval/function host wrappers call imports | silent `unreachable` を混ぜず catchable exception bridge を維持 |
| `crates/backend-wasm/src/capability_manifest.rs` | host eval/function reasons emitted | exact import ordering / reason text regression を維持 |
| `crates/cli/tests/m11_host_deny.rs` | static no-host and dynamic host-deny tests broad | new host external / method / descriptor casesを追加 |
| `crates/cli/tests/node_shim_host.rs` | focused Node shim broad | production contract と test shim の差分を縮める |
| `fixtures/core-semantics/*eval*` | static/dynamic/direct/indirect/optional fixtures broad | unsupported boundary comments と test262 classification を追加 |
| `fixtures/core-semantics/*function-constructor*` | static/dynamic Function fixtures broad | first-class plan migration regressions を追加 |
| `scripts/lib/test262_harness.py` / `crates/compiler/src/test262_preprocessor.rs` | `$262.evalScript` hook connected | `new Function` stripping / eval categorization を burn-down 用に整理 |
| `current-state.md` / docs | partial implementation 記載あり | `host.function.callMethod`, descriptor v2 remaining, host external contract を明記 |

## 7. Acceptance gates

| Gate | 条件 | archive15 状態 |
|---|---|---|
| G0 tracking/docs | issue/docs/fixtures が current support matrix と一致 | partially done; この md で更新 |
| G1 parser shape | parser が eval/function semantics を決めない | mostly done; regression guard 必須 |
| G2 static direct eval AOT | expression completion + side effect + declaration landing + no host import | broad focused slice done; canonical env open |
| G3 static direct eval declarations | var/function/class/lexical/destructuring + strict/sloppy env | partial; full lexical/TDZ open |
| G4 Annex B block function | parser splice なしで eval lowering path pass | focused done; plan migration open |
| G5 static indirect/optional eval | global semantics + no host import | partial; global env model open |
| G6 static Function constructor | literal-only AOT + metadata + no caller capture + no host import | broad focused slice done; first-class plan open |
| G7 dynamic indirect eval | exact `host.eval.indirect` manifest + host-deny + Node shim | focused done; realm/object audit open |
| G8 dynamic Function constructor | exact `host.function.compile/call/callMethod/construct` as needed | focused done; host handle contract open |
| G9 dynamic direct eval | env descriptor + strict metadata + write-back + host-deny | focused done; descriptor v2 / TDZ / ledger open |
| G10 host external contract | object/function/error identity and property/method bridge runtime-wide | focused shim only |
| G11 test262 ramp | `$262.evalScript` separated; UnsupportedEval categorized | started; broader burn-down open |

全 gate の共通条件:

- static lane artifact に `host.eval.*` / `host.function.*` を混入させない。
- dynamic host lane は exact import names と exact capability reasons を出す。
- host-deny では dynamic host lane を明確に拒否する。
- backend WAT に silent `unreachable` eval stub を混ぜない。
- parser-only semantic rewrite を新規 acceptance としない。
- `EvalKind::Direct` / `Indirect` を lowering/backend validation で保持する。

## 8. 推奨 PR 分割

### PR A: plan/docs/current-state refresh

- この md を `plans/eval-new-function-implementation-plan.md` に反映。
- `current-state.md`, `docs/language-reference/javascript-features.md`, `docs/26-semantic-feature-matrix.md` に archive15 status を揃える。
- `host.function.callMethod`、dynamic direct eval strict restricted-binding guard、host external contract remaining を追記。
- unsupported fixture comments を更新。

### PR B: `EvalFragmentPlan` canonicalization

- `EvalFragmentPlan` に scope/declaration/completion/host fields を追加。
- `EvalCompletionStep` generation を plan-owned に移す。
- static direct / indirect eval expansion が plan を consume するようにする。
- stage parity tests を維持。

### PR C: static direct eval environment model

- `EvalDeclarationPlan`, `EvalLexicalEnvPlan`, `EvalBlockFunctionPlan`。
- sloppy var/function landing、strict lexical isolation、TDZ/duplicate checks。
- class/destructuring completion と abrupt completion を Completion Record に接続。

### PR D: static indirect/global eval model

- `EvalScopeMode::Global`。
- caller collision rewrite を global env lookup に置換。
- global lexical/object/module realm fixtures。
- optional eval edge fixtures。

### PR E: static `FunctionConstructorPlan`

- synthetic `FunctionExpr` fallback を first-class plan に置換。
- parse-goal based FormalParameters / FunctionBody。
- metadata/prototype/toString/no-capture validation。
- Node-compatible SyntaxError timing。

### PR F: host external object/function contract

- RawValue host external tags / handle table contract。
- property get/set/delete, array growth, function-valued properties, method receiver。
- env-cell/catch/property fact propagation。
- `host.function.callMethod` exact capability coverage。

### PR G: dynamic direct eval descriptor v2

- typed env descriptor。
- mutation ledger。
- full TDZ model。
- runtime-created binding normal-code visibility decision。
- parser-backed strict restricted binding validation。

### PR H: test262 ramp and harness cleanup

- `$262.evalScript` realm classification。
- `new Function` stripping cleanup。
- UnsupportedEval burn-down categories。
- reference coverage artifact update。

## 9. 直近で最も効果が高い作業

次の最小作業は **PR A + PR B の一部**である。

1. この計画を `plans/eval-new-function-implementation-plan.md` に反映する。
2. `EvalFragmentPlan` を拡張する前に、現行 `EvalCompletionStep` の実装済み範囲を tests と docs で固定する。
3. `eval_expand.rs` の ad-hoc context を `EvalFragmentPlan` 由来の context に置き換え始める。
4. static direct eval declarations の plan-owned representation を入れる。
5. dynamic direct eval descriptor は、現在の focused shim を壊さずに typed descriptor v2 の設計だけ先に置く。

この順番なら、すでに広がった supported slice を壊さず、parser rewrite に戻らず、direct eval の最難所へ進める。

## 10. リスクと対策

| Risk | 影響 | 対策 |
|---|---|---|
| current focused shim を complete と誤認する | TDZ/lexical/env/realm bugs が残る | support matrix に focused vs canonical を明記 |
| parser semantic rewrite が戻る | shadowed eval/function が壊れる | parser の責務を shape preservation に限定し regression test |
| `EvalCompletionStep` が ad-hoc stage に固定される | declaration/completion/env が分散 | `EvalFragmentPlan` / `EvalCompletionPlan` に移行 |
| static indirect eval collision rewrite が global env model を隠す | global lexical vs global object bugs | `EvalScopeMode::Global` を導入 |
| synthetic FunctionExpr が Function constructor semantics を隠す | metadata/toString/prototype/realm bugs | `FunctionConstructorPlan` へ移行 |
| dynamic direct eval descriptor が initialized cells だけに依存する | TDZ / runtime-created binding visibility が不完全 | descriptor v2 + mutation ledger |
| strict restricted binding guard が heuristic のまま | false positive/negative | parser-backed validation へ移す |
| host external object facts が局所的 | object/function handle が env/catch/property 経由で消える | centralized host external fact propagation |
| manifest import set が広くなる | sandbox/capability trust 低下 | exact import derivation and host-deny tests |
| `$262.evalScript` と language eval を混同 | test262 coverage 分析が濁る | harness hook category を別管理 |

## 11. 完了時の状態

完全実装時には以下が成立する。

- static direct eval は expression completion、side effects、supported declarations、Annex B block functions、strict/sloppy lexical environment を wasm-native に実行する。
- static indirect / optional eval は global semantics で wasm-native に実行し、caller locals を capture しない。
- literal-only `Function` / `new Function` は first-class `FunctionConstructorPlan` から generated wasm function object を作り、metadata / prototype / constructability / no-capture semantics を持つ。
- dynamic indirect eval は exact `host.eval.indirect` capability で動く。
- dynamic Function constructor は exact `host.function.compile` / `host.function.call` / `host.function.callMethod` / `host.function.construct` capability で動く。
- dynamic direct eval は typed env descriptor + mutation ledger + TDZ validation + write-back により caller scope mutation を検証可能に扱う。
- host external object/function/error bridge は focused shim ではなく runtime-wide contract になる。
- `$262.evalScript` は language eval から分離された harness hook として test262 coverage に分類される。
- `UnsupportedEval` は「eval 全般が未実装」ではなく、realm、cross-realm、unsupported binding kind、security/capability policy など明確な理由だけに残る。
