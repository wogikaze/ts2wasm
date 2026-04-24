# Coding Standard

このドキュメントは ts2wasm のコード規約を定める。compiler path のすべての変更はこの規約に従う。

この規約の目的は、Rust の表面上の style を揃えることではない。TypeScript / JavaScript semantics、name resolution、builtin API、runtime ABI、WASM backend、host capability、data layout が混ざって再び emitter に沈殿することを防ぐことである。

## 0. 絶対原則

以下の原則に反する PR はマージしない。

```text
1. source 起因の問題は Diagnostic にする。
2. compiler bug は InvariantViolation または bug! で明示する。
3. Parser は構文だけを読む。
4. Resolver は名前だけを解決する。
5. BuiltinResolver は builtin API だけを解決する。
6. Lowering は解決済み表現を Lowered IR に落とす。
7. Backend は validate 済み Lowered IR と RuntimeLinkPlan だけを受け取る。
8. Runtime / host import / capability / runtime string は catalog と linker で決める。
9. raw WAT を増やさない。
10. 機能追加は docs / validation / tests / differential を同時に更新する。
```

## 1. これまでの過ちと再発防止策

### 1.1 emitter がすべてを決めていた

過去の問題:

```text
JS semantics
runtime ABI
WASM 命令列
host import
memory layout
runtime helper の選択
```

これらが emitter 内で同時に決まっていた。

禁止:

```text
backend 内で名前解決する
backend 内で builtin 判定する
backend 内で arity check する
backend 内で capability を直接決める
backend 内で runtime dependency を手書きする
```

必須:

```text
NameResolver
BuiltinResolver
Lowering
validate_lowered
RuntimeLinkPlan
Backend
```

の順に通す。

### 1.2 WAT 文字列直書きで backend が壊れやすかった

過去の問題:

```text
wat.push_str / format! で命令を生成
raw WAT template の括弧ミス
stack discipline が人力
runtime_builder が巨大文字列化
```

禁止:

```rust
wat.push_str("(local.get 0)\n");
wat.push_str(&format!("(i32.const {})\n", value));
```

移行期間の例外:

```text
legacy WAT backend の既存関数を触る場合のみ許可。
ただし新規 runtime helper を巨大 raw string として追加してはいけない。
関数単位に分割し、linker / WAT / differential test を追加する。
```

目標:

```text
typed WAT writer
または wasm-encoder
```

### 1.3 console.log を parser special form にした

過去の問題:

```text
Stmt::ConsoleLog
console / log keyword 化
console.log だけ parser で特別扱い
```

禁止:

```text
Parser で console.log を特別扱いする
新しい API を parser special form として追加する
Resolver::as_console_log_call のような ad hoc 判定を増やす
```

正しい流れ:

```text
source:
  console.log(x)

Parser:
  Expr::Call {
    callee: Expr::Member { object: Ident("console"), property: "log" },
    args: [x]
  }

BuiltinResolver:
  BuiltinCall(ConsoleLog, [x])

Lowering:
  FunctionCallKind::Builtin(BuiltinId::ConsoleLog)

RuntimeLinkPlan:
  RuntimeFn::Log
  RuntimeFn::Write
  RuntimeFn::ValueToStringInto
  RuntimeFn::Copy
  HostImport::FdWrite
  Capability::StdoutWrite
```

### 1.4 RuntimeFn がただの symbol table だった

過去の問題:

```text
RuntimeFn::symbol() だけがあり、deps/imports/capabilities/runtime_strings/result がなかった
```

必須:

```rust
pub struct RuntimeSpec {
    pub symbol: &'static str,
    pub deps: &'static [RuntimeFn],
    pub imports: &'static [HostImport],
    pub capabilities: &'static [Capability],
    pub runtime_strings: &'static [&'static str],
    pub result: RuntimeResult,
}
```

runtime helper を追加する PR は必ず次を更新する。

```text
RuntimeFn variant
RuntimeSpec
emission_order
all
runtime_builder emit function
linker structure test
differential test if behavior changes
```

### 1.5 runtime 関数は trim したが runtime strings は常時入っていた

過去の問題:

```text
console.log なしでも undefined/null/true/false/newline が data segment に入る
```

禁止:

```text
RuntimeString を WatEmitter::new で無条件 intern する
```

必須:

```text
required RuntimeFn を収集
RuntimeFn::spec().runtime_strings を集約
必要な runtime string だけ intern
user string literal とは区別可能にする
```

### 1.6 fd_write を常時 import していた

過去の問題:

```text
console.log を使わない program でも fd_write import が入る
```

禁止:

```text
backend が fd_write import 文字列を無条件で出す
```

必須:

```text
RuntimeFn::spec().imports
→ RuntimeLinkPlan.required_imports
→ import emission
```

### 1.7 capability を集約しても manifest になっていなかった

過去の問題:

```text
Capability::StdoutWrite は型としてあるが、監査可能な出力がない
```

必須:

```text
required_capabilities() を RuntimeLinkPlan から取得できる
capability manifest として JSON 出力できる設計にする
```

manifest 例:

```json
{
  "imports": ["wasi_snapshot_preview1.fd_write"],
  "capabilities": ["stdout.write"],
  "runtime": ["Log", "Write", "ValueToStringInto", "Copy"]
}
```

### 1.8 Span を導入したが AST に保持しなかった

過去の問題:

```text
SpannedToken はあるが Expr / Stmt が span を持たない
lowering / validation diagnostic が span: None になる
```

禁止:

```text
新規 AST / HIR node を span なしで追加する
source 起因 Diagnostic に span: None を返す
```

必須:

```text
Token
Stmt
Expr
HIR node
BuiltinCall
```

は source span を持つ。

### 1.9 M0 small-int を JS number のように扱った

過去の問題:

```text
ValueTag は small-int なのに JavaScript number として説明した
number range check がなかった
多桁/負数 stringify が壊れていた
```

必須:

```text
ValueTag::can_encode_number
NumberOutOfRange diagnostic
Node differential test
M0 small-int 制約の docs 記載
```

M0 数値表現:

```text
対応:
  tagged small-int
  decimal stringify for integer
  negative integer stringify

非対応:
  f64
  NaN
  Infinity
  -Infinity
  BigInt
  fractional number
```

### 1.10 tests が実行結果に偏り、構造を見ていなかった

過去の問題:

```text
Node vs iwasm の stdout は見るが、runtime linker の中身を見ない
```

必須テスト:

```text
parse snapshot
semantic / builtin resolution snapshot
lowered IR snapshot
linker structure test
WAT/import snapshot or wasm validation
Node vs wasm/iwasm differential
```

runtime linker を変えた PR は必ず linker structure test を追加する。

## 2. Panic / unwrap / expect 禁止

compiler path で `panic!`, `unwrap()`, `expect()` を使わない。

対象:

```text
Lexer
Parser
AST validation
NameResolver
BuiltinResolver
Lowering
IR validation
RuntimeLinkPlan
Backend
```

禁止:

```rust
let x = map.get(k).unwrap();
let x = map.get(k).expect("must exist");
panic!("unreachable");
```

許可:

```rust
bug!("internal invariant violated: {:?}", value);
```

`bug!` は入力起因ではない compiler bug のみ。

## 3. Diagnostic ポリシー

compiler phase は `Result<T, Diagnostic>` または `Result<T, Vec<Diagnostic>>` を返す。

禁止:

```rust
Result<T, String>
anyhow::Result<T> in compiler path
panic as error handling
```

Diagnostic の標準形:

```rust
pub struct Diagnostic {
    pub span: Span,
    pub code: DiagCode,
    pub message: String,
    pub notes: Vec<String>,
}
```

`Option<Span>` は移行期間だけ許可。新規 diagnostic で `None` を増やさない。

必須 DiagCode:

```text
UnsupportedSyntax
UnresolvedName
UnresolvedFunction
DuplicateLocal
DuplicateParameter
DuplicateFunction
ArityMismatch
InvalidTopLevelReturn
NumberOutOfRange
InvariantViolation
BackendIo
```

## 4. Span ポリシー

すべての source-derived node は span を持つ。

```rust
pub struct Span {
    pub start: u32,
    pub end: u32,
}
```

synthetic node は generated span を持つ。

```rust
Span::generated("lowered implicit undefined return")
```

必ず span を持つ diagnostic:

```text
UnsupportedSyntax
UnresolvedName
UnresolvedFunction
DuplicateLocal
DuplicateParameter
DuplicateFunction
ArityMismatch
InvalidTopLevelReturn
NumberOutOfRange
```

## 5. Phase Separation

各 phase の責務を固定する。

```text
Lexer:
  tokenization only

Parser:
  syntax only
  no builtin judgment
  no host/API judgment

AST Validator:
  syntax-level restrictions
  top-level return
  duplicate declarations where syntax/scope-level obvious

NameResolver:
  lexical scope
  function declarations
  local bindings

BuiltinResolver:
  console.log / Math.* / process.* / fs.*
  arity contract
  result contract
  capability contract

Lowering:
  resolved representation → Lowered IR
  no host import strings
  no WAT symbols

validate_lowered:
  structural invariants
  value/effect context
  arity
  local/function id consistency

RuntimeLinkPlan:
  runtime deps/imports/capabilities/runtime strings

Backend:
  encode validated program
  no name resolution
  no builtin discovery
```

## 6. AST / HIR / Lowered IR 更新規約

IR variant 追加時は以下を同時に更新する。

```text
validator
debug/snapshot printer
parse snapshot
semantic snapshot
lowered snapshot
linker structure test if runtime is affected
differential test if behavior is affected
docs/12-current-implementation-status.md
unsupported diagnostic test
```

これが揃わない PR はマージしない。

## 7. validate_lowered 必須検査

`validate_lowered` は backend 前の最後の防壁である。

必須検査:

```text
FuncId in range
function.id == program.functions[index]
params are contiguous LocalId starting from 0
locals are contiguous LocalId starting after params
top_level_locals are contiguous LocalId starting from 0
LocalId in each expression is in scope range
user function call arity
builtin call arity
builtin value/effect context
number literal encodable by ValueTag
Return appears only in function context
```

backend はこれらを再チェックしない。

## 8. RuntimeFn Catalog

runtime helper は必ず catalog に登録する。

```rust
pub enum RuntimeFn { ... }

pub struct RuntimeSpec {
    pub symbol: &'static str,
    pub deps: &'static [RuntimeFn],
    pub imports: &'static [HostImport],
    pub capabilities: &'static [Capability],
    pub runtime_strings: &'static [&'static str],
    pub result: RuntimeResult,
}
```

RuntimeFn 追加時の必須更新:

```text
spec
emission_order
all
emit function
linker test
differential test if semantic behavior changes
```

禁止:

```text
emitter が "$add" などの runtime symbol を直書きする
runtime helper を catalog 外で emit する
RuntimeFn::all() が emission_order() を返す
```

`all()` と `emission_order()` は意図的に独立させる。

```rust
// Intentionally independent from emission_order().
// Tests use this as the enum inventory.
pub const fn all() -> &'static [RuntimeFn] { ... }
```

## 9. RuntimeLinkPlan

`WatEmitter` が直接 linker にならない。RuntimeLinkPlan を独立させる。

```rust
pub struct RuntimeLinkPlan {
    pub required_runtime: BTreeSet<RuntimeFn>,
    pub required_imports: BTreeSet<HostImport>,
    pub required_capabilities: BTreeSet<Capability>,
    pub required_runtime_strings: BTreeSet<&'static str>,
}
```

生成手順:

```text
LoweredProgram scan
→ direct RuntimeFn requirements
→ dependency closure
→ imports aggregation
→ capabilities aggregation
→ runtime strings aggregation
```

WatEmitter は RuntimeLinkPlan を受け取って emit するだけにする。

## 10. Host Import / Capability Manifest

host import は RuntimeLinkPlan から生成する。

禁止:

```rust
wat.push_str("(import \"wasi_snapshot_preview1\" \"fd_write\" ...)");
```

許可:

```rust
for import in plan.required_imports() {
    module.import(import);
}
```

capability は manifest として出力できること。

```json
{
  "imports": ["wasi_snapshot_preview1.fd_write"],
  "capabilities": ["stdout.write"],
  "runtime": ["Log", "Write", "ValueToStringInto", "Copy"]
}
```

## 11. Runtime String / Data Layout

runtime strings は required RuntimeFn によってのみ intern する。

禁止:

```text
UNDEFINED / NULL / TRUE / FALSE / NEWLINE を無条件 intern
```

user string と runtime string は origin を区別可能にする。

```rust
enum StringOrigin {
    UserLiteral,
    Runtime(RuntimeFn),
}
```

memory layout validation は backend emission 前に必須。

```text
static data end <= SCRATCH_OFFSET
SCRATCH_OFFSET + SCRATCH_SIZE <= HEAP_START
SCRATCH_OFFSET < HEAP_START
heap start alignment
string data alignment
```

## 12. Builtin API 規約

builtin 追加時は以下を同時に定義する。

```text
BuiltinId
source pattern in BuiltinResolver
arity
result: Value or EffectOnly
capability requirement
RuntimeFn mapping if needed
unsupported diagnostic if partial
linker structure test
differential test
```

禁止:

```text
Parser に special form を追加
NameResolver に builtin 判定を追加
Backend に builtin 判定を追加
```

## 13. Value Representation

value representation は `runtime/value.rs` に閉じ込める。

禁止:

```rust
(value << 3) | 4
v & 0b111
v & !0b111
```

推奨:

```rust
ValueTag::encode_number(value)
ValueTag::can_encode_number(value)
ValueTag::is_string(value)
```

backend / runtime builder が layout constants を直接使う場合は、`Layout` / `ValueTag` 経由に限定する。

## 14. Feature Gate / Compatibility Level

機能追加は compatibility level を持つ。

```text
M0: single-file JS subset, small-int, function, let, if, while, console.log
M1: builtin resolver separation, richer API subset
M2: structured data subset
M3: broader JS semantics tests
```

unsupported case は silent fallback しない。

```rust
return Err(Diagnostic {
    span,
    code: DiagCode::UnsupportedSyntax,
    message: "object literal is not supported in M0".to_owned(),
    notes: vec!["planned for M2".to_owned()],
});
```

## 15. Test Policy

変更種別ごとの必須テスト:

```text
Lexer / Parser:
  parse snapshot
  unsupported syntax diagnostic

Resolver / BuiltinResolver:
  semantic snapshot
  unresolved / duplicate / arity diagnostic

Lowering:
  lowered IR snapshot
  validate_lowered negative test

RuntimeFn / RuntimeLinkPlan:
  required RuntimeFn test
  required imports test
  required capabilities test
  required runtime strings test
  emission_order/all consistency test

Runtime semantics:
  Node vs wasm/iwasm differential

Backend emission:
  wasm validation
  WAT/import snapshot during legacy period
```

必須 linker tests:

```text
console.log なし → fd_write import なし
console.log あり → fd_write import あり
runtime 不要 → runtime strings なし
Add with string → Add + IsString + Concat + ValueToStringInto + Copy
StrictEqual → StrictEqual + IsString + StringEqual
If / While → TruthyBool
```

## 16. Documentation Update Policy

コード変更と docs 更新を分離しない。

以下を変更した場合は docs 更新必須。

```text
syntax
semantics
runtime ABI
host capability
compatibility level
test policy
unsupported feature set
value representation
memory layout
```

最低限更新対象:

```text
docs/05-compatibility-and-semantics.md
docs/06-testing-and-coverage.md
docs/09-security-and-capability-model.md
docs/11-shared-definitions.md
docs/12-current-implementation-status.md
```

## 17. Commit / Review Policy

commit は論理単位で分ける。

良い例:

```text
backend: add runtime dependency linker
backend: test runtime linker contracts
ir: validate builtin and function layout invariants
runtime: fix integer stringification
```

悪い例:

```text
misc fixes
update compiler
big refactor
```

各 commit 前に必須:

```bash
cargo fmt --all --check
cargo test
```

runtime / backend / differential に関わる変更では、Node vs wasm/iwasm differential も通す。

## 18. Review Checklist

PR reviewer は以下を確認する。

```text
panic/unwrap/expect が compiler path にないか
String error が compiler path にないか
source diagnostic に span があるか
parser が API/builtin を特別扱いしていないか
Resolver に builtin 判定が混ざっていないか
backend が name/builtin/arity を判断していないか
RuntimeFn catalog が更新されているか
RuntimeLinkPlan test があるか
fd_write など host import が条件化されているか
runtime strings が必要時のみ intern されるか
ValueTag の範囲検査があるか
docs/12-current-implementation-status.md が更新されているか
```

## 19. 現在の優先順位

次の順で負債を潰す。

```text
P0:
  RuntimeLinkPlan を WatEmitter から分離
  capability manifest 出力
  AST node span
  BuiltinResolver pass 分離

P1:
  typed WAT writer
  raw WAT runtime_builder の段階的置換
  user/runtime string origin 管理
  linker snapshot fixture 化

P2:
  object / array / module
```

object / array / module は、少なくとも BuiltinResolver と AST span が入るまで着手しない。
