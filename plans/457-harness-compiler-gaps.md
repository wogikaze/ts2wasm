# Epic: Harness compiler gaps (v2)

## 目的

test262 の公式 harness を inline stub なしで直接読み込み、reference coverage の build / semantic coverage をより正確に計測できるようにする。

同時に tsc / tsgo については harness 非依存の ramp を進め、TypeScript erased syntax 対応範囲を広げる。

## 最終ゴール

1. `scripts/lib/test262_harness.py` から inline stub を削除する。
2. `reference/test262/harness/{sta.js,assert.js}` を常に実ファイルとして読み込む。
3. `mise run reference-coverage test262 --limit 1000 --detail` が inline stub 時と同等以上の `build_pass` になる。
4. `mise run gate` が通る。
5. `I-20260513-HDW7PQ` を close する。
6. tsc / tsgo ramp は別系統で継続し、tsc `build_pass >= 2000`、tsgo `semantic_pass >= 5%` を目指す。

---

# 基本方針

## 1. harness 専用ハックではなく、一般的な JS lowering の改善として入れる

今回の問題は test262 harness で顕在化しているが、実体は以下の一般的な JS パターン。

```js
var assert = {};
assert.sameValue = function(a, b) { ... };
assert.sameValue(1, 1);
```

```js
function assert(x) {
  return assert._toString(x);
}
```

```js
Test262Error.prototype.toString = function() { ... };
```

そのため、compiler 側に `test262` 特化の分岐は入れない。

## 2. Phase 3 は最初に入れるが、Phase 1 / 2 は「Undefined で逃がす」だけにしない

Phase 3 は低リスクなので先に入れる。
ただし Phase 1 / 2 は semantic coverage に直結するため、可能な範囲で以下を目指す。

* 関数名は正しく scope に bind する。
* object property に代入された function は call site から参照できるようにする。
* どうしても値表現が未実装な場合だけ、明示的に fallback する。

## 3. 各 Phase は独立 PR にする

巨大 PR にすると regression 原因が追いづらいので、最低でも以下に分ける。

```text
PR 0: baseline / regression tests
PR 1: Phase 3 prototype property assignment
PR 2: Phase 2 function self-reference
PR 3: Phase 1 var-declared receiver method dispatch
PR 4: Phase 4 real harness loading
PR 5+: Phase 5 tsc/tsgo ramp
```

---

# PR 0: Baseline と最小 regression tests

## 目的

修正前の状態を固定し、以後の PR で「何が改善し、何が壊れたか」を見えるようにする。

## 作業

### 0.1 現在値を保存

以下を実行して、結果を issue / PR description に貼る。

```bash
mise run reference-coverage test262 --limit 1000 --detail
mise run reference-coverage test262 --path-filter language/statements --limit 10 --detail
mise run reference-coverage tsc --limit 1400 --detail
mise run reference-coverage tsgo --limit 166 --detail
cargo nextest run
```

最低限記録する値。

```text
test262:
  build_pass
  semantic_pass
  top error buckets

tsc:
  build_pass
  semantic_pass
  top error buckets

tsgo:
  build_pass
  semantic_pass
  top error buckets
```

### 0.2 最小 fixture を追加

compiler regression 用に以下の最小ケースを追加する。

#### A: var-declared receiver method call

```js
var assert = {};
assert.sameValue = function(a, b) {};
assert.sameValue(1, 1);
```

期待値: build pass（現時点では failure で OK）

#### B: function self-reference

```js
function assert(x) {
  return assert;
}
```

期待値: build pass（現時点では failure で OK）

#### C: function self-reference + property access

```js
function assert(x) {
  return assert._toString(x);
}
assert._toString = function(x) {
  return "";
};
assert(1);
```

期待値: build pass（現時点では failure で OK）

#### D: prototype property assignment

```js
function Test262Error(message) {
  this.message = message;
}
Test262Error.prototype.toString = function() {
  return this.message;
};
```

期待値: build pass（現時点では failure で OK）

## 完了条件

```bash
cargo nextest run
```

が既存状態で通る。
新規 fixture は現時点では failure として記録してよい。以後の Phase で pass に変える。

---

# PR 1: Phase 3 — prototype property assignment

## 対象ファイル

```text
crates/ir/src/lowered/resolver/call/user.rs
```

## 現状

`lower_function_metadata_property` が `"prototype"` key を metadata として扱い、以下で落ちる。

```text
prototype metadata is not supported
```

対象コード例。

```js
Test262Error.prototype.toString = function() {};
```

## 修正方針

`prototype` を metadata error にしない。

重要なのは、`prototype` を「未対応 metadata」として reject しないこと。
最小実装では以下でよい。

```text
function metadata lowering 内で "prototype" を見つけたら error ではなく通常 property assignment 側に逃がす
```

もし既存構造上、通常 assignment 側へ戻せないなら、初期実装では `LoweredExpr::Undefined` に落として build を通す。ただし、この場合は PR description に明示する。

```text
Known limitation:
  prototype object の runtime semantics はまだ表現していない。
  今回の目的は公式 harness の build unblock。
```

## 推奨実装レベル

優先順位は以下。

### Best

`Test262Error.prototype.toString = function(){}` を普通の property assignment として lowering する。

### Acceptable first step

`prototype` metadata error を消し、assignment 全体を no-op / undefined 扱いにして build を通す。

### Avoid

`test262` harness のときだけ特別扱いする。

## 追加テスト

```js
function Test262Error(message) {
  this.message = message;
}
Test262Error.prototype.toString = function() {
  return this.message;
};
```

期待値: build pass

## 回帰確認

```bash
cargo nextest run
mise run reference-coverage test262 --path-filter language/statements --limit 10 --detail
```

## 完了条件

* `prototype metadata is not supported` が対象 fixture で出なくなる。
* 既存 method / user call 系テストが regression しない。
* `cargo nextest run` が通る。

---

# PR 2: Phase 2 — function self-reference in body

## 対象ファイル

第一候補:

```text
crates/ir/src/lowered/resolver/expr/control.rs
```

関連候補:

```text
crates/ir/src/semantic.rs
```

## 現状

以下が落ちる。

```js
function assert(x) {
  return assert;
}
```

エラー:

```text
unresolved name: assert
```

原因は、関数本体 lowering 時に関数自身の名前が local scope に入っていないため。

## 修正方針

元案では `function_ids` に名前があれば `Undefined` 参照で通す案だったが、これは最終手段にする。

より堅い方針は以下。

```text
function declaration の名前を、body lowering 前に scope へ pre-bind する。
```

JS 的にも、function declaration は自身の body 内から参照できる。

```js
function f() {
  return f;
}
```

これは普通に解決可能であるべき。

## 実装案

### 2.1 function declaration 登録のタイミングを確認

現在の lowering / semantic pipeline で、関数名と `function_id` がいつ作られるか確認する。

確認ポイント:

```text
- function_ids は body lowering 前に存在するか
- resolve_local が function binding を見に行けるか
- var / function / parameter の shadowing rules はどうなっているか
```

### 2.2 scope に Function binding を追加

理想形。

```rust
ResolvedLocal::Function(FunctionId)
```

または既存 enum に合わせて、

```rust
LocalBindingKind::Function
```

のような binding kind を追加する。

### 2.3 lower_ident_expr で function binding を解決

`lower_ident_expr` が local variable だけでなく function binding も解決できるようにする。

期待動作:

```js
function assert(x) {
  return assert;
}
```

で `assert` が unresolved にならない。

### 2.4 FunctionRef IR がない場合

関数を値として返す runtime semantics が未実装なら、以下のどちらかにする。

優先:

```text
LoweredExpr::FunctionRef(function_id) を追加
```

難しい場合の暫定:

```text
ResolvedLocal::Function は解決済みにするが、値 expression としては Undefined に lower する
```

ただし、これは明示的に technical debt とする。

```text
TODO:
  Function declaration as first-class value is represented as Undefined for now.
  Name resolution is correct; value semantics are partial.
```

この区別が重要です。
「名前解決できない」のと「関数値の runtime 表現が未実装」は別問題として扱う。

## shadowing ルール

最低限、以下を守る。

```js
function f() {
  var f = 1;
  return f;
}
```

この場合、body 内の `f` は local var を優先する。

```js
function f(f) {
  return f;
}
```

この場合、parameter を優先する。

優先順位:

```text
parameter / local var > inner function declaration > current function self binding > outer scope
```

厳密な JS semantics と完全一致しなくても、少なくとも既存 resolver の shadowing ルールを壊さないこと。

## 追加テスト

### self reference

```js
function assert(x) {
  return assert;
}
```

期待値: build pass

### self reference + property access

```js
function assert(x) {
  return assert._toString(x);
}
assert._toString = function(x) {
  return "";
};
assert(1);
```

期待値: build pass

### shadowing

```js
function f(f) {
  return f;
}
```

期待値: build pass, existing semantics regression なし

## 回帰確認

```bash
cargo nextest run
mise run reference-coverage test262 --path-filter language/statements --limit 10 --detail
```

## 完了条件

* `function assert(x){ return assert; }` が build pass。
* `unresolved name: assert` が対象 fixture で消える。
* 既存 resolver / semantic tests が regression しない。

---

# PR 3: Phase 1 — method call dispatch for var-declared receivers

## 対象 issue

```text
I-20260513-4E2BR9
```

## 対象ファイル

```text
crates/ir/src/lowered/resolver/call/method.rs
```

必要に応じて関連ファイルも触る。

## 現状

以下で落ちる。

```js
var assert = {};
assert.sameValue = function(a, b) {};
assert.sameValue(1, 1);
```

エラー:

```text
unknown receiver class for method sameValue
```

原因は、`assert` が var-declared object であり class/type 情報を持たないため。
`lower_method_call_expr` が Ident receiver を class dispatch として解決しようとして失敗している。

## 修正方針

単純な RuntimeCall fallback だけだと semantic coverage が薄くなる。
そのため、二段構えにする。

```text
1. known property function call は direct user function call に lower する
2. unknown property call は generic dynamic/runtime call に fallback する
```

## 実装方針

### 3.1 Object property function index を導入

以下のような assignment を検出する。

```js
assert.sameValue = function(a, b) {};
```

これを internal table に登録する。

概念的にはこういう index。

```text
(receiver_binding, property_name) -> function_id
```

例:

```text
(assert, "sameValue") -> FunctionId(...)
(assert, "_toString") -> FunctionId(...)
```

dot property だけでなく、可能なら string literal bracket も対応する。

```js
assert["sameValue"] = function(a, b) {};
```

ただし bracket 対応は Phase 1.1 では optional でよい。

### 3.2 method call lowering 時に index を見る

以下の call に対して、

```js
assert.sameValue(1, 1);
```

`assert.sameValue` が index にあるなら、class dispatch ではなく user function call として lower する。

```text
assert.sameValue(1, 1)
  -> call FunctionId(assert.sameValue), args = [1, 1]
```

`this` が必要な関数では不完全だが、test262 assert harness の主要 assertion では多くの場合 `this` に依存しない。
将来対応としては `this` binding を追加する。

### 3.3 unknown receiver class の fallback を限定的に許可する

index で解決できない場合だけ fallback する。

```text
if receiver is typed/class-known:
    existing class dispatch
else if receiver.property is registered property function:
    user function call
else if receiver is untyped object-ish:
    dynamic/runtime method call fallback
else:
    existing error
```

既存 typed method call を壊さないことが最重要。

## 重要な guardrail

以下は絶対に避ける。

```text
unknown receiver class を全部 Undefined にする
```

それをやると本当に壊れている method call まで通ってしまい、coverage の信頼性が落ちる。

fallback は以下に限定する。

```text
- receiver が var-declared object literal
- receiver が function object 参照
- receiver が既に untyped / dynamic と判定されている
```

## 追加テスト

### object property function call

```js
var assert = {};
assert.sameValue = function(a, b) {};
assert.sameValue(1, 1);
```

期待値: build pass

### function object property call

```js
function assert(x) {
  return assert._toString(x);
}
assert._toString = function(x) {
  return "";
};
assert(1);
```

期待値: build pass

### unknown method should not break typed dispatch

既存 method call fixture をすべて維持。

期待値: existing tests pass

### optional: string-literal property

```js
var assert = {};
assert["sameValue"] = function(a, b) {};
assert["sameValue"](1, 1);
```

期待値: Phase 1.1 では optional、Phase 1.2 で build pass を目指す

## 回帰確認

```bash
cargo nextest run
mise run reference-coverage test262 --path-filter language/statements --limit 10 --detail
mise run reference-coverage test262 --limit 1000 --detail
```

## 完了条件

* `var assert = {}; assert.sameValue = function(){}; assert.sameValue(1);` が build pass。
* `unknown receiver class for method X` が公式 harness 起因で出なくなる。
* typed method call の既存テストが全 PASS。
* `I-20260513-4E2BR9` を close できる。

---

# PR 4: Phase 4 — Enable real harness loading

## 対象 issue

```text
I-20260513-HDW7PQ
```

## 対象ファイル

```text
scripts/lib/test262_harness.py
```

## 前提

以下が完了済みであること。

```text
Phase 1: var-declared receiver method call
Phase 2: function self-reference
Phase 3: prototype property assignment
```

## 現状

`INLINE_STA_JS` / `INLINE_ASSERT_JS` で公式 harness を代替している。

問題:

```text
- semantic coverage が公式 harness とズレる
- test262 更新時に stub を手動更新する必要がある
- stub 未対応 assertion で runtime failure が出る
```

## 修正方針

inline stub fallback を削除する。

変更後の原則:

```text
load_harness_file(name):
    path = HARNESS_DIR / name
    read file
    strip frontmatter
    return contents
```

## 作業

### 4.1 `INLINE_HARNESS_STUBS` を削除

削除対象:

```text
INLINE_STA_JS
INLINE_ASSERT_JS
INLINE_HARNESS_STUBS
```

### 4.2 `load_harness_file` を単純化

期待する形。

```python
def load_harness_file(name: str) -> str:
    path = HARNESS_DIR / name
    text = path.read_text()
    return strip_frontmatter(text)
```

### 4.3 missing harness file は明示的に error

fallback しない。

```text
reference/test262/harness/<name> が存在しない場合は failure
```

これは正しい。
存在しない harness を stub で隠すと coverage が不正確になる。

### 4.4 A/B 比較

PR 4 の直前 commit と PR 4 後で以下を比較する。

```bash
mise run reference-coverage test262 --path-filter language/statements --limit 10 --detail
mise run reference-coverage test262 --limit 1000 --detail
```

記録する値。

```text
before:
  inline stub mode
  build_pass
  semantic_pass
  top error buckets

after:
  real harness mode
  build_pass
  semantic_pass
  top error buckets
```

## 完了条件

* inline stub が削除されている。
* test262 harness は常に `reference/test262/harness/` から読む。
* `mise run reference-coverage test262 --path-filter language/statements --limit 10` が inline stub 時と同等以上。
* `mise run reference-coverage test262 --limit 1000 --detail` が inline stub 時と同等以上の `build_pass`。
* `cargo nextest run` が通る。
* `mise run gate` が通る。
* `I-20260513-HDW7PQ` を close。

---

# PR 5+: Phase 5 — tsc / tsgo ramp

## 位置づけ

Phase 5 は test262 harness とは独立。
Phase 1-4 を待たずに並行可能。

## 現状

```text
tsc:
  total: 6537
  build_pass: 668
  limit: 1400
  pass rate: 10.4%

tsgo:
  total: 166
  build_pass: 50
  pass rate: 30.1%
```

## 目的

TypeScript erased syntax の対応範囲を広げ、build pass / semantic pass を増やす。

## 原則

tsc / tsgo では harness を作らない。
生の `.ts` を直接コンパイルし、compiler 側の TS syntax erasure を増やす。

## ramp の進め方

### 5.1 error bucket を固定する

各 ramp の前に以下を実行する。

```bash
mise run reference-coverage tsc --limit 1400 --detail
mise run reference-coverage tsgo --limit 166 --detail
```

top error bucket を記録。

特に見るもの。

```text
UnsupportedTypeScriptSyntax
UnresolvedName
parser error
lowering error
semantic error
panic
```

### 5.2 zero-runtime TS syntax を優先する

まずは runtime に影響しない erased syntax を潰す。

優先度高:

```text
type aliases
interfaces
type-only imports
type-only exports
generic type parameters
type arguments
as assertions
satisfies expressions
non-null assertions
ambient declarations
declare statements
```

これらは JS runtime に残らないので、build_pass 増加に対してリスクが低い。

### 5.3 runtime shape がある TS syntax は別 issue に分ける

以下は erased syntax と同じ PR に混ぜない。

```text
enum
namespace
parameter properties
decorators
const enum
module augmentation
import equals
export equals
```

理由は、runtime semantics や emit shape が絡むため。

### 5.4 ramp limit を段階的に上げる

tsc は一気に 6000 へ上げない。

```text
Step 1: limit 1400 -> 2000
Step 2: limit 2000 -> 3000
Step 3: limit 3000 -> 4500
Step 4: limit 4500 -> 6000+
```

各 step でやること。

```text
1. current pass/fail を測る
2. top 3 error buckets を潰す
3. newly exposed error pattern を issue 化する
4. limit を上げる
5. cargo nextest run / gate を通す
```

## Phase 5 の中間目標

### tsc milestone A

```text
build_pass >= 1000
limit >= 2000
panic = 0
```

### tsc milestone B

```text
build_pass >= 1500
limit >= 3000
UnsupportedTypeScriptSyntax を主要 bucket から落とす
```

### tsc milestone C

```text
build_pass >= 2000
limit >= 6000
```

### tsgo milestone A

```text
build_pass >= 80
semantic_pass >= 5%
```

### tsgo milestone B

```text
build_pass >= 120
semantic_pass >= 5%
UnresolvedName の主要原因を issue 化
```

## 完了条件

```text
tsc build_pass >= 2000
tsc limit >= 6000
tsgo semantic_pass >= 5%
new panic = 0
mise run gate pass
```

---

# 推奨実装順

最終的な順番はこれが堅いです。

```text
1. PR 0: baseline / fixtures
2. PR 1: Phase 3 prototype property assignment
3. PR 2: Phase 2 function self-reference
4. PR 3: Phase 1 property function method dispatch
5. PR 4: real test262 harness loading
6. PR 5+: tsc/tsgo ramp
```

Phase 3 を最初にする理由:

```text
- 最小変更
- 単独で適用可能
- regression リスクが低い
- 公式 harness の blocker を 1 つ減らせる
```

Phase 1 を Phase 2 より後にする理由:

```text
- assert は function object としても property receiver になる
- self-reference 解決ができていないと assert._toString 系の評価が不安定
- receiver dispatch の前に名前解決を固めた方が安全
```

---

# リスク管理

## Risk 1: Undefined fallback が coverage を過大評価する

### 例

```js
assert.sameValue(1, 2);
```

が no-op になって通ると semantic coverage が信用できなくなる。

### 対策

Phase 1 では可能な限り property function call を direct call に lower する。
どうしても dynamic fallback する場合は、対象を unknown receiver に限定し、PR description に fallback 件数を記録する。

---

## Risk 2: function self-reference の解決が shadowing を壊す

### 例

```js
function f(f) {
  return f;
}
```

ここで parameter より function self binding が優先されると壊れる。

### 対策

scope lookup の優先順位を明示する。

```text
parameter / local var > inner declaration > self function binding > outer scope
```

shadowing fixture を追加する。

---

## Risk 3: prototype を no-op にすると error path の意味が落ちる

`Test262Error.prototype.toString` が設定されないと、failure reporting の runtime 表現は不完全になる。

### 対策

PR 1 では build unblock として許容。
ただし、将来 issue を切る。

```text
Future issue:
  represent Function.prototype object and prototype property assignments
```

---

## Risk 4: method call fallback が既存 class dispatch を壊す

### 対策

typed receiver の場合は既存 path を最優先する。

```text
known class receiver:
    existing class dispatch only

untyped / var object receiver:
    property function index
    then dynamic fallback
```

既存 method call tests を regression gate にする。

---

# 各 issue の close 条件

## I-20260513-4E2BR9

Close 可能条件:

```text
- var-declared object receiver の property function call が build pass
- official assert.js 起因の unknown receiver class が消える
- cargo nextest run pass
```

対象: Phase 1 / PR 3

## I-20260513-HDW7PQ / #457

Close 可能条件:

```text
- INLINE_STA_JS / INLINE_ASSERT_JS 削除
- load_harness_file が real file のみ読む
- test262 --limit 1000 で inline stub 時と同等以上の build_pass
- mise run gate pass
```

対象: Phase 4 / PR 4

---

# 最終 acceptance checklist

```text
[ ] PR 0 で baseline metrics と fixture を追加
[ ] Phase 3 fixture が build pass
[ ] Phase 2 fixture が build pass
[ ] Phase 1 fixture が build pass
[ ] official reference/test262/harness/assert.js が直接 build 可能
[ ] official reference/test262/harness/sta.js が直接 build 可能
[ ] INLINE_STA_JS 削除
[ ] INLINE_ASSERT_JS 削除
[ ] INLINE_HARNESS_STUBS 削除
[ ] load_harness_file が real harness のみ読む
[ ] mise run reference-coverage test262 --path-filter language/statements --limit 10 pass
[ ] mise run reference-coverage test262 --limit 1000 --detail が inline stub 時と同等以上
[ ] cargo nextest run pass
[ ] mise run gate pass
[ ] I-20260513-4E2BR9 close
[ ] I-20260513-HDW7PQ close
[ ] tsc ramp issue 群を別 Epic / sub-issues として継続
```
