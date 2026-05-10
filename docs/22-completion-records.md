# Completion Records and Control Flow (W5)

## 1. 目的

本設計は、ts2wasm における JavaScript 制御フローの内部表現を、ECMAScript の Completion Record に基づいて統一することを目的とする。

現在の ts2wasm は例外状態を主に `$exception_pending` で管理している。しかし JavaScript の文実行結果は、単なる値ではなく、正常終了、`return`、`throw`、`break`、`continue` のいずれで完了したかを含む。これを明示的に扱わないと、以下の構文の正確な Lowering が困難になる。

* `try...catch...finally`
* `return` を含む `finally`
* ラベル付き `break` / `continue`
* `async` / `await`
* generator / async generator
* 将来的な `eval` / script-level completion value

ECMAScript 仕様上、Completion Record は `[[Type]]`, `[[Value]]`, `[[Target]]` を持ち、`break`, `continue`, `return`, `throw` などの非局所的な制御移動を説明するために使われる。ts2wasm ではこれをコンパイラ内部および WASM Lowering の不変条件として採用する。([TC39][1])

---

## 2. Completion Record モデル

### 2.1 論理モデル

ts2wasm 内部では、Completion Record を以下の論理構造として扱う。

| フィールド        |      内部名 |               型 | 意味                          |
| ------------ | -------: | --------------: | --------------------------- |
| `[[Type]]`   | `status` |           `i32` | 完了種別                        |
| `[[Value]]`  |  `value` | `jsval` / `i64` | 完了値。`empty` を表せる必要がある       |
| `[[Target]]` | `target` |           `i32` | ラベル ID。対象なしは `TARGET_EMPTY` |

### 2.2 Status Code

| Status     |   値 | 意味                              |
| ---------- | --: | ------------------------------- |
| `Normal`   | `0` | 通常完了                            |
| `Return`   | `1` | `return` による完了                  |
| `Throw`    | `2` | `throw` または reject resume による完了 |
| `Break`    | `3` | `break` による完了                   |
| `Continue` | `4` | `continue` による完了                |

`status != Normal` を **abrupt completion** と呼ぶ。

### 2.3 `empty` の扱い

`empty` は JavaScript の `undefined` とは別物である。

そのため、`Completion Value` には以下のいずれかの実装が必要である。

推奨案:

```text
value: jsval
JSVAL_EMPTY: ユーザー JS 値として観測不能な専用 sentinel
```

代替案:

```text
value: jsval
value_is_empty: i32
```

現行案のように `status: i32`, `value: i64`, `target: i32` の 3 フィールドだけで表す場合は、`i64` の `jsval` 空間に `JSVAL_EMPTY` を予約することを必須条件とする。

`empty` を `undefined` と同一視してはならない。理由は、仕様上 `UpdateEmpty(completion, undefined)` のように、`empty` の場合だけ後から値を補う処理が存在するためである。([TC39][1])

---

## 3. 基本ヘルパー

ts2wasm の IR / Lowering では、以下の抽象操作を持つ。

```ts
type CompletionStatus =
  | Normal
  | Return
  | Throw
  | Break
  | Continue;

type Completion = {
  status: CompletionStatus;
  value: JsVal | Empty;
  target: LabelId | TargetEmpty;
};
```

### 3.1 Constructor

```ts
NormalCompletion(value = Empty):
  return { status: Normal, value, target: TargetEmpty }

ReturnCompletion(value):
  return { status: Return, value, target: TargetEmpty }

ThrowCompletion(value):
  return { status: Throw, value, target: TargetEmpty }

BreakCompletion(target = TargetEmpty):
  return { status: Break, value: Empty, target }

ContinueCompletion(target = TargetEmpty):
  return { status: Continue, value: Empty, target }
```

### 3.2 UpdateEmpty

```ts
UpdateEmpty(cr, defaultValue):
  if cr.value != Empty:
    return cr

  return {
    status: cr.status,
    value: defaultValue,
    target: cr.target
  }
```

`Return` と `Throw` の `value` は原則として non-empty である。ただし Lowering の途中で invariant が壊れた場合を検出するため、debug build では `Return` / `Throw` の `value == Empty` を assert する。

---

## 4. Lowering の基本不変条件

### 4.1 文の Lowering

すべての statement lowering は、論理的に `Completion` を返す。

```ts
lowerStmt(stmt) -> Completion
```

WASM 実装上は必ずしも構造体を実体化しなくてよい。高速化のため、以下のようにローカル変数で表す。

```wat
(local $cr_status i32)
(local $cr_value  i64)
(local $cr_target i32)
```

ただし、観測可能な意味は常に Completion Record と等価でなければならない。

### 4.2 abrupt completion の伝搬

`status != Normal` の completion は、以下の構文に到達するまで上位へ伝搬する。

| Completion | 消費できる構文                                                     |
| ---------- | ----------------------------------------------------------- |
| `Return`   | 関数境界、`finally` による override                                 |
| `Throw`    | `catch`、関数境界、async promise rejection、`finally` による override |
| `Break`    | 対象の `switch` / loop / labelled statement                    |
| `Continue` | 対象の loop                                                    |
| 任意の abrupt | `finally` による保存・復元・override                                 |

### 4.3 関数境界

通常の JS 関数では、関数本体の completion を以下のように変換する。

```ts
FunctionEpilogue(cr):
  switch cr.status:
    case Normal:
      return undefined

    case Return:
      return cr.value

    case Throw:
      raise_or_propagate_exception(cr.value)

    case Break:
    case Continue:
      unreachable_or_compile_error
```

`break` / `continue` が関数境界を越えることは構文上許されないため、ここに到達した場合はコンパイラまたは parser のバグである。

### 4.4 WASM の `return` との違い

JS の `return` を、常に WASM の `return` に直接 Lowering してはならない。

特に以下は誤りである。

```ts
try {
  return 1;
} finally {
  cleanup();
}
```

`return 1` の時点で WASM 関数から脱出してしまうと、`finally` が実行されない。したがって、アクティブな `finally` がある範囲では、JS `return` は以下の Completion に Lowering する。

```ts
cr = ReturnCompletion(value)
propagate_to_cleanup_or_epilogue()
```

WASM の直接 `return` は、アクティブな cleanup / `finally` が存在しないことが静的に分かる場合の最適化としてのみ許可する。

---

## 5. `try...catch...finally`

### 5.1 仕様対応

ECMAScript では、`try...finally` はまず `try` block を評価し、次に `finally` を評価する。`finally` が normal completion なら `try` 側の completion を採用し、`finally` が abrupt completion なら `finally` 側を採用する。`try...catch...finally` でも同様に、`catch` 後の completion と `finally` の completion を比較する。([TC39][2])

### 5.2 Lowering 規則

```ts
lowerTryCatchFinally(tryBlock, catchClause?, finallyBlock?):
  blockResult = lowerBlock(tryBlock)

  if catchClause exists and blockResult.status == Throw:
    catchResult = lowerCatch(catchClause, blockResult.value)
  else:
    catchResult = blockResult

  if finallyBlock exists:
    saved = catchResult
    finallyResult = lowerBlock(finallyBlock)

    if finallyResult.status == Normal:
      result = saved
    else:
      result = finallyResult
  else:
    result = catchResult

  return UpdateEmpty(result, undefined)
```

重要な点は、`finally` が normal completion で終わった場合、`finally` 内の通常の completion value は採用しないことである。

### 5.3 例

```ts
function f() {
  try {
    return 1;
  } finally {
    return 2;
  }
}
```

結果:

```ts
2
```

`finally` が `ReturnCompletion(2)` になるため、`try` 側の `ReturnCompletion(1)` を上書きする。

```ts
function g() {
  try {
    return 1;
  } finally {
    2;
  }
}
```

結果:

```ts
1
```

`finally` は normal completion なので、`try` 側の `ReturnCompletion(1)` が復元される。

```ts
function h() {
  try {
    throw 1;
  } finally {
    return 2;
  }
}
```

結果:

```ts
2
```

`finally` の `ReturnCompletion(2)` が、先行する `ThrowCompletion(1)` を上書きする。

---

## 6. ラベル付き `break` / `continue`

### 6.1 Label ID

ECMAScript 仕様では `[[Target]]` はラベル文字列または `empty` である。ts2wasm では、コンパイル時にラベル名を関数内 `LabelId` に変換する。ラベル名自体は実行時に観測されないため、WASM 内部では整数 ID で十分である。ラベル付き文は入れ子になり得て、`break` / `continue` の制御対象として使われる。([TC39][2])

```ts
const TARGET_EMPTY = 0;
type LabelId = i32; // 1 以上
```

### 6.2 Lowering

```ts
break;
```

```ts
cr = {
  status: Break,
  value: Empty,
  target: TARGET_EMPTY
}
```

```ts
break outer;
```

```ts
cr = {
  status: Break,
  value: Empty,
  target: LabelId("outer")
}
```

```ts
continue;
```

```ts
cr = {
  status: Continue,
  value: Empty,
  target: TARGET_EMPTY
}
```

```ts
continue outer;
```

```ts
cr = {
  status: Continue,
  value: Empty,
  target: LabelId("outer")
}
```

### 6.3 消費規則

| 構文                 | 消費する completion                            |
| ------------------ | ------------------------------------------ |
| `switch`           | `Break` with `target == TARGET_EMPTY`      |
| loop               | `Break` with `target == TARGET_EMPTY`      |
| loop               | `Continue` with `target == TARGET_EMPTY`   |
| labelled loop      | `Continue` with `target` in loop label set |
| labelled statement | `Break` with `target == ownLabelId`        |

### 6.4 labelled statement

```ts
lowerLabelledStatement(labelId, body):
  cr = lowerStmt(body)

  if cr.status == Break and cr.target == labelId:
    return NormalCompletion(cr.value)

  return cr
```

注意: labelled statement は `break label` を消費するが、`continue label` は loop 側で処理する。`continue label` の target は、対象ラベルが loop を指している場合にのみ構文上有効である。

### 6.5 loop

loop の概略 Lowering は以下。

```ts
lowerLoop(loop, labelSet):
  while true:
    cr = lowerLoopBody(loop.body)

    if cr.status == Normal:
      continue_or_finish_normally()

    if cr.status == Continue:
      if cr.target == TARGET_EMPTY or cr.target in labelSet:
        continue
      else:
        return cr

    if cr.status == Break:
      if cr.target == TARGET_EMPTY:
        return NormalCompletion(
          cr.value == Empty ? undefined : cr.value
        )
      else:
        return cr

    return cr // Return / Throw
```

### 6.6 直接 WASM `br` の最適化

WASM の `br` を使った直接ジャンプは許可するが、以下の条件を満たす場合に限る。

* ジャンプ先が現在の WASM 構造内に静的に届く
* ジャンプ経路上に未実行の `finally` / cleanup が存在しない
* `break` / `continue` の target が曖昧でない
* async suspension を跨がない

これらを満たさない場合は Completion Record として伝搬する。

---

## 7. `async` / `await`

### 7.1 基本方針

`await` は単なる値待ちではなく、現在の async execution context を中断し、promise の fulfil / reject に応じて Completion Record で再開する操作として扱う。仕様上、fulfil 時は `NormalCompletion(value)`、reject 時は `ThrowCompletion(reason)` で async context を再開する。async 関数の完了時には、normal completion は `undefined` resolve、return completion はその値で resolve、throw completion は reject に対応する。([TC39][3])

### 7.2 保存対象

`await` による中断時、以下を async frame に保存する。

```ts
type AsyncFrame = {
  pc: ContinuationId;
  locals: LocalSnapshot;
  operandStack: ValueSnapshot;

  cr_status: i32;
  cr_value: jsval;
  cr_target: i32;

  activeFinallyStack: FinallyFrame[];
  lexicalEnvState?: EnvSnapshot;
};
```

特に `activeFinallyStack` が重要である。`await` の前後で `try...finally` の制御状態が失われると、以下のようなコードが壊れる。

```ts
async function f() {
  try {
    await p;
    return 1;
  } finally {
    cleanup();
  }
}
```

### 7.3 Await Lowering

概念的には以下。

```ts
lowerAwait(expr):
  valueResult = lowerExpr(expr)

  if valueResult.status != Normal:
    return valueResult

  promise = PromiseResolve(valueResult.value)

  suspend current async frame:
    pc = afterAwait
    save locals
    save active finally stack
    save current completion state

  onFulfilled(v):
    resume frame with NormalCompletion(v)

  onRejected(reason):
    resume frame with ThrowCompletion(reason)
```

再開後は、`await` 式の結果として `NormalCompletion(v)` または `ThrowCompletion(reason)` が通常の statement lowering に渡される。したがって、reject された `await` は `throw` と同じ経路で `catch` / `finally` に伝搬する。

### 7.4 async 関数境界

```ts
AsyncFunctionEpilogue(cr):
  switch cr.status:
    case Normal:
      PromiseResolve(undefined)

    case Return:
      PromiseResolve(cr.value)

    case Throw:
      PromiseReject(cr.value)

    case Break:
    case Continue:
      unreachable_or_compile_error
```

---

## 8. 既存 `$exception_pending` との統合

### 8.1 移行方針

現行の `$exception_pending` は、Phase 1 では互換レイヤーとして残してよい。ただし、最終的な制御状態の source of truth は Completion Record に統一する。

暫定実装:

```ts
if $exception_pending:
  cr.status = Throw
  cr.value = $exception_value
  cr.target = TARGET_EMPTY
```

最終実装:

```ts
$exception_pending は廃止、または cr.status == Throw から派生
```

### 8.2 二重管理の禁止

以下のような状態は不正とする。

```ts
cr.status == Normal && $exception_pending == true
cr.status == Throw  && $exception_pending == false
```

debug build では、関数境界・call 境界・catch 境界で invariant check を入れる。

---

## 9. 実装フェーズ

### Phase 0: 基盤定義

* `CompletionStatus` を定義する
* `TARGET_EMPTY` と `LabelId` を定義する
* `JSVAL_EMPTY` または `value_is_empty` を導入する
* `NormalCompletion`, `ReturnCompletion`, `ThrowCompletion`, `BreakCompletion`, `ContinueCompletion`, `UpdateEmpty` を IR レベルに導入する
* parser / semantic analysis で label target を解決する

### Phase 1: `return` / `throw` の Completion 化

* JS `return` を `ReturnCompletion(value)` に Lowering する
* JS `throw` を `ThrowCompletion(value)` に Lowering する
* 関数 epilogue で `Return` / `Throw` を処理する
* `$exception_pending` ととの bridge を実装する
* アクティブな `finally` がない場合のみ、直接 WASM `return` 最適化を許可する

### Phase 2: `try...catch...finally`

* `try` / `catch` / `finally` の lowering を Completion ベースに変更する
* `finally` 前に completion を保存する
* `finally` が normal なら保存済み completion を復元する
* `finally` が abrupt なら保存済み completion を上書きする
* `UpdateEmpty(result, undefined)` を正しく適用する

### Phase 3: ラベル付き `break` / `continue`

* `break label` / `continue label` を `target: LabelId` 付き Completion にする
* loop / switch / labelled statement で completion consumption を実装する
* 3 階層以上のネストで propagation を検証する
* cleanup がない単純ケースでは WASM `br` 最適化を許可する

### Phase 4: `async` / `await`

* async frame に `cr_status`, `cr_value`, `cr_target` を保存する
* active `finally` stack を保存・復元する
* await fulfil を `NormalCompletion(value)` として再開する
* await reject を `ThrowCompletion(reason)` として再開する
* async function epilogue で `Return` -> resolve、`Throw` -> reject に変換する

### Phase 5: 最適化

* `status == Normal` の fast path
* `target == TARGET_EMPTY` の fast path
* `try/finally` が存在しない関数での直接 `return`
* cleanup がない loop での直接 WASM `br`
* Completion Record の materialization elimination

---

## 10. 成功条件

### Gate W5-H1: `finally` による戻り値の上書き

```ts
function f() {
  try {
    return 1;
  } finally {
    return 2;
  }
}
```

期待値:

```ts
f() === 2
```

追加確認:

```ts
function g() {
  try {
    return 1;
  } finally {
    2;
  }
}
```

期待値:

```ts
g() === 1
```

### Gate W5-H2: 3 階層以上のラベル付き `break`

```ts
function f() {
  let n = 0;

  outer:
  for (let i = 0; i < 3; i++) {
    for (let j = 0; j < 3; j++) {
      for (let k = 0; k < 3; k++) {
        n++;
        break outer;
      }
    }
  }

  return n;
}
```

期待値:

```ts
f() === 1
```

追加確認:

```ts
function g() {
  let n = 0;

  outer:
  for (let i = 0; i < 3; i++) {
    for (let j = 0; j < 3; j++) {
      n++;
      continue outer;
    }
  }

  return n;
}
```

期待値:

```ts
g() === 3
```

### Gate W5-H3: async 関数内の `await` と `try...finally`

```ts
async function f(log) {
  try {
    log.push("try");
    await Promise.resolve();
    return 1;
  } finally {
    log.push("finally");
  }
}
```

期待値:

```ts
const log = [];
await f(log) === 1;
log.join(",") === "try,finally";
```

追加確認:

```ts
async function g() {
  try {
    return 1;
  } finally {
    await Promise.resolve();
    return 2;
  }
}
```

期待値:

```ts
await g() === 2
```

reject 経路:

```ts
async function h() {
  try {
    await Promise.reject("x");
  } finally {
    return 2;
  }
}
```

期待値:

```ts
await h() === 2
```

finally が normal completion の場合:

```ts
async function i() {
  try {
    await Promise.reject("x");
  } finally {
    await Promise.resolve();
  }
}
```

期待値:

```ts
i() rejects with "x"
```

---

## 11. 非目標

W5 では以下を非目標とする。

* WASM exception-handling proposal への完全依存
* `with` 構文の完全サポート
* `eval` / script-level completion value の完全再現
* generator / async generator の完全実装
* すべての `break` / `continue` を Completion Record 化することによる最適化放棄

ただし、`empty` sentinel と Completion Record の基本構造は、将来的な `eval`、generator、async generator の実装を妨げない形で導入する。

---

## 12. 未解決事項

### 12.1 `JSVAL_EMPTY` の表現

`jsval` のタグ空間に安全な sentinel を予約できるか確認する。予約できない場合は `value_is_empty: i32` を追加する。

推奨決定:

```text
まず JSVAL_EMPTY を試す。
タグ空間に余裕がない場合のみ value_is_empty を導入する。
```

### 12.2 call ABI

compiled JS function call の ABI を以下のどちらにするか決める。

案 A:

```text
return value: i64
throw state: global cr_status / cr_value
```

案 B:

```text
return values: (i64 value, i32 status)
```

案 B の方が Completion Record と整合しやすいが、既存 runtime との接続コストを確認する必要がある。

### 12.3 直接 WASM 制御命令の最適化条件

`return` / `br` を直接使える条件を、以下のような静的フラグで判定する。

```ts
canUseDirectReturn =
  activeFinallyDepth == 0 &&
  !insideAsyncSuspensionRegion

canUseDirectBreak =
  activeFinallyDepth == 0 &&
  targetIsLexicallyReachable &&
  !crossesAsyncSuspensionRegion
```

### 12.4 generator への拡張

generator / async generator では、`yield` / `return` / `throw` による再開要求も Completion Record として扱う設計に拡張できる。W5 では async/await の再開モデルまでを対象とし、generator は次フェーズで扱う。

[1]: https://tc39.es/ecma262/multipage/ecmascript-data-types-and-values.html "ECMAScript® 2027 Language Specification"
[2]: https://tc39.es/ecma262/multipage/ecmascript-language-statements-and-declarations.html "ECMAScript® 2027 Language Specification"
[3]: https://tc39.es/ecma262/multipage/control-abstraction-objects.html "ECMAScript® 2027 Language Specification"
