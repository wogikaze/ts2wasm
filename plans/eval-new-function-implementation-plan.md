# eval / `Function` constructor 実装計画

Created: 2026-05-17  
Primary issue: `issues/I-20260517-WE8P5A.md` (`UnsupportedEval: implement eval support (596 cases)`)  
Related issues: `issues/done/I-20260513-HD4K3Q.md`, `issues/done/I-20260513-B49ZZE.md`, `issues/I-20260513-WBEJBE.md`, `issues/I-20260515-7N7MWQ.md`

## 1. 目的

`eval` と `Function` / `new Function` は、このプロジェクトが「TypeScript / JavaScript source を wasm に変換する compiler」であり続けられるかを決める難所である。
この document の目的は、次の 2 つを両立する実装順序を固定することである。

1. **静的に解析できるコードは wasm としてコンパイルする。**
   - `eval("...")` や `new Function("...", "...")` の source が compile-time literal として確定している場合、可能な範囲では runtime eval へ逃がさず、通常の parse / resolve / lower / backend pipeline に乗せる。
   - この場合、capability manifest は standalone のままにする。eval そのものを理由に Node host を要求してはならない。
2. **本当に runtime でしか分からない dynamic code は、明示的な capability と監査可能な Node host shim として扱う。**
   - `host.eval.*` / `host.function.*` import を exact に manifest へ出す。
   - `standalone: false` と `node_host.imports` / `capability_reasons` が一致しない artifact は emit しない。
   - whole-program source を Node.js `eval` へ渡す wrapper 実装は禁止する。host shim が扱うのは、ユーザーコードが runtime で生成した dynamic source value に限定する。

この方針は `docs/01-project-definition.md` の禁止事項と、`docs/14-runtime-abi.md` の `Direct eval execution strategy` を前提にする。

## 2. 現状整理

### 2.1 実装済みの薄い slice

現在は static string の direct eval expression statement だけが部分対応されている。

- `crates/syntax/src/ast.rs`
  - `Expr::is_direct_eval_call()` と `Expr::direct_eval_literal_source()` がある。
- `crates/frontend/src/parser/statements_general.rs`
  - `eval("...");` のような expression statement を parser 内で eval source の statement 群に展開する。
  - block function declaration の一部 pattern を特別扱いして caller scope へ展開する。
- `crates/ir/src/lowered/program_direct_eval.rs`
  - direct eval block function binding の env cell / heap closure 補助がある。
- `issues/done/I-20260513-HD4K3Q.md`
  - done になっているが、実際の受け入れ範囲は static-string direct eval の compile-time 展開である。

この slice は重要だが、次の制限がある。

- `let x = eval("1 + 1")` のような **eval の completion value を式として使うケース** は、parser statement 展開だけでは扱いにくい。
- `eval` binding の shadowing 判定が parser の `possible_eval_shadowing` heuristic に依存している。
- direct / indirect / optional / member eval の判定が parser と resolver に分散している。
- dynamic source は未対応で、runtime ABI / capability manifest の形がまだない。

### 2.2 現在の拒否箇所

- `crates/frontend/src/parser/expressions_main.rs`
  - `globalThis.eval("x")`, `globalThis["eval"]("x")`, `(0, eval)("x")`, `eval?.("x")` を parser 段階で `issue-347` として拒否している。
- `crates/resolve/src/name_resolver.rs`
  - unshadowed `eval(...)` を `UnsupportedEval` / `issue-429` で拒否する path が残っている。
  - unqualified `Function(...)` / `new Function(...)` を `issue-062` で拒否している。
  - `new eval` も `UnsupportedEval` で拒否している。
- `crates/backend-wasm/src/runtime/host/emit.rs`
  - `$dollar_262_eval` は `unreachable` 実装で、test262 host hook の実行体がない。
- `crates/runtime-catalog/src/host_import.rs`
  - `HostImport` に eval / function constructor 用の NodeShim import がない。
- `crates/runtime-catalog/src/capability.rs`
  - eval / function constructor 用の Host capability がない。

### 2.3 tracking 上の注意

`issues/done/I-20260513-B49ZZE.md` は `Function constructor and indirect eval` が done のように見えるが、evidence は `build_smoke_function_constructor: PASS (diagnostic)` であり、実装完了ではない。
次の作業では、この issue を「diagnostic slice was accepted」と解釈し、実機能は `I-20260517-WE8P5A` 配下で再設計する。

## 3. 非目標と禁止事項

この実装では以下を行わない。

- original TypeScript / JavaScript source 全体を文字列として保持し、generated wasm から Node.js `eval` / `Function` / `vm` へ渡す wrapper 化。
- static literal eval まで host eval に逃がすこと。
- capability manifest なしで Node host eval を暗黙利用すること。
- parser 段階で indirect eval を構文エラー扱いし続けること。
- 仕様が難しいから `eval` / `Function` を permanent expected-fail に分類すること。

ただし、段階的実装のため、dynamic direct eval のうち caller lexical scope の read/write-back が必要なケースは最後の phase まで診断を維持してよい。
これは拒否ではなく、scope write-back model を安全に導入するための順序である。

## 4. 基本設計

### 4.1 two-lane design

`eval` / `Function` は、source の確定タイミングで 2 lane に分ける。

| Lane | 対象 | 実行方式 | capability | 目的 |
|---|---|---|---|---|
| AOT lane | source / params / body が compile-time literal として確定 | compiler pipeline で parse / resolve / lower し、wasm として emit | eval 自体では不要 | standalone を維持し、wrapper 化を防ぐ |
| Dynamic host lane | source が runtime value | audited Node host shim import | `host.eval.*` / `host.function.*` | pure wasm で不可能な runtime code generation を明示的に扱う |

この分岐は resolver / lowering で行い、parser の statement rewrite だけに閉じ込めない。

### 4.2 eval kind

`eval` は少なくとも次の kind を区別する。

| Kind | 例 | Scope | 優先実装 |
|---|---|---|---|
| direct eval | `eval(src)` | caller lexical / variable environment | static literal を最優先 |
| indirect eval | `(0, eval)(src)`, `globalThis.eval(src)` | global environment | static literal と dynamic host を優先 |
| optional direct eval | `eval?.(src)` | direct eval ではなく indirect-like 扱いになるケースがあるため spec 確認後 | 後続 |
| construct eval | `new eval(...)` | eval は constructor ではない | `TypeError` または現行診断を spec に寄せる |

初期実装では、parser が indirect eval を拒否せず、AST / resolved IR 上で `EvalKind::Indirect` として表現できることを優先する。

### 4.3 `Function` / `new Function` kind

`Function(...)` と `new Function(...)` は同じ constructor semantics を共有する。

- parameter string 群と body string を結合して function source を構築する。
- created function は caller lexical scope を capture しない。
- global scope で評価される。
- `name` は通常 `"anonymous"`。
- `length` は parameter count から決まる。
- call 可能な function object であり、`new` で construct 可能な通常 function として prototype を持つ。

compile-time literal だけで source が確定する場合は、generated function と function object metadata を compiler 側で合成する。
runtime value が混じる場合は dynamic host lane に入れる。

## 5. 意味論 contract

### 5.1 direct eval

`eval(src)` が unshadowed intrinsic eval への direct call である場合、以下を満たす。

- `src` が string でない場合は `src` をそのまま返す。
- `src` が string の場合、eval code として parse する。
- sloppy mode では `var` / function declaration が caller variable environment に作用する。
- strict mode eval code の lexical declaration は eval code 内に閉じる。
- completion value は `eval` expression の戻り値になる。
- eval 内の exception は caller 側へ伝搬する。
- direct eval によって `new.target`, `this`, `arguments`, function/class bindings へ影響するケースは、phase ごとに support matrix を明示する。

最初の AOT direct eval phase では、literal source を `EvalFragment` として保持し、caller scope context 付きで resolve / lower する。
現行の parser 展開は互換性のために残してもよいが、最終的には expression-level completion value を扱える IR path へ移す。

### 5.2 indirect eval

indirect eval は global eval として扱う。

- caller lexical bindings を参照・変更しない。
- global `var` / function declaration は global object / global env へ作用する。
- static literal indirect eval は AOT lane で global eval fragment として lower できる。
- dynamic source は `host.eval.indirect` に限定して Node host shim へ渡す。

### 5.3 dynamic direct eval

dynamic direct eval は最も危険で、caller local binding の read/write-back が必要になる。

MVP では次のように段階分けする。

1. source が compile-time literal の direct eval は AOT lane で実装する。
2. source が dynamic だが caller lexical / local binding を必要としないと静的に証明できるケースだけ `host.eval.direct.global_only` 相当で許可する。
3. caller local の read/write-back が必要な dynamic direct eval は、env snapshot / mutation ledger / write-back table が実装されるまで `UnsupportedEval` を維持する。

この順序により、不完全な host eval で local mutation を取りこぼす事故を避ける。

### 5.4 Function constructor

`Function` constructor は direct eval ではなく global function compilation として扱う。

- literal args only:
  - parameter list と body を compile-time に parse する。
  - generated function を `LoweredProgram.generated_functions` に追加する。
  - function object metadata (`length`, `name`, `prototype`, constructability) を runtime object model に登録する。
- dynamic args:
  - Node host shim で compile する。
  - returned host function object は RawValue として表現し、call / construct / property access の境界を明示する。
  - first-class function object model (`issues/I-20260513-WBEJBE.md`) と連携する。

## 6. AST / resolved IR / lowered IR 設計

### 6.1 parser / syntax

現行の `Expr::Call` と `Expr::New` は維持するが、helper を増やす。

追加候補:

```rust
pub enum EvalCalleeKind {
    Direct,
    IndirectMember,
    IndirectComma,
    IndirectOptional,
}

pub enum DynamicCodeSource {
    StaticLiteral(String),
    RuntimeExpr(Box<Expr>),
    NonStringExpr(Box<Expr>),
}
```

parser の責務:

- syntactic shape を落とさず AST に残す。
- indirect eval を parser diagnostic にしない。
- literal source の tokenization / nested parse は resolver/lowering へ移すか、少なくとも `EvalFragment` として source span と mode を保持する。
- `Function(...)` / `new Function(...)` の args は通常の call/new expression として保持する。

削除・緩和する箇所:

- `Parser::indirect_eval_call_diagnostic` による即時 rejection。
- `possible_eval_shadowing` による過剰 rejection。shadowing は resolver の binding table で判断する。

### 6.2 resolver

resolver は次を判定する。

- `eval` が user binding で shadow されているか。
- call が direct eval か indirect eval か。
- source が static literal か runtime value か。
- `Function` が user binding で shadow されているか。
- `Function(...)` / `new Function(...)` の args がすべて static literal か。
- target policy が dynamic host eval を許すか。

追加候補:

```rust
pub enum ResolvedDynamicCodeKind {
    DirectEval,
    IndirectEval,
    FunctionConstructor { called_with_new: bool },
}

pub enum ResolvedExpr {
    // existing variants ...
    Eval {
        kind: EvalKind,
        source: EvalSource,
        caller_scope: Option<ScopeId>,
        strict_caller: bool,
        span: Span,
    },
    FunctionConstructor {
        args: Vec<ResolvedExpr>,
        static_plan: Option<StaticFunctionConstructorPlan>,
        called_with_new: bool,
        span: Span,
    },
}
```

resolver diagnostic policy:

- shadowed `eval` / `Function` は通常 identifier / call として扱う。
- unshadowed eval / Function は専用 IR へ変換する。
- host lane が disabled の場合は、dynamic source に対して `UnsupportedEval` または `UnsupportedRuntimeSubset` を出す。
- diagnostic message には direct / indirect / Function constructor / static / dynamic の分類を含める。

### 6.3 lowering

lowering は AOT lane と dynamic host lane を分けて `LoweredExpr` にする。

追加候補:

```rust
pub enum LoweredExpr {
    // existing variants ...
    EvalStaticFragment {
        kind: EvalKind,
        fragment_id: EvalFragmentId,
        span: Span,
    },
    EvalHostCall {
        kind: EvalKind,
        source: Box<LoweredExpr>,
        env: Option<EvalEnvDescriptorId>,
        span: Span,
    },
    FunctionConstructorStatic {
        function_id: FuncId,
        metadata: FunctionObjectMetadata,
        span: Span,
    },
    FunctionConstructorHost {
        params_and_body: Vec<LoweredExpr>,
        span: Span,
    },
}
```

必要な補助構造:

- `EvalFragmentId`
  - eval source text, strict flag, caller/global scope mode, original span を保持。
- `EvalCompletionSlot`
  - eval fragment の completion value を expression result に戻す。
- `EvalHoistPlan`
  - sloppy direct eval の `var` / function declaration を caller variable env に接続する。
- `EvalEnvDescriptor`
  - dynamic direct eval 用。scope id、readable binding list、writable binding list、write-back slot を持つ。
- `FunctionObjectMetadata`
  - `name`, `length`, `prototype`, constructable, strict, source span。

### 6.4 backend

backend は次を emit する。

- `EvalStaticFragment`
  - 通常の lowered statements / completion record として emit。
  - host import を追加しない。
- `EvalHostCall`
  - runtime catalog 経由で該当 `RuntimeFn` を要求する。
  - import 文字列を backend が直書きしない。
- `FunctionConstructorStatic`
  - generated function を callable object として allocate / return。
  - `.length`, `.name`, `.prototype` metadata を既存 function object model と揃える。
- `FunctionConstructorHost`
  - `host.function.compile` で host function handle を作る。
  - call / construct / property access に必要な bridge を runtime function に集約する。

## 7. Runtime catalog / capability / host ABI

### 7.1 HostImport 追加案

`crates/runtime-catalog/src/host_import.rs` に NodeShim import を追加する。

```rust
HostImport::EvalIndirect,
HostImport::EvalDirectGlobalOnly,
HostImport::FunctionCompile,
HostImport::FunctionCall,
HostImport::FunctionConstruct,
```

manifest name 例:

| HostImport | spec.name | manifest import |
|---|---|---|
| `EvalIndirect` | `eval.indirect` | `host.eval.indirect` |
| `EvalDirectGlobalOnly` | `eval.directGlobalOnly` | `host.eval.directGlobalOnly` |
| `FunctionCompile` | `function.compile` | `host.function.compile` |
| `FunctionCall` | `function.call` | `host.function.call` |
| `FunctionConstruct` | `function.construct` | `host.function.construct` |

`host.eval.direct` という名前を最初から広く使うと、caller local mutation まで実装済みに見える。
MVP では `directGlobalOnly` のように制限を名前へ含め、full direct eval は env write-back 実装後に追加する。

### 7.2 Capability 追加案

`crates/runtime-catalog/src/capability.rs` に次を追加する。

```rust
Capability::HostEvalIndirect,
Capability::HostEvalDirectGlobalOnly,
Capability::HostFunctionCompile,
Capability::HostFunctionCall,
Capability::HostFunctionConstruct,
```

次の箇所も同時に更新する。

- `Capability::manifest_name()`
- `node_shim_import_to_capability()`
- `cap_is_host()`
- `RuntimeSpec` の imports / capability arrays
- `RuntimeLinkPlan::populate_derived_sets()` の reason 生成
- `backend-wasm/src/capability_manifest.rs` の manifest reason key

特に manifest validation は `node_host.imports` の各 import 名に一致する `capability_reasons` key を要求する。
そのため、`host.eval.indirect` の import を出す場合は `capability_reasons["host.eval.indirect"]` を必ず追加する。

### 7.3 RuntimeFn 追加案

`crates/runtime-catalog/src/runtime_fn.rs` に追加する。

```rust
RuntimeFn::EvalIndirectHost,
RuntimeFn::EvalDirectGlobalOnlyHost,
RuntimeFn::FunctionCompileHost,
RuntimeFn::FunctionCallHost,
RuntimeFn::FunctionConstructHost,
RuntimeFn::FunctionObjectCreateStatic,
```

`RuntimeSpec` では、host lane の runtime fn だけ NodeShim import と Host capability を持つ。
static lane の `FunctionObjectCreateStatic` は import を持たない。

### 7.4 Host ABI の粒度

初期 ABI は RawValue i32 を中心にする。

- `host.eval.indirect(source_value: RawValue, strict_flag: i32) -> RawValue`
- `host.eval.directGlobalOnly(source_value: RawValue, strict_flag: i32) -> RawValue`
- `host.function.compile(params_array_value: RawValue, body_value: RawValue, strict_flag: i32) -> RawValue`
- `host.function.call(function_value: RawValue, this_arg: RawValue, args_array_value: RawValue) -> RawValue`
- `host.function.construct(function_value: RawValue, args_array_value: RawValue) -> RawValue`

exception handling は host shim 側で JS exception を runtime exception representation へ変換する。
この変換が未整備なら、初期 slice では thrown error を `UnsupportedRuntimeSubset` に落とすのではなく、runtime helper と test を先に入れる。

### 7.5 Node host shim の安全条件

- shim は generated wasm の exported memory を通じて source RawValue を decode する。
- shim は original module source 全体を受け取らない。
- shim は import 単位で tree-shake される。
- shim は `eval` / `Function` を使う import を manifest に露出する。
- `--host-deny node` または host-deny validation では、dynamic host lane を明確に fail させる。

## 8. 実装 phase

### Phase 0: audit と tracking 修正

目的: 既存の done/diagnostic slice と本実装範囲を混同しない。

作業:

- `issues/done/I-20260513-B49ZZE.md` の note に「diagnostic acceptance only」を追記するか、`I-20260517-WE8P5A` から明示的に参照する。
- `docs/language-reference/javascript-features.md` の eval row を「static direct eval partial; dynamic/indirect/function planned by docs/28」に更新する。
- `docs/14-runtime-abi.md` の Direct eval strategy と本 doc の host ABI 名を揃える。
- parser / resolver / lowering / backend の current rejection point を grep で inventory 化する。

Exit criteria:

- eval / Function の current support matrix が docs と issue 上で一致している。
- diagnostic-only done を機能完了と誤読しない状態になっている。

### Phase 1: eval を expression-level static AOT に昇格

目的: `eval("...")` を statement rewrite ではなく expression として扱い、completion value を正しく返す。

作業:

- `Expr::Call` から unshadowed direct eval を resolver で `ResolvedExpr::Eval { kind: Direct, source: StaticLiteral, ... }` へ変換する。
- eval source を `EvalFragment` として parse し、caller scope context を渡して resolve する。
- `eval("1 + 1")` / `let x = eval("1 + 1")` / `return eval("x")` の completion value を返す。
- `eval("var x = 1")` の caller variable env hoisting を整理する。
- 現行 parser expansion は compatibility path として一時的に残し、同じ fixture が新 path で通るようになったら削る。

主な変更ファイル:

- `crates/frontend/src/parser/statements_general.rs`
- `crates/syntax/src/ast.rs`
- `crates/resolve/src/name_resolver.rs`
- `crates/ir/src/builtin_resolved.rs`
- `crates/ir/src/lowered/program.rs`
- `crates/ir/src/lowered/program_direct_eval.rs`
- `crates/backend-wasm/src/expr_emit.rs`

新規テスト例:

```ts
let x = 1;
let y = eval("x + 2");
console.log(y); // 3
```

```ts
function f() {
  let x = "before";
  eval('x = "after";');
  return x;
}
console.log(f()); // after
```

```ts
let r = eval('1; 2;');
console.log(r); // 2
```

Exit criteria:

- static direct eval は host import を追加しない。
- `m11_host_deny` に「static direct eval remains standalone」を追加し、pass する。
- completion value を使う eval fixtures が Node differential で一致する。

### Phase 2: static literal indirect eval

目的: indirect eval の parser rejection を外し、static literal は global eval fragment として wasm 化する。

作業:

- `Parser::indirect_eval_call_diagnostic` を削除または feature gate 下へ移動する。
- resolver で `(0, eval)("...")`, `globalThis.eval("...")`, `globalThis["eval"]("...")` を `EvalKind::Indirect` にする。
- source が static literal の場合、global scope context で parse / resolve / lower する。
- caller local を参照した場合は spec 通り global lookup になり、unresolved なら global unresolved / ReferenceError path になるようにする。

新規テスト例:

```ts
let x = 1;
globalThis.x = 10;
let y = (0, eval)("x");
console.log(y); // 10
```

Exit criteria:

- parser tests の `rejects_indirect_eval_calls_with_issue_347` を「marks/resolves indirect eval」に置き換える。
- static indirect eval も host import なしで standalone。
- global scope 参照と caller local 非 capture の differential test が pass する。

### Phase 3: static `Function` / `new Function`

目的: literal args の `Function` constructor を runtime codegen なしで generated wasm function にする。

作業:

- resolver で unshadowed `Function(...)` / `new Function(...)` を専用 expression に変換する。
- args がすべて static string literal の場合、parameter list と body を compile-time に parse する。
- parse 時に `Function` constructor 特有の parameter/body 結合ルールを実装する。
- generated function は global scope で resolve し、caller lexical scope を capture しない。
- function object metadata を作る。
  - `.length`
  - `.name === "anonymous"`
  - `.prototype`
  - callable / constructable flag
- `issues/I-20260513-WBEJBE.md` の first-class Function object model と統合する。

新規テスト例:

```ts
let f = new Function("a", "b", "return a + b;");
console.log(f(1, 2)); // 3
```

```ts
let x = 1;
let f = Function("return typeof x;");
console.log(f()); // global lookup; caller local を capture しない
```

```ts
let f = new Function("a", "b", "return a + b;");
console.log(f.length);
console.log(f.name);
```

Exit criteria:

- literal-only `Function` / `new Function` は host import なしで動く。
- 現行 `build_smoke_function_constructor` は diagnostic expectation から behavior expectation に更新する。
- `Function` が user binding で shadow されている場合は従来通り user call として扱う。

### Phase 4: dynamic indirect eval host lane

目的: pure wasm で不可能な runtime source の indirect eval を capability-gated Node host shim で実装する。

作業:

- `HostImport::EvalIndirect`, `Capability::HostEvalIndirect`, `RuntimeFn::EvalIndirectHost` を追加する。
- runtime link plan validation に import/capability/reason の対応を追加する。
- capability manifest に `host.eval.indirect` と reason を出す。
- backend runtime helper `$eval_indirect_host` を追加する。
- Node shim import `host.eval.indirect` を実装する。
- `--host-deny node` では compile または validation で明確に fail させる。

新規テスト例:

```ts
let src = "1 + 2";
console.log((0, eval)(src)); // 3, node-host target only
```

manifest expectation:

```json
{
  "standalone": false,
  "node_host": {
    "required": true,
    "imports": ["host.eval.indirect"]
  },
  "capability_reasons": {
    "host.eval.indirect": ["indirect eval with runtime source"]
  }
}
```

Exit criteria:

- dynamic indirect eval fixture passes on node-shim target.
- same fixture fails under host-deny with explicit diagnostic.
- static indirect eval fixture still has no `host.eval.indirect` import.

### Phase 5: dynamic `Function` / `new Function` host lane

目的: runtime-generated params/body を Node host shim で compile し、function object として扱う。

作業:

- `HostImport::FunctionCompile`, `Capability::HostFunctionCompile`, `RuntimeFn::FunctionCompileHost` を追加する。
- dynamic host function handle の value representation を定義する。
  - wasm heap object wrapping host handle id
  - or RawValue object tag with host external id table
- call / construct bridge を追加する。
  - 既存 `ReflectApply` / `ReflectConstruct` と競合しないようにする。
  - 必要なら `host.function.call` / `host.function.construct` を追加する。
- property access bridge を定義する。
  - minimum: `.length`, `.name`, `.prototype`
  - follow-up: arbitrary property get/set
- thrown SyntaxError / runtime error を caller へ伝搬する。

新規テスト例:

```ts
let body = "return a + b;";
let f = new Function("a", "b", body);
console.log(f(1, 2)); // 3, node-host target only
```

```ts
let f = Function("return this === globalThis;");
console.log(f());
```

Exit criteria:

- dynamic `new Function` passes on node-shim target with manifest import.
- host-deny fails clearly.
- literal-only `new Function` remains standalone.

### Phase 6: dynamic direct eval with env descriptor

目的: caller local binding を読む/書く dynamic direct eval を安全に実装する。

作業:

- `EvalEnvDescriptor` を lower する。
  - readable bindings
  - writable bindings
  - var/function declaration landing zone
  - lexical declaration isolation for strict eval
- host shim へ env snapshot object を渡す。
- host eval 後、mutation ledger を wasm env cells / locals へ write back する。
- local を env cell 化する必要がある関数を事前に mark する。
- unsupported な binding kind は diagnostic にする。
  - TDZ が必要な lexical binding
  - private name
  - module live binding
  - nested closure mutation と競合する binding

Exit criteria:

- dynamic direct eval が caller local read/write を Node differential で一致させる。
- env descriptor の write-back missing は validation で検出される。
- dynamic direct eval による host import は full direct eval 用の名前で manifest に出る。

### Phase 7: test262 ramp と expected-fail 再分類

目的: `I-20260517-WE8P5A` の 596 blocked cases を段階的に burn down する。

作業:

- `reference-coverage` の eval / Function constructor filter を作る。
- test262 result を次に分類する。
  - static direct eval AOT
  - static indirect eval AOT
  - static Function constructor AOT
  - dynamic indirect eval host
  - dynamic Function constructor host
  - dynamic direct eval env write-back
  - realm / cross-realm / `$262.evalScript` gap
- `docs/15-coverage-matrix.md` と generated coverage artifact のラベルを更新する。

Exit criteria:

- UnsupportedEval 件数が phase ごとに下がる。
- expected-fail は「実装しないから」ではなく、realm/harness 等の明確な未対応理由に限定される。

## 9. テスト計画

### 9.1 unit tests

- parser
  - indirect eval を reject しない。
  - direct eval syntactic shape を保持する。
  - optional/member/comma eval shape を保持する。
- resolver
  - unshadowed eval / Function は専用 IR へ変換する。
  - shadowed eval / Function は user binding として扱う。
  - static/dynamic source の分類が正しい。
- runtime catalog
  - new HostImport が manifest name / spec.name / wat_symbol を持つ。
  - NodeShim import と Host capability の対応が validation で通る。
  - reason key が `host.*` import 名と一致する。
- capability manifest
  - dynamic eval import は `standalone:false`。
  - static eval は import なし。
  - missing reason は validation failure。

### 9.2 integration / CLI tests

- `m6_builtin_methods.rs`
  - static direct eval expression value。
  - static indirect eval。
  - static `new Function`。
  - dynamic indirect eval node-shim。
  - dynamic `new Function` node-shim。
- `m11_host_deny.rs`
  - static direct eval は host-deny でも pass。
  - dynamic eval は host-deny で fail。
- `m2_node_diff.rs`
  - Node output と比較する fixtures を追加。

### 9.3 negative tests

- `eval` shadowing:

```ts
function f(eval) { return eval("x"); }
```

この場合は intrinsic direct eval ではなく通常 call である。

- `Function` shadowing:

```ts
const Function = (x) => x;
console.log(Function("return 1"));
```

この場合は dynamic code generation ではない。

- host denied:

```ts
let s = "1 + 1";
(0, eval)(s);
```

`--host-deny node` では capability denial になる。

- syntax error timing:

```ts
new Function("return ;;");
```

Node と同じタイミングで SyntaxError を出す。

## 10. Acceptance gates

この作業は難所なので、phase ごとに以下を gate にする。

| Gate | 条件 |
|---|---|
| G1 static direct eval | `eval("expr")` が expression value を返し、host import なし |
| G2 static indirect eval | parser rejection なし、global scope semantics、host import なし |
| G3 static Function constructor | literal-only `new Function` が generated wasm function として動く |
| G4 dynamic indirect eval | `host.eval.indirect` が manifest に exact に出て、node-shim で pass |
| G5 dynamic Function constructor | `host.function.compile` 系 import が exact に出て、function object call が pass |
| G6 dynamic direct eval | env descriptor / write-back が validated され、caller local mutation が pass |
| G7 coverage | `UnsupportedEval` の減少が reference-coverage artifact で確認できる |

すべての gate で共通:

- panic / `unreachable` による失敗は禁止。
- unsupported は `UnsupportedEval` などの分類済み diagnostic にする。
- host import を追加した場合、runtime catalog / link plan / capability manifest / generated shim が同じ import name を使う。
- standalone artifact に Node host eval import が混入しない。

## 11. リスクと対策

| Risk | 影響 | 対策 |
|---|---|---|
| direct eval の scope mutation を取りこぼす | observable semantics が壊れる | static AOT を先行し、dynamic direct は env descriptor 完成まで制限 |
| parser rewrite が completion value を失う | `eval("1;2")` が不正 | expression-level `EvalFragment` へ移行 |
| Function constructor が caller scope を capture する | spec 違反 | global scope resolve を強制し、capture list を空に validate |
| host eval が wrapper 化に見える | project identity 破壊 | host lane は runtime-generated source のみに限定し、manifest に exact import を出す |
| manifest reason mismatch | artifact validation failure | `host.*` import key と reason key の一致を unit test 化 |
| dynamic host function object が wasm object model と乖離 | call/property が壊れる | first-class Function object issue と統合し、host handle wrapper を定義 |
| test262 realm / `$262.evalScript` と混同 | coverage 分析が濁る | eval language feature と harness/realm gap を分類する |

## 12. 推奨実装順序

最短で成果が出て、かつ壊れにくい順序は以下である。

1. Phase 0: tracking / docs / diagnostic inventory。
2. Phase 1: static direct eval を expression-level に昇格。
3. Phase 2: static indirect eval を global AOT で通す。
4. Phase 3: literal-only `new Function` を static generated function として通す。
5. Phase 4: dynamic indirect eval を `host.eval.indirect` で capability-gated 実装。
6. Phase 5: dynamic `Function` / `new Function` を host function object で実装。
7. Phase 6: dynamic direct eval の env descriptor / write-back。
8. Phase 7: test262 ramp。

最初から dynamic direct eval を実装しようとすると、scope, TDZ, closure, var hoist, function hoist, exception, completion value が同時に絡む。
そのため、AOT lane と global dynamic lane で coverage を先に伸ばし、最後に caller-scope dynamic direct eval を入れる。

## 13. 直近 PR 分割案

### PR 1: documentation and classification

- この document を追加。
- language reference / runtime ABI の eval row を更新。
- current rejection points の test names を整理。

### PR 2: parser no-longer-rejects indirect eval

- indirect eval parser diagnostic を外す。
- AST tests を「shape is preserved」に変更。
- resolver はまだ diagnostic を出してもよい。

### PR 3: resolved eval IR

- `ResolvedExpr::Eval` を追加。
- direct/indirect/static/dynamic classification unit tests を追加。
- shadowed eval tests を追加。

### PR 4: static direct eval expression completion

- `EvalFragment` lowering。
- completion value slot。
- static direct eval fixtures。

### PR 5: static indirect eval

- global scope eval fragment。
- fixtures and host-deny no-import assertions。

### PR 6: static Function constructor

- literal args parse。
- generated function registration。
- function object metadata。

### PR 7: dynamic indirect eval host catalog

- HostImport / Capability / RuntimeFn / manifest。
- Node shim import。
- host-deny test。

### PR 8: dynamic Function constructor host lane

- host compile/call/construct imports。
- host function handle wrapper。
- fixtures。

### PR 9+: dynamic direct eval env write-back

- env descriptor。
- mutation ledger。
- local/env-cell write-back。
- test262 ramp。

## 14. 完了時の状態

完了後、次が成り立つ。

- static direct eval / static indirect eval / literal-only `Function` constructor は wasm-native に実行される。
- runtime-generated indirect eval / Function constructor は Node host capability として明示される。
- dynamic direct eval は env descriptor によって caller scope mutation を検証可能な形で扱う。
- `UnsupportedEval` は実装可能な eval case では減り、残る場合も理由が分類されている。
- `docs/01-project-definition.md` の「JS engine wrapper 禁止」を破らない。
