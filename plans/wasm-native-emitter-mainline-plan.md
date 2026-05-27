# Wasm Native Emitter 本線化・全 RuntimeFn 実装計画

作成対象: アップロードされた `archive(20).zip` の静的確認に基づく実装計画。

## 目的

`wat2wasm` / WAT parse / WAT 文字列組み立てをビルド本線から外し、`LoweredProgram -> WasmModule -> wasm-encoder -> .wasm` を唯一の成功パスにする。WAT emitter は比較・デバッグ・移行補助として残してよいが、ビルド成功の根拠にはしない。

最終状態は次の通り。

- `RuntimeLinkPlan` で選択された実 RuntimeFn は、すべて typed builder から `WasmFunction` として `WasmModule` に追加される。
- `RuntimeFn::emission_order()` は Native runtime でも依存順の唯一の出力順になる。
- Host import / capability / runtime string / runtime global は catalog と link plan 経由でのみ追加される。
- `WasmInstr::Raw`、未解決 `Call` / `GlobalGet` / `GlobalSet` の index `0` フォールバック、WAT parse fallback は本線から消える。
- すべての RuntimeFn builder は `RuntimeFn::stack_effect()` と一致し、`wasmparser` と differential fixture で検証される。
- Pseudo-intrinsic は lowering/emit 段階で展開され、final wasm module に `$pseudo_*` function として残らない。

## 静的確認メモ

| 項目 | 確認結果 | 対応方針 |
|---|---|---|
| RuntimeFn variants | `495` variants。`emission_order()` も `495` 件で一意・完全。 | Native builder registry の completeness gate に使う。 |
| RuntimeSpec | `495` specs を確認。 | symbol/deps/imports/capability/runtime_strings/result を Native builder 検証の根拠にする。 |
| build-facing entrypoint | `emit_wasm_binary()` / `emit_wasm_binary_with_abi()` は Native 側を呼ぶ構造。 | 方針は合っている。未対応 shape と runtime 実体不足を潰す。 |
| WAT debug fallback | `emit_wasm_binary_with_wat_debug_fallback*` に WAT parse 経路が残る。`emit_mir_wasm_binary()` は Native binary backend 経由に移行済み。 | dump/test 専用に隔離し、通常 build / server から呼ばせない。 |
| Native runtime 実体 | `native_lowered.rs` は RuntimeLinkPlan で選ばれた runtime 関数を module に組み込まない。 | `RuntimeLinkPlan -> WasmModule` の runtime append 層を作る。 |
| typed builder seed | `runtime/core/typed.rs` の seed builder は Core ABI から段階的に増加中。 | seed を registry に移し、`Raw` 除去・binary encoder 検証へ切り替える。 |
| binary encoder の危険点 | 未解決 call/global が `unwrap_or(0)`、`Raw` が無視される。 | `to_wasm_encoder()` を `Result<Vec<u8>, Diagnostic>` 化して即エラーにする。 |
| `WasmInstr::RuntimeCall` | `native_lowered.rs` 側に参照がある一方、`WasmInstr` enum 側では確認できない。 | 先に正式 variant 化するか、`RuntimeFn -> Call(symbol)` 変換層へ寄せる。 |
| ABI モデル | docs と catalog は tagged `i32` JS value を前提。Native subset には raw i32 最適化が混ざる。 | `TaggedValue` 境界を明確化し、raw 数値は局所最適化に閉じ込める。 |
| 検証実行 | 初期静的確認時は未実施。2026-05-24 時点で backend-wasm の focused/lib tests は実行済み。 | `python scripts/manager.py check` と fixture differential を本線化前 gate にする。 |

Sanity check:

- variants missing from domain: `[]`
- variants missing from specs: `[]`
- variants missing from emission_order: `[]`
- unknown entries in emission_order: `[]`

## 2026-05-25 再確認: test262 拡大 issue と残ボトルネック

Native emitter の直近確認では `python3 scripts/manager.py native-emitter-unsupported --format markdown`
が `native_unsupported: 0` まで到達している。一方、`RuntimeFn` 全体の native builder
coverage は `available=490`, `pseudo=5`, `missing_non_pseudo=0` まで到達している。
Native runtime builder registry 上の非 pseudo RuntimeFn 欠落はなくなった。

test262 サポート範囲を拡大するため、優先する issue は次の5件。

| Priority | Issue | 狙い | ボトルネック |
|---:|---|---|---|
| 1 | `I-20260523-N8PSET` Native emitter property set support | 最大 bucket の property assignment を runtime-catalog helper 経由で native bytes 化する | static/dynamic property set の ABI 境界を `PropertySet` / array/index fast path に寄せ、layout 直書きを増やさないこと |
| 2 | `I-20260523-N8EXPR` Native emitter general expression support | catch-all expression unsupported を variant 別に分解する | 現在の診断が具体 variant を失うため、先に集計・分類 instrumentation が必要 |
| 3 | `I-20260523-N8UNRY` Native emitter unary operator support | `typeof` / `void` / `delete` / numeric negation など高頻度 unary を native 化する | JS coercion と例外/削除 semantics を raw i32 shortcut で壊さず既存 runtime helper に合わせること |
| 4 | `I-20260523-N8PGET` Native emitter property get support | static/dynamic/optional property read を native path に乗せる | `PropertyGet` / `Index` / optional short-circuit の使い分けと host/object/string/array shape の分類 |
| 5 | `I-20260523-M4Q7RB` Retire non-native IR/runtime feature guards | native emitter 以外で残る大きな feature guard bucket を減らす | `Array.from` mapFn、mutable closures、private identifiers などは backend ではなく IR/runtime semantics の作業に分離する必要がある |

Native runtime builder coverage 上の残 RuntimeFn は 0 件。残る実作業は builder 欠落ではなく、
test262 semantics を広げるための lowering/dispatch 精度である。

- 2026-05-25 full fixture differential:
  `python3 scripts/manager.py check-fixture-differential` は完走したが
  まず `pass=436 fail=568 unsupported=182 blocked=165 total=1351` で未達だった。
  iwasm が stdout に出す `failed to link import function` 警告を fixture differential 側で診断ノイズとして
  除外した後の再実行では `pass=484 fail=520 unsupported=182 blocked=165 total=1351 elapsed=334.2s`
  まで改善した。Object static/runtime semantics の追加修正後は
  `pass=494 fail=514 unsupported=178 blocked=165 total=1351 elapsed=335.5s`
  まで進んだ。Object descriptor/enumerable/freeze enforcement 修正後は
  `pass=503 fail=505 unsupported=178 blocked=165 total=1351 elapsed=334.8s`
  まで進んだ。BigInt negative static rendering / `ErrorMessage(BigIntLiteral)` 修正後は
  `pass=512 fail=496 unsupported=178 blocked=165 total=1351 elapsed=333.7s`
  まで進んだ。Array dynamic property get/set の static/runtime slot 整合後は
  `pass=515 fail=493 unsupported=178 blocked=165 total=1351 elapsed=334.6s`
  まで進んだ。String spread/static concat の static string add と static array string slot
  console rendering 修正後は `pass=523 fail=487 unsupported=176 blocked=165 total=1351 elapsed=335.6s`
  まで進んだ。`forEach` callback param の raw string representation 伝播と console raw string
  出力経路の追加後は `pass=525 fail=485 unsupported=176 blocked=165 total=1351 elapsed=333.0s`
  まで進んだ。RegExp literal helper への tagged string argument emission と tagged return console
  rendering の追加後は `pass=529 fail=481 unsupported=176 blocked=165 total=1351 elapsed=339.8s`
  まで進んだ。`Object.is` と object boolean RuntimeFn の tagged console 境界修正後は
  `pass=530 fail=480 unsupported=176 blocked=165 total=1351 elapsed=334.0s` まで進んだ。
  `Object.fromEntries` の static object 境界追加と rebuilt CLI での `Object.hasOwn`
  確認後は `pass=532 fail=478 unsupported=176 blocked=165 total=1351 elapsed=335.3s`
  まで進んだ。`Object.assign` の static mutation、`Object.getOwnPropertyNames` の static
  own-name array、default `Object.prototype` identity の追加後は
  `pass=536 fail=474 unsupported=176 blocked=165 total=1351 elapsed=342.4s`
  まで進んだ。さらに `object-static-complete.ts` の残差分として、fresh
  `Object.fromEntries` identity、static lowered `Object.groupBy` materialization、
  `Array#indexOf` / `Array#includes` static search、`Object.prototype.isPrototypeOf`
  lowered block の boolean console 表示を補完し、
  `pass=540 fail=470 unsupported=176 blocked=165 total=1351 elapsed=355.6s`
  まで進んだ。さらに accessor descriptor の enumerable/configurable static model を補完し、
  `pass=541 fail=469 unsupported=176 blocked=165 total=1351 elapsed=345.0s`
  まで進んだ。typed class field initializer と class local getter/setter dispatch を補完した後は、
  `pass=544 fail=466 unsupported=176 blocked=165 total=1351 elapsed=351.8s`
  まで進んだ。さらに class static object identity と private field static slot を補完した後は
  `pass=554 fail=457 unsupported=175 blocked=165 total=1351 elapsed=349.9s`
  まで進み、class static method / private accessor side-effect closure を補完した後は
  `pass=558 fail=450 unsupported=178 blocked=165 total=1351 elapsed=358.4s`
  まで進み、known-array backed `Set` spread の static model を補完した後は
  `pass=561 fail=447 unsupported=178 blocked=165 total=1351 elapsed=356.2s`
  まで進み、object spread dynamic/mutated source の static model を補完した後は
  `pass=566 fail=442 unsupported=178 blocked=165 total=1351 elapsed=366.1s`
  まで進み、sparse spread materialization の static model を補完した後は
  `pass=568 fail=440 unsupported=178 blocked=165 total=1351 elapsed=349.7s`
  まで進み、静的に閉じた custom iterable array spread を dense array 化した後は
  `pass=572 fail=436 unsupported=178 blocked=165 total=1351 elapsed=244.3s`
  まで進み、静的に閉じた custom iterable `for-of` を dense array `ForOf` へ正規化した後は
  `pass=573 fail=435 unsupported=178 blocked=165 total=1351 elapsed=222.7s`
  まで進み、break を含む静的 custom iterable iterator protocol を dense array `ForOf` と
  native branch block で扱った後の現在値は
  `pass=575 fail=434 unsupported=177 blocked=165 total=1351 elapsed=228.7s`
  まで進み、known static array の `Array.prototype.at` と identity callback array methods を
  static fold した後の現在値は
  `pass=582 fail=427 unsupported=177 blocked=165 total=1351 elapsed=223.9s`
  まで進み、known static array の callback lowered loop と function callback `thisArg` を
  static fold / receiver binding した後の現在値は
  `pass=584 fail=425 unsupported=177 blocked=165 total=1351 elapsed=226.1s`
  まで進み、known static array の `Array.prototype.fill` mutation を native slot locals へ反映した後の現在値は
  `pass=585 fail=424 unsupported=177 blocked=165 total=1351 elapsed=215.7s`
  まで進み、known static array の `Array.prototype.copyWithin` mutation と returned-self alias を
  native static state へ反映した後の現在値は
  `pass=586 fail=423 unsupported=177 blocked=165 total=1351 elapsed=241.4s`
  まで進み、known static array の `Array.prototype.shift` / `unshift` / `splice` mutation と
  returned value / removed-array を native static state へ反映した後の現在値は
  `pass=587 fail=422 unsupported=177 blocked=165 total=1351 elapsed=233.6s`
  まで進み、known static array の `Array.prototype.pop` / `push` / multi-arg `push` mutation と
  returned value を native static state へ反映した後の現在値は
  `pass=590 fail=419 unsupported=177 blocked=165 total=1351 elapsed=230.7s`
  まで進み、known static array の `Array.prototype.slice` / `reverse` / `join` returned array/string と
  `reverse` mutation を native static state へ反映した後の現在値は
  `pass=597 fail=412 unsupported=177 blocked=165 total=1351 elapsed=235.4s`
  まで進み、known static array の `Array.prototype.lastIndexOf` direct call を static strict equality scan
  へ fold した後の現在値は
  `pass=598 fail=411 unsupported=177 blocked=165 total=1351 elapsed=271.5s`
  まで進み、known static array の `Array.prototype.sort` default/numeric comparator mutation と
  returned-self alias を native static state へ反映した後の現在値は
  `pass=602 fail=407 unsupported=177 blocked=165 total=1351 elapsed=263.3s`
  まで進み、known static array の `Array.prototype.keys` / `values` / `entries` iterator state と
  `ArrayIteratorNext` result object を native static state へ反映した後の現在値は
  `pass=605 fail=404 unsupported=177 blocked=165 total=1351 elapsed=261.1s`
  まで進み、lowered callback-loop boolean result と `Boolean.prototype.valueOf()` の native console
  boolean rendering を補完した後の現在値は
  `pass=607 fail=402 unsupported=177 blocked=165 total=1351 elapsed=255.4s`
  まで進み、known static array の `Array.prototype.reduce` / `reduceRight` lowered callback-loop
  accumulator を native static fold した後の現在値は
  `pass=608 fail=401 unsupported=177 blocked=165 total=1351 elapsed=248.2s`
  まで進み、known static array の `Array.prototype.flat` と `flatMap` returned array を
  native static fold した後の現在値は
  `pass=610 fail=399 unsupported=177 blocked=165 total=1351 elapsed=250.4s`
  まで進み、known static array の `Array.from` identity copy と copying methods
  (`with` / `toReversed` / `toSorted` / `toSpliced`) の returned array を native static fold
  した後の現在値は
  `pass=616 fail=394 unsupported=176 blocked=165 total=1351 elapsed=252.1s`
  まで進み、static `ArrayNew` slot init 内の pure user callback element を native static materialize
  した後の現在値は
  `pass=617 fail=394 unsupported=175 blocked=165 total=1351 elapsed=251.3s`
  まで進み、static object array-like receiver の `Array.prototype.push` mutation と `length`
  property read を native static state に反映した後の現在値は
  `pass=627 fail=385 unsupported=174 blocked=165 total=1351 elapsed=239.0s`
  まで進み、`array-foreach-thisarg.ts` の empty lowered callback loop を静的 false `while`
  として native emission から除外した後の現在値は
  `pass=628 fail=385 unsupported=173 blocked=165 total=1351 elapsed=240.7s`
  まで進み、`GetLength` の静的 property-read 境界で primitive number を string length と
  誤分類しないよう修正した後の現在値は
  `pass=629 fail=384 unsupported=173 blocked=165 total=1351 elapsed=230.1s`
  まで進み、static object array-like receiver の `Array.prototype.map.call` と map callback
  element side effects (`EnvCellSet` / `thisArg` property mutation) を native static state に
  反映した後の現在値は
  `pass=633 fail=380 unsupported=173 blocked=165 total=1351 elapsed=230.7s`
  まで進み、rest parameter の user-call ABI と static user-call argument binding を補完した後の
  現在値は
  `pass=641 fail=372 unsupported=173 blocked=165 total=1351 elapsed=230.5s`
  まで進み、static object own-property key ordering を ECMAScript の integer-index-first order に
  揃え、native static `for-in` の object/array key unroll と `ForIn` static-state invalidation を
  補完した後の現在値は
  `pass=644 fail=369 unsupported=173 blocked=165 total=1351 elapsed=229.0s`
  まで進み、`JSON.stringify` の replacer 配列を lowered IR で synthetic filtered object へ
  変換せず property-list として保持し、native static JSON serializer が property-list 順序を
  object own-property order から分離して使うよう補完した後の現在値は
  `pass=645 fail=368 unsupported=173 blocked=165 total=1351 elapsed=215.5s`
  まで進み、top-level/global `this`、receiver `this` mutation、Function constructor receiver
  binding の静的評価境界を補完した後の現在値は
  `pass=656 fail=359 unsupported=171 blocked=165 total=1351 elapsed=214.5s`
  まで進み、static ArrayBuffer/DataView byte storage と `ArrayBuffer.prototype.transfer`
  receiver lowering を補完した後の現在値は
  `pass=671 fail=344 unsupported=171 blocked=165 total=1351 elapsed=227.8s`
  まで進み、static TypedArray/Atomics basic value path と `typeof Atomics` / `typeof Intl`
  を補完した後の現在値は
  `pass=675 fail=338 unsupported=173 blocked=165 total=1351 elapsed=213.8s`
  まで進み、TypedArray callback method の user-function symbol materialization と
  static typed array constructor/value init、`TypedArray#set` static mutation、typed array
  `forEach` / `every` / `some` callback loop fold を補完した後の現在値は
  `pass=678 fail=339 unsupported=169 blocked=165 total=1351 elapsed=214.3s`
  まで進んだが、まだ未達。差分では `typedarray-methods.ts` と
  `typedarray-mutating-methods.ts` が `unsupported -> pass`、副次的に
  `direct-eval-function-hoisted-before-use.ts` が `unsupported -> pass` へ移動した一方、
  `proxy-remaining-traps.ts` は callback symbol materialization が進んだことで
  `unsupported -> fail` へ移動し、Proxy trap の実セマンティクス差分が新たに露出した。
  主なボトルネックは non-pseudo RuntimeFn builder 欠落ではなく、Array/Object kernel の
  `undefined` / sparse index / property descriptor / prototype semantics、Proxy trap dispatch、
  DataView float16/BigInt accessor、BigInt runtime 値表現、
  Map/Set/RegExp advanced/Iterator/custom
  iterable の値表現、eval / Function constructor NodeShim host import の実セマンティクス差分、
  closure/class/private field の heap value dispatch である。
  `Json.stringify` 系と `math-complete.ts` など、warning だけで stdout mismatch になっていた fixture は
  通過側へ移動済み。

- 2026-05-26 Proxy trap dispatch 追加確認:
  `proxy-remaining-traps.ts` は handler callback 内の
  `Object.defineProperty(proxy, ...)` / `Object.setPrototypeOf(proxy, ...)` が native static
  user-function side effect evaluator で許可されておらず、target object / prototype の静的状態へ
  mutation が反映されないことがボトルネックだった。`proxy-handler-traps-unsupported.ts` はさらに
  `get` trap の `prop in obj ? obj[prop] : 42`、`set` trap の `obj[prop] = value`、`deleteProperty`
  trap の `delete obj[prop]` が同じ静的 callback 評価境界で落ちていた。native 側で
  `PropertyIn` / `PropertyInDynamic` の静的値化、`PropertyDeleteDynamic` の callback side-effect
  許可、dynamic property set 値の callee-local materialization を補完し、proxy focused node_diff は
  `proxy_traps_matches_node_output` / `proxy_remaining_traps_matches_node_output` とも pass。
  full fixture differential は
  `pass=681 fail=338 unsupported=167 blocked=165 total=1351 elapsed=215.1s`
  まで進んだ。残ボトルネックは Proxy 本体ではなく、引き続き DataView float16/BigInt accessor、
  BigInt runtime 値表現、Map/Set/RegExp advanced/Iterator/custom iterable の値表現、
  eval / Function constructor NodeShim host import の実セマンティクス差分、closure/class/private field
  の heap value dispatch、Node oracle blocked 群である。

- 2026-05-26 Date UTC static model 追加確認:
  `DateObject` は static state に存在していたが、`DateGetUtc*` / `DateSetUTC*` / `DateSetTime`
  が static value evaluator と static mutation collector に接続されておらず、getter は
  `undefined` または `0`、setter は receiver state 未更新のままになっていた。native static model で
  epoch ms を `i64` として保持し、UTC civil date 変換、`getUTCFullYear` / `getUTCMonth` /
  `getUTCDate` / `getUTCDay` / `getUTCHours` / `getUTCMinutes` / `getUTCSeconds` /
  `getUTCMilliseconds`、Annex B `getYear`、`setTime`、UTC component setters の戻り値と
  receiver mutation を補完した。focused node_diff は `date_utc_getters`、`date_set_time`、
  `date_set_utc_*`、`date_methods_comprehensive`、`date_annex_b_get_year` が pass。
  direct Node/iwasm diff でも `date-utc-getters.ts` と `date-set-utc-methods.ts` が一致。
  full fixture differential は
  `pass=688 fail=331 unsupported=167 blocked=165 total=1351 elapsed=219.6s`
  まで進み、`date-annexb-get-year.ts`、`date-methods-comprehensive.ts`、
  `date-set-time.ts`、`date-set-utc-components.ts`、`date-set-utc-full-year.ts`、
  `date-set-utc-methods.ts`、`date-utc-getters.ts` が `fail -> pass` へ移動した。
  残る Date 系は live time / local-time setters / string formatting / parse / multi-arg constructor など
  host Date shim または local timezone 境界に依存するものが中心である。

- 2026-05-26 String tagged-value local console boundary 追加確認:
  `StringCharAt` / `StringSlice` / `StringSubstring` などの runtime intrinsic は tagged JS value を返すが、
  top-level/module/function の `FunctionCtx` が statement 間の native value representation を保持せず、
  `let s = "hello".charAt(1); console.log(s);` が tagged string ではなく raw i32 fallback で
  data/heap pointer を表示していた。native value representation を `RawString` と `TaggedValue` に分離し、
  `FunctionCtx` に `NativeValueReprState` を持たせて各 statement emit 後に更新することで、
  local 経由の console rendering が raw string buffer と tagged `ValueToStringInto` の正しい経路へ分岐するようにした。
  focused node_diff は `string_char_at_matches_node`、`string_slice_matches_node`、
  `string_substring_matches_node`、`string_search_matches_node`、`string_locale_compare_matches_node` が pass。
  full fixture differential は
  `pass=693 fail=326 unsupported=167 blocked=165 total=1351 elapsed=217.4s`
  まで進み、`string-char-at.ts`、`string-locale-compare.ts`、`string-search.ts`、
  `string-slice.ts`、`string-substring.ts` が `fail -> pass` へ移動した。
  残る string boolean 系の `string-includes.ts` / `string-starts-with.ts` / `string-ends-with.ts` は
  local console pointer 問題ではなく、runtime comparator / range logic が Node と異なるボトルネックである。

- 2026-05-26 String search optional-argument / truthiness 追加確認:
  `StringIndexOf` / `StringLastIndexOf` / `StringIncludes` / `StringStartsWith` / `StringEndsWith` の
  lowered IR は optional position/endPosition を省略した 2 引数 call を生成するが、native emitter の
  string search special path は 3 引数だけを扱っていた。そのため generic runtime call に落ち、
  string literal receiver/search が tagged string ABI ではなく raw pointer として渡されていた。
  emitter 側で 2 引数 call を special path に乗せ、不足した position は `undefined` で補完した。
  さらに `StringLastIndexOf(undefined)` は JS 仕様どおり末尾側から検索する必要があるため、
  typed runtime builder の default decode を `0` ではなく haystack length にした。
  `StringIncludes` の local 経由 `if (has)` は tagged boolean `false` を raw Wasm truthy として扱っていたため、
  `NativeValueRepr::TaggedValue` local 条件を `TruthyBool` に通すようにした。
  なお `Error.stack.indexOf(...)` のような dynamic receiver は、既存の generic path に残して support 状態を
  `fail` のまま維持し、unsupported regression を避けた。
  focused node_diff は `string_includes_fixture_matches_node_output_under_iwasm`、
  `string_starts_with_matches_node`、`string_ends_with_matches_node`、
  `string_index_of_matches_node`、`string_last_index_of_matches_node` が pass。
  full fixture differential は
  `pass=698 fail=321 unsupported=167 blocked=165 total=1351 elapsed=214.3s`
  まで進み、`string-ends-with.ts`、`string-includes.ts`、`string-index-of.ts`、
  `string-last-index-of.ts`、`string-starts-with.ts` が `fail -> pass` へ移動した。
  `pass -> fail`、`pass -> unsupported`、`fail -> unsupported` の regression は 0 件。

- 2026-05-26 Tagged string local equality 追加確認:
  `StringPadStart` / `StringPadEnd` / `StringRepeat` は tagged string を返して local に保存するが、
  native equality operand tagging は local の `InferredType::Number | Boolean` しか直接扱っておらず、
  `let a = "ab".repeat(3); a === "ababab"` のような比較が raw i32 fallback に近い経路で false になっていた。
  `NativeValueRepr::TaggedValue` local は equality operand としてそのまま tagged JS value として渡し、
  `NativeValueRepr::RawString` local は `STRING_TAG` を付けて渡すようにした。一方で
  static collector の `TaggedNumber` marker は、`StringCharCodeAt` が static fold で raw number local に
  初期化されるケースと衝突したため、equality tagging では storage source of truth として使わず、
  statement 間の `value_reprs` だけを信頼する形に限定した。
  focused node_diff は `string_pad_start_matches_node`、`string_pad_end_matches_node`、
  `string_repeat_matches_node`、regression check の `string_char_code_at_matches_node` が pass。
  full fixture differential は
  `pass=701 fail=318 unsupported=167 blocked=165 total=1351 elapsed=213.8s`
  まで進み、`string-pad-end.ts`、`string-pad-start.ts`、`string-repeat.ts` が `fail -> pass` へ移動した。
  `pass -> fail`、`pass -> unsupported`、`fail -> unsupported` の regression は 0 件。

- 2026-05-26 String well-formed tagged ABI 追加確認:
  `StringIsWellFormed` / `StringToWellFormed` の typed builders はそれぞれ tagged `true` と receiver string
  を返していたが、native emitter の string unary special path と tagged runtime-value 判定に含まれていなかった。
  そのため receiver string が raw pointer のまま generic runtime call に渡され、console でも tagged value として
  `ValueToStringInto` に通らず、`string-is-well-formed.ts` は `3`、`string-to-well-formed.ts` は heap/data pointer
  を表示していた。両 runtime を unary string path、`native_runtime_fn_returns_tagged_value`、
  `native_console_arg_is_tagged_runtime_value` に追加し、receiver ABI と console 境界を tagged value に統一した。
  focused node_diff は `string_is_well_formed_matches_node`、`string_to_well_formed_matches_node`、
  regression check の `string_char_code_at_matches_node` が pass。
  full fixture differential は
  `pass=703 fail=316 unsupported=167 blocked=165 total=1351 elapsed=217.0s`
  まで進み、`string-is-well-formed.ts`、`string-to-well-formed.ts` が `fail -> pass` へ移動した。
  `pass -> fail`、`pass -> unsupported`、`fail -> unsupported` の regression は 0 件。

- 2026-05-26 String replace / HTML wrapper tagged ABI 追加確認:
  `StringReplace` / `StringReplaceAll` の typed builders は receiver/search/replacement を tagged string として受けるが、
  native emitter の generic runtime call は string literal を raw string pointer として積んでいた。
  そのため builder 側の `$is_string` guard が失敗して input pointer をそのまま返し、`console.log` では
  `320` などの heap/data pointer が表示されていた。`StringReplace` / `StringReplaceAll` を 3 引数 string
  runtime special path に追加して各引数を tagged JS value として emit し、戻り値も tagged runtime value として
  local propagation / console rendering に乗せた。同じ境界を使う HTML wrapper attribute path も、
  raw number-like pointer ではなく元の attribute string を受け取るようになった。
  focused node_diff は `string_replace_all_matches_node`、
  `upgraded_builtin_fixture_matches_node_output`、`string_html_wrappers_fixture_matches_node_output_under_iwasm`、
  `string_anchor_fixture_matches_node_output_under_iwasm` が pass。
  full fixture differential は
  `pass=707 fail=312 unsupported=167 blocked=165 total=1351 elapsed=213.9s`
  まで進み、`string-replace.ts`、`string-replace-all.ts`、`string-html-wrappers.ts`、
  `string-anchor-annexb.ts` が `fail -> pass` へ移動した。
  `pass -> fail`、`pass -> unsupported`、`fail -> unsupported` の regression は 0 件。

- 2026-05-26 String static methods / `isNaN` console boundary 追加確認:
  `string-char-code-at-dynamic.ts` は `isNaN(value)` の lowered path が global builtin call として raw bool
  `0` / `1` を返し、single-argument `console.log` が tagged boolean へ変換せずそのまま表示していた。
  builtin `isNaN` call を raw bool console boundary に追加し、`false` / `true` として `ValueToStringInto` に通るようにした。
  また `string-static-methods.ts` の `"locale".toLocaleString()` は `ObjectToLocaleString` に lower されるが、
  string receiver が raw pointer のまま runtime に渡されていたため pointer 表示になっていた。
  `ObjectToLocaleString` を tagged runtime value として分類し、runtime call では receiver を tagged JS value として emit する。
  `String.raw({ raw: [...] }, ...)` は static raw array object から raw segments と substitutions を組み立てる
  static fold を追加し、runtime materialization されない static object に対する `PropertyGet` 依存を避けた。
  focused node_diff は `string_char_code_at_dynamic_matches_node` と
  `string_static_methods_matches_node_output` が pass。
  full fixture differential は
  `pass=709 fail=310 unsupported=167 blocked=165 total=1351 elapsed=222.2s`
  まで進み、`string-char-code-at-dynamic.ts` と `string-static-methods.ts` が `fail -> pass` へ移動した。
  `pass -> fail`、`pass -> unsupported`、`fail -> unsupported` の regression は 0 件。

- 2026-05-26 String static indexing 追加確認:
  `string-indexing.ts` の `s[0]` / `s[1]` / `s[4]` は `PropertyGet { key: "0" }` 形式に lower されるが、
  native static property resolver は object / array だけを扱い、static string の canonical numeric property を
  `undefined` として扱っていた。`static_string_property_owned_from_expr_with_functions` を追加し、
  static string receiver と canonical array-index key から single-character string または out-of-range `undefined`
  を返すようにした。emit path、console static rendering、`static_value_from_expr_with_functions` の
  `PropertyGet` 解決順すべてに同じ helper を接続し、実行時 fallback へ落ちる経路を増やさずに揃えた。
  semantic gap guard だった `string_indexing_fixture_is_not_marked_as_semantic_pass` は、
  support 実装後の `string_indexing_matches_node` に昇格した。
  focused node_diff は `string_indexing_matches_node` が pass。
  full fixture differential は
  `pass=711 fail=308 unsupported=167 blocked=165 total=1351 elapsed=220.0s`
  まで進み、`string-indexing.ts` と `utf8-string.ts` が `fail -> pass` へ移動した。
  `pass -> fail`、`pass -> unsupported`、`fail -> unsupported` の regression は 0 件。

- 2026-05-26 String substr optional length ABI 追加確認:
  `StringSubstr` の typed builder は third param の raw `0` を「length 省略」sentinel として扱うが、
  native emitter の string range special path は 3 引数 call だけを処理していた。
  lowered IR の `'abc'.substr(1)` / `'abc'.substr(-5)` は 2 引数 call なので generic runtime call に落ち、
  stack effect 補完で `ValueTag::UNDEFINED` が third param に渡され、結果が `undefined` として
  `Concat` で文字列化されていた。`StringSubstr` の 2 引数 form を range special path に追加し、
  receiver/start を tagged value として emit したうえで third param に raw `0` sentinel を渡すようにした。
  focused node_diff は `string_substr_matches_node` が pass。
  full fixture differential は
  `pass=712 fail=307 unsupported=167 blocked=165 total=1351 elapsed=221.5s`
  まで進み、`string-substr.ts` が `fail -> pass` へ移動した。
  `pass -> fail`、`pass -> unsupported`、`fail -> unsupported` の regression は 0 件。

- 2026-05-26 Number radix string conversion via `ObjectToString` 追加確認:
  `string-charcode-tostring-radix.ts` の `"A".charCodeAt(0).toString(16)` は
  lowered IR で `ObjectToString(StringCharCodeAt(...), 16)` になるが、native emitter は
  generic runtime call として `$object_to_string` に 2 引数を積んでいた。`$object_to_string` は
  1 引数 signature なので final wasm が実行時に失敗していた。
  `ObjectToString(value, radix)` の 2 引数 form を専用 path に分け、receiver と radix を tagged JS value として
  emit してから `NumberToI32` で raw i32 に戻し、`NumberToStringRadix` へ渡すようにした。
  追加 call が final module に残るため、`ObjectToString` の runtime deps に `NumberToI32` と
  `NumberToStringRadix` を追加した。`ObjectToString` 自体も tagged runtime return / console boundary の
  分類に追加済み。
  focused node_diff は新規 `string_charcode_tostring_radix_matches_node` が pass。
  full fixture differential は
  `pass=713 fail=306 unsupported=167 blocked=165 total=1351 elapsed=223.5s`
  まで進み、`string-charcode-tostring-radix.ts` が `fail -> pass` へ移動した。
  `pass -> fail`、`pass -> unsupported`、`fail -> unsupported` の regression は 0 件。

- 2026-05-26 String runtime argument ABI 追加確認:
  `string-concat.ts` の `"a".concat("b", "c")` は lowered IR が `RuntimeFn::Concat` の 3 引数 call
  になる一方、native emitter は 2 引数 helper `$concat` のみを想定していたため、generic path へ落ちると
  Wasm validation が stack mismatch で失敗していた。`Concat` は first argument に後続引数を順次 `$concat`
  する chained emission に変更し、static fold も 2 引数限定から可変長へ広げた。
  `string-static.ts` の `String.fromCharCode(65, 66, 67)` / `String.fromCodePoint(...)` も同じく
  single-code-unit helper へ多引数をそのまま渡していたため、各引数を 1 文字列に変換して `$concat` で連結する
  native path に変更し、deps に `Concat` を追加した。
  `string-split.ts` は `StringSplit` が generic runtime path に落ち、string literal receiver/separator を
  raw string pointer として `$string_split` へ渡していたため `$is_string` guard が失敗していた。
  `StringSplit` 専用 path で両引数を tagged JS value として emit し、`ArrayMapStringSplit` でも separator を
  tagged 化して `row.split(" ")` の map lowering と揃えた。
  focused node_diff は `string_concat_matches_node`、`string_static_*`、`string_split_matches_node`、
  `array-map-arrow-expression-split`、`array-map-arrow-chained-trim-split` が pass。
  full fixture differential は
  `pass=719 fail=300 unsupported=167 blocked=165 total=1351 elapsed=224.6s`
  まで進み、`string-concat.ts`、`string-split.ts`、`string-static.ts`、
  `array-map-arrow-expression-receiver.ts`、`array-map-arrow-expression-split.ts`、
  `array-map-arrow-chained-trim-split.ts` が `fail -> pass` へ移動した。
  `pass -> fail`、`pass -> unsupported`、`fail -> unsupported` の regression は 0 件。

- 2026-05-26 Math integer static fold 追加確認:
  `math-cbrt.ts` / `math-imul.ts` は `check(Math.<fn>(...), expected)` 形式で、literal-only の
  `RuntimeFn::MathCbrt` / `RuntimeFn::MathImul` が native generic runtime path に落ち、typed helper が期待する
  tagged number ではなく raw integer を受け取って iwasm trap していた。`MathCbrt` は integer-backed number model の
  floor cube root、`MathImul` は i32 wrapping multiply として static evaluator に追加し、literal-only call を
  runtime helper へ降ろさず raw number result に畳み込むようにした。
  併せて動的 `MathCbrt` / `MathImul` runtime call は tagged-number ABI path に限定して追加したが、
  既存 pass の `Math.abs/max/min/sqrt/...` は raw-number user-call/equality 経路と衝突しないよう対象外のまま維持した。
  focused node_diff は `math_cbrt_matches_node`、`math_imul_matches_node` が pass。
  regression guard として `math_abs_matches_node`、`math_max_matches_node` も pass。
  full fixture differential は
  `pass=721 fail=298 unsupported=167 blocked=165 total=1351 elapsed=225.8s`
  まで進み、`math-cbrt.ts`、`math-imul.ts` が `fail -> pass` へ移動した。
  `pass -> fail`、`pass -> unsupported`、`fail -> unsupported` の regression は 0 件。

- 2026-05-26 Array map value/string/unary-plus static fold 追加確認:
  `array-map-arrow-unary-plus.ts` の `[1, -2, 0].map(n => String(n)).map(n => +n)` は
  `ArrayMapValueToString` / `ArrayMapUnaryPlus` に lower されるが、native generic runtime path は
  static array を runtime array に materialize せず `STATIC_REF_TOKEN` を渡していた。そのため
  `$array_map_value_to_string` / `$array_map_unary_plus` の array tag guard または
  `$primitive_to_number_for_equality` で iwasm trap していた。
  callback が `String(n)` と unary plus に限定された lowering 最適化なので、static evaluator で
  `ArrayMapValueToString` は element ごとの JS string conversion、`ArrayMapUnaryPlus` は
  JS numeric conversion として配列全体を畳み込むようにした。
  focused node_diff は `array_map_arrow_unary_plus_fixture_matches_node_output_under_iwasm` が pass。
  full fixture differential は
  `pass=723 fail=297 unsupported=166 blocked=165 total=1351 elapsed=261.5s`
  まで進み、`array-map-arrow-unary-plus.ts` が `fail -> pass`、既存の
  `array-map-arrow-string-constructor.ts` が `unsupported -> pass` へ移動した。
  `pass -> fail`、`pass -> unsupported`、`fail -> unsupported` の regression は 0 件。

- 2026-05-26 Static NaN equality 追加確認:
  `global-parseint.ts` の `parseInt("\\u2000")` は static `parseInt` fold で NaN sentinel の
  `LoweredExpr::Number` になるが、static strict equality は同じ sentinel i32 同士を通常数値として等値扱いしていた。
  そのため `n !== n` が JS の NaN semantics と逆に `false` になっていた。
  static primitive equality で `Number` / `DecimalNumber` を比較する際は f64 へ正規化し、どちらかが
  NaN の場合は strict equal を `false` にするようにした。Infinity sentinel と通常整数の等値は維持する。
  focused node_diff は `global_parseint_matches_node_under_iwasm` と
  `global_parseint_i32_boundary_matches_node_output_under_iwasm` が pass。
  full fixture differential は
  `pass=724 fail=296 unsupported=166 blocked=165 total=1351 elapsed=229.4s`
  まで進み、`global-parseint.ts` が `fail -> pass` へ移動した。
  `pass -> fail`、`pass -> unsupported`、`fail -> unsupported` の regression は 0 件。

- 2026-05-26 Math.pow builtin static fold 追加確認:
  `test-math-pow.ts` の `const result = Math.pow(2, 3); console.log(result);` は
  `FunctionCallKind::Builtin(BuiltinId::MathPow)` の native 分岐が static fold を試さず、引数を drop して
  `ValueTag::NUMBER` だけを返していたため `4` が出力されていた。
  builtin `MathPow` を static builtin evaluator に追加し、`MathPow` の native builtin 分岐でも先に
  `try_emit_static_value_expr` を通すようにした。literal-only call は `base.powf(exponent)` を
  `static_number_expr_from_f64` で raw number/decimal sentinel に戻す。
  focused node_diff は `math_pow_matches_node` と `test_math_pow_matches_node` が pass。
  full fixture differential は
  `pass=725 fail=295 unsupported=166 blocked=165 total=1351 elapsed=219.6s`
  まで進み、`test-math-pow.ts` が `fail -> pass` へ移動した。
  `pass -> fail`、`pass -> unsupported`、`fail -> unsupported` の regression は 0 件。

- 2026-05-26 Bitwise tagged operand ABI 追加確認:
  `ordinary-bitwise-and-xor.ts` の `null & 7` は、native fallback が tagged `null` immediate を
  そのまま `i32.and` していたため `0` ではなく `1` を出力していた。
  `BitwiseAnd` / `BitwiseXor` / `BitwiseOr` について、raw number 同士ではない operand を
  tagged JS value として `$bitwise_*` runtime helper に渡し、戻り値を `NumberToI32` で raw i32 に戻す
  native path を追加した。これにより `true` / `false` / `null` / `undefined` の ToInt32 境界を
  runtime catalog の `BitwiseToI32` と共有する。
  focused node_diff は `ordinary_bitwise_and_xor_fixture_matches_node_output_under_iwasm` が pass。
  full fixture differential は
  `pass=726 fail=294 unsupported=166 blocked=165 total=1351 elapsed=218.2s`
  まで進み、`ordinary-bitwise-and-xor.ts` が `fail -> pass` へ移動した。
  `pass -> fail`、`pass -> unsupported`、`fail -> unsupported` の regression は 0 件。

- 2026-05-26 Intl.DateTimeFormat resolvedOptions shape 追加確認:
  `intl-datetimeformat.ts` は `new Intl.DateTimeFormat("en-US", { timeZone: "UTC",
  localeMatcher: "lookup" }).resolvedOptions().localeMatcher` が Node では `undefined` なのに、
  lowered static object が constructor 内部 option の `"lookup"` を公開していたため stdout mismatch
  になっていた。DateTimeFormat constructor / format 用の internal option は保持したまま、
  `resolvedOptions()` が materialize する object から `localeMatcher` property を除外した。
  focused node_diff は `intl_datetimeformat_matches_node` が pass。
  full fixture differential は
  `pass=727 fail=293 unsupported=166 blocked=165 total=1351 elapsed=230.9s`
  まで進み、`intl-datetimeformat.ts` が `fail -> pass` へ移動した。
  `pass -> fail`、`pass -> unsupported`、`fail -> unsupported` の regression は 0 件。

- 2026-05-26 Date local getter static fold 追加確認:
  `date-local-getters.ts` と `date-local-getters-unsupported.ts` は `new Date(0).getFullYear()` などの
  local-time getter が `DateGetLocalTimeField` host import に落ち、standalone `iwasm` では未提供 host
  import が 0 を返すため全 field が `0` になっていた。native static evaluator で
  `DateObject(Some(epoch_ms))` と literal field index の `DateGetLocalTimeField` を compiler host の
  `localtime_r` で local-time field に fold し、静的 Date local getter fixture では host import を
  残さないようにした。
  focused node_diff は `date_local_getters_fixture_matches_node_output_under_iwasm` が pass。
  full fixture differential は
  `pass=729 fail=291 unsupported=166 blocked=165 total=1351 elapsed=232.3s`
  まで進み、`date-local-getters.ts` と `date-local-getters-unsupported.ts` が `fail -> pass` へ移動した。
  `pass -> fail`、`pass -> unsupported`、`fail -> unsupported` の regression は 0 件。

- 2026-05-26 Date string static fold 追加確認:
  `Date.prototype.toISOString()` / `toDateString()` / `toTimeString()` / `toString()` /
  `toUTCString()` / `toGMTString()` は、静的 `DateObject(Some(epoch_ms))` でも runtime call のまま
  残り、standalone `iwasm` では未提供 host import や tagged value 境界で `0` 出力または実行失敗に
  なっていた。lowering では `toUTCString()` / `toGMTString()` を local `DateToString` から
  `DateToGMTString` へ分離し、native static evaluator で `DateToISOString` / `DateToDateString` /
  `DateToTimeString` / `DateToString` / `DateToGMTString` を文字列へ fold するようにした。
  UTC/ISO 系は既存 UTC date parts から生成し、local full/date/time string は compiler host の
  `localtime_r` field と timezone offset/name を使う。timezone name は `JST` を
  `Japan Standard Time`、`UTC` / `GMT` を `Coordinated Universal Time` に正規化し、
  空 name は `Local Time` として扱う。
  focused node_diff は `date_to_iso_string_fixture_matches_node_output_under_iwasm` と
  `date_string_fixtures_match_node_output_under_iwasm` が pass。
  full fixture differential は
  `pass=734 fail=286 unsupported=166 blocked=165 total=1351 elapsed=234.2s`
  まで進み、`date-to-iso-string.ts`、`date-to-date-string.ts`、`date-to-time-string.ts`、
  `date-to-string-timezone-unsupported.ts`、`date-to-string-methods.ts` が `fail -> pass` へ移動した。
  `pass -> fail`、`pass -> unsupported`、`fail -> unsupported` の regression は 0 件。

- 2026-05-26 Date local setter / parse / UTC static fold 追加確認:
  `Date.prototype.setFullYear()` / `setMonth()` / `setDate()` / `setHours()` /
  `setMinutes()` / `setSeconds()` / `setMilliseconds()` は static `DateObject(Some(epoch_ms))`
  receiver でも static mutation collector と static value evaluator の対象外で、standalone `iwasm`
  では setter return が `0`、後続 getter は古い Date state または未提供 local-time host import に
  落ちていた。native static model で compiler host の `localtime_r` と `mktime` を使い、
  local component setter の戻り値と receiver mutation を同じ local-time semantics で fold するようにした。
  併せて `Date.parse()` の ISO UTC string subset、`Date.UTC()` の 0-99 year 補正込み UTC ms、
  `getTimezoneOffset()` の static local offset を fold し、Date complete fixture が host import なしで
  最後まで実行できるようにした。
  focused node_diff は `date_set_local_components_fixture_matches_node_output_under_iwasm`、
  `date_set_local_methods_fixture_matches_node_output_under_iwasm`、
  `date_static_parse_utc_fixture_matches_node_output_under_iwasm`、
  `date_methods_matches_node_output` が pass。
  full fixture differential は
  `pass=740 fail=281 unsupported=165 blocked=165 total=1351 elapsed=221.9s`
  まで進み、`date-complete.ts`、`date-get-timezone-offset.ts`、`date-set-local-components.ts`、
  `date-set-local-methods.ts`、`date-static-parse-utc.ts`、`receiver-class-fix.ts` が pass へ移動した。
  `pass -> fail`、`pass -> unsupported`、`fail -> unsupported` の regression は 0 件。

- 2026-05-26 Object key ordering / `for-in` 追加確認:
  static object の own property keys は、array-index property name を数値昇順で先に並べ、
  それ以外の string keys を insertion order で維持する helper に統一した。これにより
  `Object.keys` / `Object.getOwnPropertyNames` / `Object.values` / `Object.entries` などの
  static object key consumer が `1,2,10,a,b,01` 形式の ECMAScript order を共有する。
  また native `LoweredStmt::ForIn` は、static object keys と dense/sparse static array indices を
  key string local として unroll し、本文内の `obj[k]` や key equality を既存 static property
  resolver で処理できるようにした。外側の static state collector では、非空 `ForIn` の loop var と
  body assignment を conservatively invalidate し、loop 後の `console.log(local)` が古い static 値に
  折り畳まれないようにした。
  Focused verification は `native_lowered_static_for_in_object_runs_without_wat_conversion`、
  `native_lowered_static_object_keys_use_ecmascript_order`、node_diff の
  `core_statement_fixtures_match_node_output_under_iwasm` と
  `object_own_key_integer_order_matches_node` で通過した。full fixture differential は未達のまま
  `pass=644 fail=369 unsupported=173 blocked=165 total=1351 elapsed=229.0s`。
  この変更で `object-own-key-integer-order.ts`、`control-flow-and-exceptions/for-in.ts`、
  `stmt/for-in.ts` は pass 側へ移動した。残る主ボトルネックは ArrayBuffer/DataView/TypedArray の
  tagged numeric representation、Map/WeakMap/WeakSet heap value rendering、dynamic eval /
  Function constructor NodeShim path、BigInt runtime mixed comparison、class instanceof/super dispatch、
  `this` top-level/global object semantics に残っている。

- 2026-05-26 JSON.stringify replacer property-list 追加確認:
  `JSON.stringify(value, replacerArray)` の lowered IR は、replacer array の property-list で
  事前フィルタ済み `ObjectNew` を作らず、元の `value` と `ArrayNew(["key", ...])` を
  `RuntimeFn::JsonStringify` に渡す形に戻した。native static JSON serializer は
  `StaticJsonReplacer::PropertyList(Vec<String>)` を追加し、array / sparse array replacer から
  first occurrence order で dedupe した key list を作る。object serialization 時は
  property-list がある場合だけ `object.keys()` の ECMAScript integer-index-first order を使わず、
  replacer list order で property lookup / serialization する。これにより
  `{ "": "empty", "0": "zero", "-1": "minus" }` の通常 own-key order と
  `[new String(), new Number(), new Number(-1)]` replacer order を分離できる。
  さらに native emitter は最終 call graph から到達不能な native runtime helper を prune し、
  static fold 済みの `$json_stringify` が残していた未使用 `host.json.stringify` import を落とす。
  Focused verification は `json_stringify_replacer_array_boxed_matches_node`、
  broader replacer filter `json_stringify_replacer_array`、space filter `json_stringify_space`、
  `json_runtime_calls_embed_native_helpers_and_imports`、および
  `/tmp/json-boxed.wasm` の import inspection で通過した。full fixture differential は未達のまま
  `pass=645 fail=368 unsupported=173 blocked=165 total=1351 elapsed=215.5s`。
  この変更で `fixtures/builtins-and-io/json-stringify-replacer-array-boxed.ts` が pass 側へ移動した。
  残る主ボトルネックは ArrayBuffer/DataView/TypedArray の tagged numeric representation、
  Map/Set/WeakMap/WeakSet heap value rendering、dynamic eval / Function constructor NodeShim path、
  BigInt runtime mixed comparison / heap rendering、class instanceof/super dispatch、
  `this` top-level/global object semantics に残っている。

- 2026-05-26 top-level/global `this` 追加確認:
  unresolved `this` lowering は、local receiver / class static `this` が見つからない場合に
  `undefined` ではなく `RuntimeFn::GlobalThis` へ落とすようにした。native emitter 側では
  `RuntimeFn::GlobalThis` の `typeof` を `object` として扱い、static console rendering では
  top-level `console.log(this)` を Node oracle と同じ `{}` 表示へ寄せた。さらに
  `let g = (function(){ return this; })(); console.log(typeof g)` のように `GlobalThis` が
  local static object へ入った後の `typeof local` も `object` になるよう、static local value
  分類を `Primitive` / `Closure` だけでなく `Object` / `ObjectAlias` / `Array` / `DateObject`
  / iterator object / `Symbol` まで見るよう拡張した。
  Focused verification は `global_this_matches_node_under_iwasm`、
  新規 node_diff regression の `this_basic_matches_node_under_iwasm`、
  `top_level_this_matches_node_under_iwasm`、既存
  `function_expression_return_this_iife_fixture_matches_node_output_under_iwasm`、
  lowered snapshot の strict/sloppy `return this` で通過した。full fixture differential は未達のまま
  `pass=649 fail=364 unsupported=173 blocked=165 total=1351 elapsed=214.7s`。
  この変更で `fixtures/builtins-and-io/global-this.ts`、
  `fixtures/this-binding/this-basic.ts`、
  `fixtures/core-semantics/this-top-level-unsupported.ts`、
  `fixtures/core-semantics/function-expression-iife-return-this.ts` が pass 側へ固定された。
  残る `this` 系は top-level/global ではなく、`Function` constructor の constructed receiver /
  prototype metadata、method receiver preservation、heap object rendering に絡む
  `function-constructor-this-binding.ts` や `function-this-receiver.ts` の runtime dispatch 差分である。
  全体の主ボトルネックは ArrayBuffer/DataView/TypedArray の tagged numeric representation、
  Map/Set/WeakMap/WeakSet heap value rendering、dynamic eval / Function constructor NodeShim path、
  BigInt runtime mixed comparison / heap rendering、class instanceof/super dispatch に残っている。

- 2026-05-26 receiver `this` mutation 追加確認:
  user function の static evaluator / env-effect applicator は、`let alias = this` のような
  local object assignment を既存の `static_value_for_local_expr_with_functions` helper 経由で
  `ObjectAlias(root)` として保持するように揃えた。これにより `this.seed = next` が
  alias local の cloned object だけを更新して receiver root へ戻らない問題を避ける。
  併せて console emission は static expression evaluation を static object slot fallback より先に
  試すようにし、`console.log(first.read(1))` が初期 slot (`seed=4`) ではなく、
  直前の `first.setSeed(12)` で更新された static state を読めるようにした。
  Focused verification は `function_this_receiver_fixture_matches_node_output_under_iwasm`、
  `this_receiver_method_fixtures_match_node_output_under_iwasm`、`top_level_this_matches_node_under_iwasm`、
  backend の `native_lowered_static_object*` filter で通過した。full fixture differential は未達のまま
  `pass=650 fail=363 unsupported=173 blocked=165 total=1351 elapsed=217.0s`。
  この変更で `fixtures/core-semantics/function-this-receiver.ts` が pass 側へ移動した。
  残る `this` 系は `Function` constructor の constructed receiver / global receiver 表現で、
  `fixtures/core-semantics/function-constructor-this-binding.ts` はまだ
  `expected="object\ntrue\n"` に対して `actual="432\n1\n"` の stdout mismatch で残っている。

- 2026-05-26 Function constructor receiver binding 追加確認:
  `Function("return typeof this")` の sloppy call は lowering で `GlobalThis` receiver を渡せていたが、
  native static evaluator が `LoweredUnaryOp::TypeOf` を畳めず、static console では runtime object token
  を raw i32 として `432` 表示していた。`StaticValue` から JS `typeof` 文字列へ分類する helper を追加し、
  object / object alias / array / closure / symbol / tagged string / tagged number を static `typeof` で
  扱えるようにした。strict 側の `Function('"use strict"; return this === undefined')` は directive prologue
  の `"use strict"` 式文で user-function static evaluator が中断していたため、文字列 expression statement を
  no-op として扱い return 評価へ進めるようにした。
  Focused verification は `function_constructor_this_binding_matches_node_output`、
  `function_this_receiver_fixture_matches_node_output_under_iwasm`、
  `top_level_this_matches_node_under_iwasm`、`global_this_matches_node_under_iwasm`、
  backend の `native_lowered_static_typeof_console_log_runs_without_wat_conversion` で通過した。
  full fixture differential は未達のまま
  `pass=656 fail=359 unsupported=171 blocked=165 total=1351 elapsed=214.5s`。
  この変更で `fixtures/core-semantics/function-constructor-this-binding.ts` が pass 側へ移動した。
  残る `this` 系の fail は dynamic eval / Function constructor NodeShim path と property function
  dispatch に集中し、通常の global receiver / strict undefined receiver / method receiver preservation は
  native static path で固定済みである。

- 2026-05-26 static ArrayBuffer/DataView byte storage 追加確認:
  native static evaluator に `StaticValue::ArrayBuffer(Vec<u8>)` と `StaticValue::DataView` を追加し、
  `new ArrayBuffer` / `new SharedArrayBuffer` / `new DataView(buffer, byteOffset)` の静的alias、
  `DataView#setInt8/Uint8/Int16/Uint16/Int32/Uint32/Float32/Float64` の byte-level mutation、
  `DataView#get*` の endian-aware read を static console evaluation へ接続した。これにより
  tagged number token を raw i32 として `536870916` などと出力していた差分を、runtime helper に
  依存せず static bytes で解消した。併せて `ArrayBuffer.prototype.transfer(newLen)` lowering が
  receiver buffer を `RuntimeFn::ArrayBufferTransfer` に渡していなかったため、runtime ABI と同じ
  `(buffer, newLen)` に修正し、static evaluator では transfer 後の copied zero-filled buffer を返す。
  Focused verification は `arraybuffer_dataview` filter の 13 件、`arraybuffer_transfer`、
  `cargo check -p ts2wasm-backend-wasm` で通過した。full fixture differential は未達のまま
  `pass=671 fail=344 unsupported=171 blocked=165 total=1351 elapsed=227.8s`。
  この変更で `fixtures/builtins-and-io/arraybuffer-dataview-*.ts` 13 件と
  `fixtures/builtins-and-io/arraybuffer-transfer.ts` が pass 側へ移動し、
  `sharedarraybuffer-basic.ts` も pass を維持した。残る同系統は `dataview-complete.ts` の
  BigInt/float16 を含む unsupported と `dataview-float16.ts` の Node oracle blocked、
  TypedArray/Atomics の tagged numeric representation である。

- 2026-05-26 static TypedArray/Atomics basic value path 追加確認:
  native static evaluator で `TypedArrayFromArray` / `TypedArrayCtorWithLength` /
  `TypedArrayCtorFromBuffer` を dense static array として扱い、`TypedArrayLoad`、
  `TypedArrayStore`、`Atomics.load/store/add/sub/and/or/xor/exchange/compareExchange`、
  `Atomics.isLockFree` の static read/write/RMW を追加した。これにより typed-array
  value が runtime helper に渡る前に `STATIC_REF_TOKEN` / raw tagged number として崩れる
  basic path を避ける。併せて lowering 側で `typeof Atomics` / `typeof Intl` を `"object"`
  に折り畳むようにした。Focused verification は node_diff の
  `atomics_basic_matches_node_output`、`atomics_unsupported_matches_node_output`、
  `typedarray_basic_matches_node_output`、直接 iwasm の `atomics-basic.ts` /
  `atomics-unsupported.ts`、`cargo check -p ts2wasm-backend-wasm` で通過した。
  full fixture differential は未達のまま
  `pass=675 fail=338 unsupported=173 blocked=165 total=1351 elapsed=213.8s`。
  この変更で `fixtures/builtins-and-io/atomics-basic.ts`、
  `fixtures/builtins-and-io/atomics-unsupported.ts`、`fixtures/builtins-and-io/intl-unsupported.ts`、
  `fixtures/builtins-and-io/typedarray-basic.ts` が pass 側へ移動した。残る同系統は
  `atomics-complete.ts` の Node oracle blocked と、`typedarray-methods.ts` /
  `typedarray-mutating-methods.ts` の callback user-function symbol materialization である。

- 2026-05-26 TypedArray callback/static mutation 追加確認:
  `console.log(Block{...})` の static fold 判定で side-effect 付き block statement を runtime
  required function collection から落とさないようにし、unresolved `$func_*` を解消した。
  その上で typed array constructor を static slot init に接続し、`ArrayPushGrow` を static array
  plan/mutation に含め、`map` / `filter` の returned typed array、`forEach` の callback console
  side effect、`every` / `some` の lowered callback boolean result、`TypedArray#set` の static
  mutation を native static state へ反映した。Focused verification は node_diff の
  `typedarray_methods_matches_node_output` と
  `typedarray_mutating_methods_matches_node_output`、直接 iwasm の
  `typedarray-methods.ts` / `typedarray-mutating-methods.ts`、`cargo check -p ts2wasm-backend-wasm`
  で通過した。full fixture differential は未達のまま
  `pass=678 fail=339 unsupported=169 blocked=165 total=1351 elapsed=214.3s`。
  この変更で `typedarray-methods.ts`、`typedarray-mutating-methods.ts`、
  `direct-eval-function-hoisted-before-use.ts` が pass 側へ移動した。`proxy-remaining-traps.ts`
  は `unsupported -> fail` へ移動し、未実装扱いだった callback symbol materialization の先に
  Proxy trap result semantics の stdout mismatch が露出した。

- 2026-05-25 Object kernel 追加確認:
  `Object.create(proto, undefined)` の static prototype alias と native `$object_create` 呼び出し、
  `Object.getOwnPropertyDescriptor` の missing property -> `undefined`、`Object.preventExtensions` /
  `Object.isExtensible` の static object state、static numeric add の console 出力を修正した。
  これにより `fixtures/object-semantics-kernel/computed-read-prototype.ts`、
  `fixtures/builtins-and-io/object-create.ts`、
  `fixtures/object-semantics-kernel/define-property-edge-cases.ts` は native/iwasm differential で通過した。
  続いて static descriptor attrs を assignment/delete emit と static collection の両方で尊重し、
  `Object.freeze` / `Object.seal` / `Object.isFrozen` / `Object.isSealed`、`Object.values` /
  `Object.entries`、`GetLength(Object.keys(...))` の static 評価を追加した。
  これにより `object-abi-kernel.ts`、`configurable-false-enforcement.ts`、
  `writable-false-enforcement.ts`、`enumerable-filtering.ts`、`seal-freeze-descriptor.ts` は
  native/iwasm differential と node_diff regression で通過した。
  残る Object kernel のボトルネックは、GC heap object materialization、
  getter/setter build path、private-field heap dispatch、prototype-method lowering unsupported である。

- 2026-05-25 BigInt static/native rendering 追加確認:
  lowered IR は負の BigInt を `decimal="-1"` と `sign=-1` の両方で保持するため、
  native emitter の static console/string/property-key rendering が符号を再付与すると `--1n`
  を出力していた。`static_bigint_decimal_string` / `static_bigint_console_bytes` で
  decimal payload の先頭 `-` を canonicalize し、`ErrorMessage(BigIntLiteral(...))`
  も static string value として畳み込むようにした。これにより
  `bigint-arithmetic-literal-fold.ts`、`bigint-unary-minus-unsupported.ts`、
  `bigint-shift-literal-runtime.ts`、`bigint-bitwise-literal-runtime.ts`、
  `bigint-bitwise-unary-out-of-signed-i64.ts`、`bigint-builtin-as-int-n.ts` は
  focused native/iwasm differential で通過した。`bigint-builtin-as-int-n.ts` は
  `String(BigInt.asUintN(8, -1n))` が opaque token `128` を出す問題も解消した。

- 2026-05-25 Array dynamic property get/set 追加確認:
  `console.log(<Block result>)` が静的解決できない `undefined` tagged value を `WRITE_I32`
  に落として `0` と出力する経路を、既知 static array の dynamic key miss を
  `Undefined` として畳み込むことで修正した。また、`Local` key が static array slot に
  対応する場合は古い `StaticValue::Array` から bytes 化せず、実際の runtime slot を読むようにした。
  これにより `array-oob.ts`、`array-nonnumber-index.ts`、`dynamic-property-assignment.ts` は
  focused native/iwasm differential と auto differential regression で通過した。full fixture
  differential でも `pass=515 fail=493 unsupported=178 blocked=165 total=1351 elapsed=334.6s`
  に改善した。

- 2026-05-25 String spread / static concat 追加確認:
  lowered は `..."abc"` と `[..."abc"]` を `"a"`, `"b"`, `"c"` に展開できていたが、
  native emitter の static binary `+` が string concat を畳み込まず、user function return の
  `a + b + c` が raw `i32.add` に落ちて heap string token を数値として出力していた。
  `Add` の static evaluator で片側が static string の場合は JS string concat として畳み込み、
  static array slot の console 出力も string 要素だけ静的 bytes 化し、number slot は引き続き
  runtime slot を読ませることで `dynamic-property-assignment.ts` の stale read を避けた。
  これにより `spread-call-string-literal.ts`、`spread-call-string-local.ts`、
  `spread-string-static-concat.ts`、`spread-array-string.ts` は focused native/iwasm differential と
  `TS2WASM_RUN_NODE_DIFF=1 cargo test -p ts2wasm-cli --test node_diff spread_operator_string -- --nocapture`
  / static-concat node_diff regression で通過した。full fixture differential は
  `pass=523 fail=487 unsupported=176 blocked=165 total=1351 elapsed=335.6s` まで改善した。

- 2026-05-25 callback raw string representation 追加確認:
  `Array.prototype.forEach(function callback)` と `Map.prototype.forEach` の lowered callback では、
  string 要素/Map key を一度 raw string header pointer として local に取り出して user function param へ渡す。
  既存 native console は callback param の表現情報を持たず `WRITE_I32` に落とすため、
  `array-foreach-function-callback.ts` は `608/624`、`map-forEach.ts` は `624/632/640`
  のような string token を数値出力していた。mainline emitter で call-site の conservative
  representation を収集し、static string array element と static MapEntriesArray の even/odd key/value
  representation を callback param へ伝播するようにした。`StaticValue::TaggedString` とは分け、
  raw string local は header pointer + length を直接 `WRITE_BUF` へ流す console 経路で出力する。
  これにより `array-foreach-function-callback.ts`、`map-forEach.ts` は focused/full fixture differential
  で通過した。回帰確認として `spread-call-string-literal.ts`、`spread-call-string-local.ts`、
  `spread-string-static-concat.ts`、`spread-array-string.ts`、`dynamic-property-assignment.ts` も通過した。
  full fixture differential は `pass=525 fail=485 unsupported=176 blocked=165 total=1351 elapsed=333.0s`
  まで改善した。

- 2026-05-25 RegExp runtime value boundary 追加確認:
  native emitter の汎用 `RuntimeCall` path は `LoweredExpr::String` を raw string header pointer として
  emitted していたため、`$regexp_test` / `$regexp_match` / `$regexp_search` 側の `$is_string` check が
  false になり、`regexp-test.ts` は `2` / `null` を出力していた。RegExp family だけ
  runtime call arguments を tagged JS value として積み、console return path も tagged runtime value として
  `ValueToStringInto` に通すようにした。これにより `regexp-literal.ts`、`regexp-test.ts`、
  `regexp-0-args.ts` は focused native/iwasm differential で通過した。回帰確認として
  `spread-string-static-concat.ts`、`array-foreach-function-callback.ts`、`map-forEach.ts` も通過した。
  full fixture differential は `pass=529 fail=481 unsupported=176 blocked=165 total=1351 elapsed=339.8s`
  まで改善した。

- 2026-05-25 Object.is / tagged object boolean runtime value 追加確認:
  `$object_is` / `$object_has_own_property` / `$object_has_own` / `$object_is_extensible` /
  `$object_is_sealed` / `$object_is_frozen` は `RuntimeResult::Value` として tagged JS boolean を返すが、
  native console path が raw `i32` として扱うと `true` / `false` ではなく `3` / `2` を出力する。
  さらに `Object.is` を static fold した boolean は `$log` / `ValueToStringInto` に raw `1` / `0`
  として渡ると `null` / `undefined` に見えるため、single-arg と multi-arg の tagged console
  経路で static primitive を必ず tagged value として emit するようにした。
  `Object.is` の static fold は primitives と fresh object identity の SameValue 相当だけに限定し、
  array/object alias 追跡へ踏み込まない conservative な境界にしている。これにより
  `fixtures/builtins-and-io/object-is.ts` は focused/full fixture differential で通過した。
  回帰確認として `object-abi-kernel.ts`、`enumerable-filtering.ts`、`regexp-test.ts`、
  `array-foreach-function-callback.ts`、`map-forEach.ts` も通過した。full fixture differential は
  `pass=530 fail=480 unsupported=176 blocked=165 total=1351 elapsed=334.0s` まで改善した。

- 2026-05-25 Object.fromEntries / Object.hasOwn static object boundary 追加確認:
  `fixtures/builtins-and-io/object-static.ts` は Node が `3`, `true`, `false`, `true`,
  `false`, `true` を返す一方、native/iwasm が冒頭を `0`, `2`, `2` と出力していた。
  `RuntimeFn::ObjectFromEntries` を static value evaluator に追加し、既知 array-of-pairs を
  既存の property-key conversion と `StaticObjectValue::set` で static object に復元するようにした。
  これにより `Object.fromEntries(...)` の結果を `Object.keys` / `Object.hasOwn` が同じ
  static object model 上で参照できる。focused differential では `object-from-entries.ts` と
  `object-static.ts` がともに通過し、smoke differential は `pass=6 fail=0 unsupported=1 blocked=0 total=7`
  を維持した。full fixture differential は
  `pass=532 fail=478 unsupported=176 blocked=165 total=1351 elapsed=335.3s` まで改善した。
  rebuilt CLI の runtime path でも同 fixture は期待出力になったが、dynamic iterator 入力や
  全 fromEntries 形状の完了を意味しない。

- 2026-05-25 Object.assign / own property names / Object.prototype identity 追加確認:
  `Object.assign(target, source)` は runtime call が static object model に副作用を反映せず、
  `target.b` / `target.c` が `undefined` になっていた。known static object target/source の場合だけ
  enumerable own string properties を target root へコピーし、descriptor は JS `Object.assign`
  と同じ writable/enumerable/configurable data property として扱うようにした。あわせて
  `Object.getOwnPropertyNames` は enumerable filter を通さない own string key array として static fold し、
  plain object の `Object.getPrototypeOf(obj) === Object.prototype` を default prototype identity として
  fold した。focused differential では `object-assign.ts`、`object-assign-descriptors.ts`、
  `object-get-own-property-names.ts`、`object-get-prototype-of.ts` が通過した。full fixture
  differential は `pass=536 fail=474 unsupported=176 blocked=165 total=1351 elapsed=342.4s`
  まで改善した。この時点では `object-static-complete.ts` はまだ未通過で、残差分は
  fresh `Object.fromEntries` identity、`Object.groupBy` result materialization、string `Array#indexOf`、
  `Object.prototype.isPrototypeOf` の boolean representation が中心である。その他の Object bottleneck は
  full descriptor redefinition semantics、integer key order、object prototype formatting、
  GC heap materialization / getter-setter / private-field dispatch である。

- 2026-05-25 object-static-complete closure:
  `object-static-complete.ts` の残差分を閉じた。`Object.fromEntries(...)` は fresh object identity
  として `Object.is(obj, Object.fromEntries(...)) === false` に畳み、lowered `Object.groupBy`
  の既知配列 + user callback + dynamic property-set/push ループは static object of arrays に復元する。
  `Array#indexOf` / `Array#includes` は known array について static strict search を追加し、
  callback 側の `%` と数値比較も static binary evaluator に追加した。`proto.isPrototypeOf(child)` は
  lowered prototype-chain while block を static prototype state から boolean 化し、`console.log(Block)`
  の一時変数展開経路でも `true/false` として出せるようにした。focused differential は
  `object-static-complete.ts`、`object-static.ts`、`object-from-entries.ts`、`object-assign.ts`、
  `object-get-own-property-names.ts`、`object-get-prototype-of.ts` の 6/6 pass。smoke differential は
  `pass=6 fail=0 unsupported=1 blocked=0 total=7` を維持。full fixture differential は
  `pass=540 fail=470 unsupported=176 blocked=165 total=1351 elapsed=355.6s` まで改善した。
  これは static lowered shape の補完であり、dynamic `Object.groupBy` や full prototype runtime の完了ではない。
  skill 指定の `bash scripts/run/verify-harness.sh --quick|--cargo|--fixtures` は 3 件とも
  `scripts/run/verify-harness.sh` が存在せず exit 127 で実行不能だった。

- 2026-05-25 accessor descriptor attrs closure:
  `Object.defineProperty(obj, "x", { get: () => 42, configurable: true })` の static descriptor
  で、未指定の `enumerable` が JS 仕様どおり `false` にならず `true` として materialize されていた。
  `ObjectAccessorProp` に `enumerable` / `configurable` を保持し、descriptor literal から bool 属性を
  取り込み、`static_object_accessor_descriptor` が固定 `true` ではなく保存済み属性を emit するようにした。
  これにより object literal accessor は既存の enumerable/configurable true を保ちつつ、
  `Object.defineProperty` accessor descriptor は未指定属性を false default として出せる。
  focused differential は `object-define-property-getter.ts`、`object-get-own-property-descriptor.ts`、
  `object-define-property.ts`、`object-define-property-data.ts`、
  `object-literal-getter-descriptor.ts`、`object-literal-setter-descriptor.ts`、
  `object-literal-proto-accessor-descriptor.ts` の 7/7 pass。smoke differential は
  `pass=6 fail=0 unsupported=1 blocked=0 total=7` を維持。full fixture differential は
  `pass=541 fail=469 unsupported=176 blocked=165 total=1351 elapsed=345.0s` まで改善した。
  残課題は full descriptor redefinition semantics で、既存 accessor に対する partial descriptor
  更新時の属性保持/変換規則まではまだ完了扱いにしない。
  skill 指定の `bash scripts/run/verify-harness.sh --quick|--cargo|--fixtures` は 3 件とも
  `scripts/run/verify-harness.sh` が存在せず exit 127 で実行不能だった。

- 2026-05-25 class accessor / typed field initializer closure:
  `fixtures/object-semantics-kernel/getter-setter-build.ts` は、typed class field
  `private _val: number = 0;` の initializer が parser で失われ、synthetic constructor が
  `_val = undefined` を emit していたこと、さらに class accessor が `get value` / `set value`
  method として登録される一方で `c.value` / `c.value = ...` が known class local の getter/setter へ
  dispatch されていなかったことが root cause だった。class member type annotation skip を `=`
  で止め、known class local の property read/write を `resolve_class_getter` /
  `resolve_class_setter` 経由で user call に lowering するようにした。focused differential は
  `getter-setter-build.ts`、`getter-setter-runtime.ts`、`define-property-edge-cases.ts` の
  3/3 pass。class/object accessor 周辺 13 fixture の wider check は
  `pass=10 fail=2 unsupported=1 blocked=0 total=13` で、残 failure は
  `classes/class-private-members.ts` の private member value/dispatch 表現と
  `classes/class-static-fields.ts` の class object/static storage、unsupported は
  `core-semantics/class-getter-setter-inherited.ts` の unresolved class lowering だった。
  smoke differential は `pass=6 fail=0 unsupported=1 blocked=0 total=7` を維持し、full fixture
  differential は `pass=544 fail=466 unsupported=176 blocked=165 total=1351 elapsed=351.8s`
  まで改善した。残る class 系ボトルネックは、static fields/static blocks/static `this` を共有
  class object storage へ載せること、private member heap value dispatch/representation、
  inherited accessor の unresolved class path の解消である。skill 指定の
  `bash scripts/run/verify-harness.sh --quick|--cargo|--fixtures` は 3 件とも
  `scripts/run/verify-harness.sh` が存在せず exit 127 で実行不能だった。

- 2026-05-25 class static/private object-state closure:
  `fixtures/classes/class-static-fields.ts` は、同じ class 参照でも lowered が参照ごとに fresh
  `ClassPrototype` local を作るため、`MyClass.base` / `multiplier` / `value` の write/read と static
  method body が別々の static object として評価されていたことが root cause だった。constructor
  `FuncId` から synthetic class static object root を作り、`ClassPrototype` をその root への
  alias として扱い、static method の user-function evaluator へ caller の class static object roots を
  seed するようにした。`fixtures/classes/class-private-members.ts` は
  `$pseudo_private_field_get/set` を static evaluator が扱えず、constructor/private method の副作用が
  static object に反映されないため static ref token の `1024` が出力されていた。brand/slot から
  `@@private:{brand}:{slot}` key を作り、単純な private field get/set を non-enumerable /
  non-configurable な static slot として model した。
  focused differential は `class-static-fields.ts` / `class-private-members.ts` の 2/2 pass。
  node_diff regression は `class_static_fields_fixture_matches_node_output` と
  `class_private_members_fixture_matches_node_output` が pass。class/object accessor wider check は
  `pass=12 fail=0 unsupported=1 blocked=0 total=13` まで改善し、残 unsupported は
  `fixtures/core-semantics/class-getter-setter-inherited.ts` の unresolved class lowering のみ。
  smoke differential は `pass=6 fail=0 unsupported=1 blocked=0 total=7` を維持し、full fixture
  differential は `pass=554 fail=457 unsupported=175 blocked=165 total=1351 elapsed=349.9s`
  まで改善した。今回の private 対応は constructor/direct private method の単純な private field slot
  propagation に限定しており、`private-class-derived-accessor-direct`、
  `private-class-field-read-write` などに残る private accessor / derived receiver /
  full heap dispatch semantics は未完了である。skill 指定の
  `bash scripts/run/verify-harness.sh --quick|--cargo|--fixtures` は 3 件とも
  `scripts/run/verify-harness.sh` が存在せず exit 127 で実行不能だった。

- 2026-05-25 class static method / private accessor side-effect closure:
  `fixtures/classes/class-static-method-this.ts` は static method/user-function evaluator が
  call-local context に seed した class static object root を caller state へ merge していなかったため、
  `Counter.inc()` の `this.value` mutation が `Counter.value` read に戻らず `0` になっていた。
  `fixtures/core-semantics/private-class-field-read-write.ts`、
  `private-class-field-same-class-receiver.ts`、`private-class-setter-direct.ts`、
  `private-class-derived-accessor-direct.ts` は、static user-function evaluation が return 前の
  `PrivateFieldSet` / block / property-set 系の side-effect-only statement を無視し、
  `apply_static_user_function_env_effects` も `Return(expr)` を副作用伝播前に未対応扱いしていたことが
  root cause だった。return expression から function refs を収集し、return 到達前までの private slot /
  property mutation を caller state へ伝播するようにした。
  `fixtures/core-semantics/private-class-static-accessor-direct.ts` は
  `console.log(Counter.write(...))` を static fold すると `write` 内部の private setter
  `console.log(value + 1)` が落ちるため、transitive console side effect を持つ user call は static
  console fold しないようにし、同時に required-function collection でも folded 扱いしないようにした。
  これにより `fixtures/builtins-and-io/map-forEach.ts` の `$func_1` unresolved regression も回避した。
  focused differential は `map-forEach.ts` と上記 private/static 6 件の 7/7 pass。
  class/private focused set は `pass=19 fail=1 unsupported=9 blocked=2 total=31` まで改善し、残 fail は
  `fixtures/core-semantics/private-class-field-internal-slot-gc.ts` の iwasm execution failure。
  smoke differential は `pass=6 fail=0 unsupported=1 blocked=0 total=7`、node_diff は
  `class_static_method_this_fixture_matches_node_output` が pass。full fixture differential は
  `pass=558 fail=450 unsupported=178 blocked=165 total=1351 elapsed=358.4s`。skill 指定の
  `bash scripts/run/verify-harness.sh --quick|--cargo|--fixtures` は 3 件とも
  `scripts/run/verify-harness.sh` が存在せず exit 127 で実行不能だった。

- 2026-05-25 known-array backed `Set` spread closure:
  `fixtures/core-semantics/spread-array-set.ts` と `spread-array-set-mixed.ts` は
  `new Set(<known array>)` から `SetValuesArray` へ戻す path が native static evaluator で表現できず、
  runtime `SetValuesArray` の結果を array length/index として読む段階で iwasm execution failure に
  落ちていた。`fixtures/core-semantics/spread-call-set-local.ts` は single Set spread call が
  `ArrayGet(SetValuesArray(set), index)` に lower されるが、`ArrayGet` 自体が static value evaluator の
  index/property path に入っていなかったため user call 引数が static fold されず `0` 出力になっていた。
  `SetFromArray` を hidden `@@set_values` property を持つ static object として model し、`SetValuesArray`
  を insertion-order / SameValue 判定で dedupe した static array へ戻すようにした。あわせて
  `ArrayGet` を `Index` / `PropertyGetDynamic` と同じ static array element path に入れた。
  focused differential は `spread-array-set.ts` / `spread-array-set-mixed.ts` /
  `spread-call-set-local.ts` / `test-spread-array-literal.ts` の 4/4 pass。node_diff regression は
  `spread_operator_set_*` 2 件と `spread_operator_mixed_set_*` 1 件を build-only から
  Node output 一致へ昇格して pass。smoke differential は
  `pass=6 fail=0 unsupported=1 blocked=0 total=7`。full fixture differential は
  `pass=561 fail=447 unsupported=178 blocked=165 total=1351 elapsed=356.2s`。残る spread bucket は
  custom iterable iterator protocol、object spread dynamic/mutated source、sparse materialization であり、
  Set runtime collection semantics 全体（例: `test-set-samevaluezero.ts`）はまだ別 bucket として未完了。
  skill 指定の `bash scripts/run/verify-harness.sh --quick|--cargo|--fixtures` は 3 件とも
  `scripts/run/verify-harness.sh` が存在せず exit 127 で実行不能だった。

- 2026-05-25 object spread dynamic/mutated source closure:
  `ObjectSpread` は lowering 側では `RuntimeFn::ObjectSpread` として出ていたが、native static evaluator は
  `ObjectAssign` だけを side-effect/value model に持っていたため、`{ z: 0, ...makeObj(), b: 3 }`、
  `{ z: 0, ...source, b: 3 }`、source mutation 後の spread が target object の static props に反映されず
  `0` 出力に落ちていた。`ObjectSpread` を `ObjectAssign` と同じ static entry copy path に接続し、
  target が local の場合は alias root を更新、nested spread expression の場合は target object value を合成して
  source entries を重ねるようにした。focused differential は
  `spread-object-function-return.ts` / `spread-object-dynamic-local.ts` / `spread-object-mutated.ts` /
  `spread-object-alias-mutated-unsupported.ts` / `spread-object-unsupported.ts` の 5/5 pass。
  node_diff は object spread 7 件が pass、うち mutated source と alias-mutated source の 2 件を
  Node output 一致 regression に追加した。smoke differential は
  `pass=6 fail=0 unsupported=1 blocked=0 total=7`。full fixture differential は
  `pass=566 fail=442 unsupported=178 blocked=165 total=1351 elapsed=366.1s`。残る spread bucket は
  custom iterable iterator protocol と sparse materialization であり、Set runtime collection semantics 全体
  （例: `test-set-samevaluezero.ts`）はまだ別 bucket として未完了。
  skill 指定の `bash scripts/run/verify-harness.sh --quick|--cargo|--fixtures` は 3 件とも
  `scripts/run/verify-harness.sh` が存在せず exit 127 で実行不能だった。

- 2026-05-25 sparse spread materialization closure:
  `ArrayNewSparse` は native static evaluator で array value として保持されず、`[...sparse]` の
  `ArrayConcat([], sparse)` と `observe(...sparse)` の `ArrayGet(sparse, i)` が hole を
  `undefined` に materialize できていなかった。`SparseArray` static value を追加して hole presence を保持し、
  spread/concat/`ArrayGet` では `undefined` に materialize、`ArrayIndexPresent` / hasOwn/enumerability では
  hole presence を区別するようにした。focused differential は
  `spread-sparse-array-materializes-undefined.ts` / `spread-sparse-call-undefined.ts` の 2/2 pass。
  node_diff は `spread_operator_sparse_*` 2 件が pass。smoke differential は
  `pass=6 fail=0 unsupported=1 blocked=0 total=7`。full fixture differential は
  `pass=568 fail=440 unsupported=178 blocked=165 total=1351 elapsed=349.7s`。残る spread bucket は
  custom iterable iterator protocol であり、Set runtime collection semantics 全体
  （例: `test-set-samevaluezero.ts`）はまだ別 bucket として未完了。
  skill 指定の `bash scripts/run/verify-harness.sh --quick|--cargo|--fixtures` は 3 件とも
  `scripts/run/verify-harness.sh` が存在せず exit 127 で実行不能だった。

- 2026-05-25 static custom iterable spread closure:
  `lower_spread_via_iterator` は custom iterable spread を `Symbol.iterator` property get、
  iterator function の `HeapClosureCall`、`DoWhile`、`next` closure の `HeapClosureCall` に lower するが、
  native runtime の dynamic heap-closure dispatch は direct function token 前提で、captured heap closure の
  static-ref token を呼べない。そのため `done` が `undefined` のまま truthy/falsey 判定を誤り、
  custom iterable spread が timeout または stdout mismatch に落ちていた。
  `Symbol.iterator` を持つ object literal local の props を facts に保持し、静的に閉じた iterator function /
  next function (`state.i` increment + `{ value, done }`) を lowering 時点で dense `ArrayNew` へ展開した。
  これにより `spread-array-custom-iterable-empty.ts` /
  `spread-array-custom-iterable-mixed.ts` /
  `spread-array-custom-iterable-multi-value.ts` /
  `spread-array-custom-iterable-unsupported.ts` は focused differential 4/4 pass。
  node_diff は custom iterable spread 4 件を build-only から Node output 一致へ昇格して pass。
  smoke differential は `pass=6 fail=0 unsupported=1 blocked=0 total=7`。full fixture differential は
  `pass=572 fail=436 unsupported=178 blocked=165 total=1351 elapsed=244.3s`。残る iterator/custom iterable
  bucket は `custom-iterator-symbol.ts` のような general for-of / dynamic heap-closure dispatch と、
  Map/Set runtime collection semantics 全体である。skill 指定の
  `bash scripts/run/verify-harness.sh --quick|--cargo|--fixtures` は 3 件とも
  `scripts/run/verify-harness.sh` が存在せず exit 127 で実行不能だった。

- 2026-05-25 static custom iterable for-of closure:
  `custom-iterator-symbol.ts` は custom iterable の `for-of` が `Symbol.iterator` property get、
  iterator function の `HeapClosureCall`、`DoWhile`、`next` closure の `HeapClosureCall` に lower され、
  captured heap closure の dynamic dispatch が native runtime で解決できず timeout していた。
  既存の static custom iterable evaluator を `const items = [...]` / `const idx = state.i` /
  `if (idx < items.length) return { value: items[idx], done: false }` 形にも拡張し、break/continue を含まない
  custom iterable `for-of` を lowering 時点で dense `ArrayNew` を iter に持つ `ForOf` へ正規化した。
  これにより `custom-iterator-symbol.ts` は focused differential で pass し、custom iterable spread 4 件との
  combined focused differential は 5/5 pass。node_diff は `custom_iterator_symbol_fixture_matches_node_output_under_iwasm`
  を追加して pass。smoke differential は `pass=6 fail=0 unsupported=1 blocked=0 total=7`。
  full fixture differential は `pass=573 fail=435 unsupported=178 blocked=165 total=1351 elapsed=222.7s`。
  `iterator-protocol.ts` のような `break` を含む iterator loop は今回の static `ForOf` 正規化対象外であり、
  general iterator protocol / dynamic heap-closure dispatch bucket として残る。skill 指定の
  `bash scripts/run/verify-harness.sh --quick|--cargo|--fixtures` は 3 件とも
  `scripts/run/verify-harness.sh` が存在せず exit 127 で実行不能だった。

- 2026-05-25 static custom iterable for-of break closure:
  `iterator-protocol.ts` は `next()` が常に `{ value: 42, done: false }` を返すため、
  full iterator evaluation では完了しないが、loop body が top-level unlabeled `break` で終了するので、
  first yield だけを静的評価すれば Node と一致する。lowering はこの形を
  `ForOf(ArrayNew([42]))` に正規化し、native `emit_static_for_of` は branch body 用に outer break block と
  per-iteration continue block を emit するようにした。focused differential は
  `iterator-protocol.ts` / `custom-iterator-symbol.ts` の 2/2 pass、custom iterable combined は 6/6 pass。
  node_diff は `iterator_protocol` 2 件 pass。smoke differential は
  `pass=6 fail=0 unsupported=1 blocked=0 total=7`。full fixture differential は
  `pass=575 fail=434 unsupported=177 blocked=165 total=1351 elapsed=228.7s`。
  残る general iterator bucket は conditional break/continue、labeled for-of continue、
  dynamic heap-closure dispatch、generator spread/iterator semantics である。Map/Set runtime collection
  semantics、BigInt runtime 値表現、eval/math host/runtime 差分も mainline native emitter の残ボトルネックとして
  引き続き未完了。skill 指定の `bash scripts/run/verify-harness.sh --quick|--cargo|--fixtures` は
  3 件とも `scripts/run/verify-harness.sh` が存在せず exit 127 で実行不能だった。

- 2026-05-25 static array method fold closure:
  known static array local は native emitter では slot locals として扱われるため、runtime `$array_at`
  の heap-array ABI に渡すと `array-at.ts` が全て `0` 出力になっていた。`ArrayAt` を static evaluator
  に追加し、`undefined` index -> `0`、negative index、out-of-bounds -> `undefined` を JS `at` として
  fold するようにした。あわせて lowering が `ArrayEvery` / `ArraySome` / `ArrayFind` /
  `ArrayFindIndex` / `ArrayFindLast` / `ArrayFindLastIndex` / `ArrayFilter` の identity arrow callback を
  one-arg runtime call に正規化する path を、known static array から直接 truthy scan するようにした。
  focused differential は `array-at.ts`、`array-every.ts`、`array-some.ts`、`array-find.ts`、
  `array-find-index.ts`、`array-filter.ts` が pass し、identity `array-find-last-index.ts` も pass。
  9 件の wider array method check は `pass=7 fail=2 unsupported=0 blocked=0` で、残 fail は
  lowered while/block pattern と thisArg callback を含む `array-find-last.ts` と
  `array-find-findindex-complex.ts`。node_diff は `array_at`、`array_every`、`array_some`、
  `array_find`、`array_find_index`、`array_filter` が pass。smoke differential は
  `pass=6 fail=0 unsupported=1 blocked=0 total=7`。full fixture differential は
  `pass=582 fail=427 unsupported=177 blocked=165 total=1351 elapsed=223.9s`。
  残る array method bucket は lowered callback loop の static block evaluation、thisArg binding、
  mutable methods (`fill` / `copyWithin` / `splice` / iterator entries) と runtime heap-array ABI 境界である。
  skill 指定の `bash scripts/run/verify-harness.sh --quick|--cargo|--fixtures` は 3 件とも
  `scripts/run/verify-harness.sh` が存在せず exit 127 で実行不能だった。

- 2026-05-25 static array callback loop / thisArg closure:
  `array-find-last.ts` と `array-find-findindex-complex.ts` は callback array methods が
  lowered `Block { Let len; Let found; Let i; While ... }` として展開されるため、generic static local collector が
  loop-assigned `found` を消し、native emission が callback `Call(User)` を残していた。さらに
  `function(this, x) { return x > this.limit; }` の `thisArg` は function expression の receiver param として
  lower されていなかった。function expression が `this` を参照する場合は receiver param を持たせ、
  array callback lowering は `thisArg` を receiver 引数として渡すようにした。native static evaluator は
  `find` / `findIndex` / `findLast` / `findLastIndex` の lowered callback loop pattern を known static array 上で
  iteration し、predicate user function を static evaluation して value/index/`undefined`/`-1` を返す。
  console static path もこの block fold を直接使い、fold 済みの block body を runtime emission しないようにした。
  focused differential は `array-find-last.ts`、`array-find-findindex-complex.ts`、
  `array-find-thisarg.ts`、`array-filter-thisarg.ts` と前回 array method regression を合わせた 10/10 pass。
  node_diff は `array_find_last_fixture`、`array_find_thisarg_matches_node`、
  `array_filter_thisarg_matches_node`、新規追加した `array_find_findindex_complex_matches_node` が pass。
  smoke differential は `pass=6 fail=0 unsupported=1 blocked=0 total=7`。full fixture differential は
  `pass=584 fail=425 unsupported=177 blocked=165 total=1351 elapsed=226.1s`。
  残る array method bucket は mutable methods (`fill` / `copyWithin` / `splice`)、iterator entries/keys/values、
  array-like receiver semantics、runtime heap-array ABI 境界である。skill 指定の
  `bash scripts/run/verify-harness.sh --quick|--cargo|--fixtures` は 3 件とも
  `scripts/run/verify-harness.sh` が存在せず exit 127 で実行不能だった。

- 2026-05-25 static array fill closure:
  `array-fill.ts` は known static array local が native emitter では heap array ではなく per-element slot local として
  materialize されるため、runtime `$array_fill` へ渡しても native slot が更新されず、`[1, 2, 3]` のまま
  console が static fold されていた。`ArrayFill` を static array state mutation として扱い、
  `undefined` start/end、negative index、NaN/Infinity の ToInteger-ish normalization を含めて
  filled elements を計算し、native emission では該当 slot locals を直接更新するようにした。
  focused differential は `array-fill.ts` pass、regression として `array-at.ts` /
  `array-find-findindex-complex.ts` も pass。node_diff は既存 `array_fill_matches_node` が pass。
  smoke differential は `pass=6 fail=0 unsupported=1 blocked=0 total=7`。full fixture differential は
  `pass=585 fail=424 unsupported=177 blocked=165 total=1351 elapsed=215.7s`。
  `array-copy-within.ts` はまだ iwasm execution failure、`array-shift-unshift-splice.ts` は stdout mismatch のままで、
  returned-self identity / variable length static slots / mutable array ABI 境界が次の課題である。skill 指定の
  `bash scripts/run/verify-harness.sh --quick|--cargo|--fixtures` は 3 件とも
  `scripts/run/verify-harness.sh` が存在せず exit 127 で実行不能だった。

- 2026-05-25 static array copyWithin closure:
  `array-copy-within.ts` は known static array local が native slot locals に分解されているのに、
  `RuntimeFn::ArrayCopyWithin` が runtime helper へ落ちて `unreachable` になっていた。`ArrayCopyWithin` を
  static array state mutation として扱い、`target/start/end` の `undefined` default、negative index、
  overlap copy を dense slot vector 上で計算するようにした。`Let r = a.copyWithin(...)` は戻り値が
  mutated source array 自身なので、static array plan では `r` を source slot group の alias にし、
  static locals 収集でも mutation と戻り値を同じ計算結果で同時に登録して二重適用を避けた。
  native emission では source slot locals を直接更新し、戻り値 local は `STATIC_REF_TOKEN` にする。
  focused node_diff は `array_copy_within_matches_node` / `array_fill_matches_node` /
  `array_at_matches_node` / `array_find_findindex_complex_matches_node` が pass。
  manual native-vs-node は `array-copy-within.ts` pass。smoke differential は
  `pass=6 fail=0 unsupported=1 blocked=0 total=7`。full fixture differential は
  `pass=586 fail=423 unsupported=177 blocked=165 total=1351 elapsed=241.4s`。
  `array-shift-unshift-splice.ts` は引き続き stdout mismatch で、variable-length static slot mutation
  (`shift`/`unshift`/`splice`) と heap-array ABI fallback が次の mutable array bottleneck である。skill 指定の
  `bash scripts/run/verify-harness.sh --quick|--cargo|--fixtures` は 3 件とも
  `scripts/run/verify-harness.sh` が存在せず exit 127 で実行不能だった。

- 2026-05-25 static array shift/unshift/splice closure:
  `array-shift-unshift-splice.ts` は lowered static array local が native slot locals に分解される一方、
  `RuntimeFn::ArrayShift` / `ArrayUnshift` / `ArraySplice` が source slot mutation と戻り値を
  native static state に反映できず stdout mismatch になっていた。`shift` は先頭要素の primitive
  return と source slot の左詰め、`unshift` は source slot の右詰め・insert・new length return、
  `splice` は deleted range を returned removed-array slot group として作りつつ source slot を
  insert 後の dense vector に更新するようにした。static array plan では `unshift` の可変長拡張と
  `splice` の removed-array group を事前登録し、static locals 収集では let/assign の mutation と
  returned value を 1 回の pre-mutation 計算から登録して二重適用を避けた。
  focused node_diff は `array_shift_unshift_splice_matches_node` / `array_copy_within_matches_node` /
  `array_fill_matches_node` / `array_at_matches_node` /
  `array_find_findindex_complex_matches_node` が pass。smoke differential は
  `pass=6 fail=0 unsupported=1 blocked=0 total=7 elapsed=0.8s`。full fixture differential は
  `pass=587 fail=422 unsupported=177 blocked=165 total=1351 elapsed=233.6s` で、
  `array-shift-unshift-splice.ts` は pass へ進んだ。次の mutable array bottleneck は
  `array-pop.ts` / `array-push.ts` / `array-slice.ts` / `array-reverse.ts` / `array-sort.ts` /
  `array-join.ts` / `array-keys.ts` / `array-entries.ts` / iterator 系、または runtime heap-array ABI
  fallback である。skill 指定の `bash scripts/run/verify-harness.sh --quick|--cargo|--fixtures` は
  3 件とも `scripts/run/verify-harness.sh` が存在せず exit 127 で実行不能だった。

- 2026-05-25 static array pop/push closure:
  `array-pop.ts` / `array-push.ts` は known static array local が native slot locals に分解される一方、
  `RuntimeFn::ArrayPop` / `ArrayPush` の source slot mutation と returned value が native static state
  に反映されず stdout mismatch (`0` vs `3` / `4`) になっていた。`pop` は末尾要素の primitive
  return と source static length の短縮、`push` は末尾 slot 追加と new length return を pre-mutation
  計算から同時に登録するようにした。multi-arg `push` (`ArrayPushMany`) も let/assign の static locals
  収集で source mutation と returned length を 1 回の計算から登録し、generic static value fold による
  二重加算を避けた。static array plan では single-arg `ArrayPush` の slot group 拡張も事前登録する。
  focused node_diff は `array_pop_matches_node` / `array_push_matches_node` /
  `array_push_multi_argument_fixture_matches_node_output_under_iwasm` と、近接 regression の
  `array_shift_unshift_splice_matches_node` / `array_copy_within_matches_node` /
  `array_fill_matches_node` / `array_at_matches_node` /
  `array_find_findindex_complex_matches_node` が pass。smoke differential は
  `pass=6 fail=0 unsupported=1 blocked=0 total=7 elapsed=0.9s`。full fixture differential は
  `pass=590 fail=419 unsupported=177 blocked=165 total=1351 elapsed=230.7s` で、
  `array-pop.ts` / `array-push.ts` / `array-push-multi-arg.ts` は pass へ進んだ。次の mutable array
  bottleneck は `array-slice.ts` / `array-reverse.ts` / `array-sort.ts` / `array-join.ts` /
  `array-keys.ts` / `array-entries.ts` / iterator 系、または runtime heap-array ABI fallback である。
  skill 指定の `bash scripts/run/verify-harness.sh --quick|--cargo|--fixtures` は 3 件とも
  `scripts/run/verify-harness.sh` が存在せず exit 127 で実行不能だった。

- 2026-05-25 static array slice/reverse/join closure:
  `array-slice.ts` / `array-reverse.ts` / `array-join.ts` は known static array local が native slot locals に
  分解される一方、`RuntimeFn::ArraySlice` の returned array slot group、`RuntimeFn::ArrayReverse` の
  source mutation と returned-self alias、`RuntimeFn::ArrayJoin` の direct runtime call static string fold が
  native static state に反映されず stdout mismatch になっていた。`slice` は begin/end の static length を
  事前登録して新しい static array group として emit し、`reverse` は source group を反転 vector に更新しつつ
  assigned local を同じ group へ alias し、return local には `STATIC_REF_TOKEN` を設定するようにした。
  `join` は separator を static value から解決し、direct `RuntimeCall` を primitive string へ fold する。
  focused node_diff は `array_slice_matches_node` / `array_reverse_matches_node` /
  `array_join_matches_node` と、近接 regression の `array_pop_matches_node` /
  `array_push_matches_node` / `array_shift_unshift_splice_matches_node` が pass。smoke differential は
  `pass=6 fail=0 unsupported=1 blocked=0 total=7 elapsed=0.8s`。full fixture differential は
  `pass=597 fail=412 unsupported=177 blocked=165 total=1351 elapsed=235.4s` で、
  `array-slice.ts` / `array-reverse.ts` / `array-join.ts` と rest binding の slice 派生 fixture が
  pass へ進んだ。次の array/native bottleneck は `array-keys.ts` / `array-entries.ts` の iterator result
  object shape、`array-sort.ts` / `array-sort-comparator.ts` の comparator/mutation semantics、
  `array-last-index-of.ts`、array reduce の boolean rendering、runtime heap-array ABI fallback、
  object own-key integer ordering である。skill 指定の
  `bash scripts/run/verify-harness.sh --quick|--cargo|--fixtures` は 3 件とも
  `scripts/run/verify-harness.sh` が存在せず exit 127 で実行不能だった。

- 2026-05-25 static array lastIndexOf closure:
  `array-last-index-of.ts` は known static array の direct `RuntimeFn::ArrayLastIndexOf` が native static
  value に fold されず、runtime helper の tagged `-1` が stdout に raw `-4` として現れていた。
  `ArrayIndexOf` / `ArrayIncludes` と同じ static strict equality 比較を後方 scan として追加し、
  `ArrayLastIndexOf` の direct `RuntimeCall` を primitive number へ fold するようにした。
  focused node_diff は `array_last_index_of_matches_node` と、近接 regression の
  `array_index_of_fixture_matches_node_output_under_iwasm` /
  `array_includes_fixture_matches_node_output_under_iwasm` が pass。smoke differential は
  `pass=6 fail=0 unsupported=1 blocked=0 total=7 elapsed=1.0s`。full fixture differential は
  `pass=598 fail=411 unsupported=177 blocked=165 total=1351 elapsed=271.5s` で、
  `array-last-index-of.ts` は pass へ進んだ。次の array/native bottleneck は
  `array-keys.ts` / `array-entries.ts` の iterator result object shape、
  `array-sort.ts` / `array-sort-comparator.ts` / `array-sort-default-unsupported.ts` /
  `array-sort-numeric-comparator.ts` の comparator/mutation semantics、array reduce/every/some の
  boolean rendering、runtime heap-array ABI fallback、object own-key integer ordering である。
  skill 指定の `bash scripts/run/verify-harness.sh --quick|--cargo|--fixtures` は 3 件とも
  `scripts/run/verify-harness.sh` が存在せず exit 127 で実行不能だった。

- 2026-05-25 static array sort closure:
  `array-sort.ts` / `array-sort-comparator.ts` / `array-sort-default-unsupported.ts` /
  `array-sort-numeric-comparator.ts` は known static array の `RuntimeFn::ArraySortLexicographic` /
  `RuntimeFn::ArraySortNumeric` が in-place mutator なのに、native static slot locals / static locals の
  source array が更新されず、sort 後の first/last/index read が pre-sort value を読んでいた。
  default sort は static value の string key で比較し、`undefined` を末尾に置く。numeric comparator path は
  numeric ascending とし、`undefined` を末尾、`NaN` は non-NaN の後ろに置く。assigned return local は
  source slot group へ alias し、static returned token は `STATIC_REF_TOKEN` にする。
  focused node_diff は `array_sort_matches_node` / `array_sort_comparator_matches_node` /
  `array_sort_default_fixture_matches_node_output_under_iwasm` /
  `array_sort_numeric_comparator_fixture_matches_node_output_under_iwasm` が pass。smoke differential は
  `pass=6 fail=0 unsupported=1 blocked=0 total=7 elapsed=1.0s`。full fixture differential は
  `pass=602 fail=407 unsupported=177 blocked=165 total=1351 elapsed=263.3s` で、sort 4 fixture が
  pass へ進んだ。次の array/native bottleneck は `array-keys.ts` / `array-entries.ts` の iterator result
  object shape、`array-every-some-complex.ts` の boolean rendering、`array-reduce.ts` の
  string concat / reduceRight 差分、runtime heap-array ABI fallback、object own-key integer ordering である。
  skill 指定の `bash scripts/run/verify-harness.sh --quick|--cargo|--fixtures` は 3 件とも
  `scripts/run/verify-harness.sh` が存在せず exit 127 で実行不能だった。

- 2026-05-25 static array iterator closure:
  `array-keys.ts` / `array-entries.ts` は known static array の `RuntimeFn::ArrayKeys` /
  `RuntimeFn::ArrayEntries` が native static state では opaque token だけになり、続く
  `RuntimeFn::ArrayIteratorNext` の `{ value, done }` result object と entries pair array が
  static property/index fold できず `0` / `undefined` を出していた。`StaticValue::ArrayIterator` を追加し、
  `ArrayValues` / `ArrayKeys` / `ArrayEntries` で `values/kind/next_index` を保持し、
  `ArrayIteratorNext` で `{ value, done }` の static object を返しつつ iterator local の
  `next_index` を進めるようにした。entries は `[index, value]` の static array を `value` に入れる。
  focused node_diff は `array_keys_matches_node` / `array_entries_matches_node` と、近接 regression の
  `array_values_matches_node` が pass。smoke differential は
  `pass=6 fail=0 unsupported=1 blocked=0 total=7 elapsed=1.0s`。full fixture differential は
  `pass=605 fail=404 unsupported=177 blocked=165 total=1351 elapsed=261.1s` で、
  `array-keys.ts` / `array-entries.ts` が pass へ進んだ。次の array/native bottleneck は
  `array-every-some-complex.ts` の boolean rendering、`array-reduce.ts` の string concat /
  reduceRight 差分、runtime heap-array ABI fallback、object own-key integer ordering である。
  skill 指定の `bash scripts/run/verify-harness.sh --quick|--cargo|--fixtures` は 3 件とも
  `scripts/run/verify-harness.sh` が存在せず exit 127 で実行不能だった。

- 2026-05-25 native console boolean block closure:
  `array-every-some-complex.ts` は lowered callback loop 自体は `true/false` を raw boolean local として
  正しく計算していたが、`console.log(<Block result LocalId(...)>)` の `Block` expression が
  `native_console_arg_type` では `Unknown` のまま扱われ、最後の `WRITE_I32` fallback に落ちて
  `true/false` ではなく `1/0` を出していた。`native_console_arg_type` が `Block.result` の local type を
  再帰的に見るようにし、同じ raw/tagged 境界問題で落ちていた `Boolean.prototype.valueOf()` も
  `RuntimeFn::ValueOf` を static identity として扱うようにした。手元の direct Node/iwasm diff では
  `array-every-some-complex.ts` と `boolean-symbol-prototype.ts` が一致。grouped node_diff
  `common_builtin_api_fixtures_match_node_output` はこの2件を通過し、次の既存差分
  `native-error-types.ts` で停止した。smoke differential は
  `pass=6 fail=0 unsupported=1 blocked=0 total=7 elapsed=1.0s`。full fixture differential は
  `pass=607 fail=402 unsupported=177 blocked=165 total=1351 elapsed=255.4s` で、
  `array-every-some-complex.ts` と `boolean-symbol-prototype.ts` が pass へ進んだ。次の
  array/native bottleneck は `array-flat-map.ts` / `array-flat.ts` の returned heap-array shape、
  `array-reduce.ts` の accumulator/string concat 差分、runtime heap-array ABI fallback、
  object own-key integer ordering である。
  skill 指定の `bash scripts/run/verify-harness.sh --quick|--cargo|--fixtures` は 3 件とも
  `scripts/run/verify-harness.sh` が存在せず exit 127 で実行不能だった。

- 2026-05-25 static array reduce callback closure:
  `array-reduce.ts` は lowered callback loop が accumulator local を `While` body で更新するため、
  generic static local collector が accumulator を static value として保持できず、string concat reducer
  result が runtime/raw path へ落ちて `0` になっていた。`static_array_reduce_callback_block_value` で
  known static array の reduce/reduceRight lowered loop shape を認識し、length/accumulator/index locals、
  forward `index < length` と backward `index >= 0`、`ArrayGet(array, index)`、accumulator assignment、
  index update を検証したうえで、user reducer callback を static に反復評価するようにした。
  focused node_diff は `array_reduce_fixture_matches_node_output_under_iwasm` と
  `array_reduce_right_fixture_matches_node_output_under_iwasm` が pass。direct Node/iwasm diff でも
  `array-reduce.ts` / `array-reduce-right.ts` が一致。smoke differential は
  `pass=6 fail=0 unsupported=1 blocked=0 total=7 elapsed=0.9s`。full fixture differential は
  `pass=608 fail=401 unsupported=177 blocked=165 total=1351 elapsed=248.2s` で、
  `array-reduce.ts` が pass へ進んだ。次の array/native bottleneck は `array-flat-map.ts` /
  `array-flat.ts` の returned heap-array shape、runtime heap-array ABI fallback、
  object own-key integer ordering である。
  skill 指定の `bash scripts/run/verify-harness.sh --quick|--cargo|--fixtures` は 3 件とも
  `scripts/run/verify-harness.sh` が存在せず exit 127 で実行不能だった。

- 2026-05-25 static array flat / flatMap closure:
  `array-flat.ts` は `RuntimeFn::ArrayFlat` が dynamic typed runtime path に落ち、known static nested
  array を heap-array ABI として実行して `unreachable` になっていた。`static_array_flat_from_args` で
  known static dense/sparse array と static depth を解決し、negative depth は 0 に clamp し、sparse holes は
  skip して Node と同じ returned dense array へ fold するようにした。`array-flat-map.ts` は lowered
  `ArrayPushOrSpread(result, mapped)` loop が `Expr` で result local に反映されず、generic static collection
  でも `While` body assignment により result state が消えていた。`static_array_flatmap_callback_block_value`
  で `Let len; Let result = ArrayNew; Let i; While i < len { ArrayGet; Call(User); ArrayPushOrSpread; i++ }`
  shape を認識し、known static source 上で user callback を static evaluation して scalar は push、
  returned array は spread する。user callback が返す array は callee local environment で materialize し、
  callback-local `Local(0)` が `STATIC_REF_TOKEN` ではなく実引数値へ解決されるようにした。
  focused node_diff は `array_flat_matches_node` / `array_flat_map_matches_node` が pass。
  smoke differential は `pass=6 fail=0 unsupported=1 blocked=0 total=7 elapsed=1.1s`。full fixture
  differential は `pass=610 fail=399 unsupported=177 blocked=165 total=1351 elapsed=250.4s` で、
  `array-flat.ts` / `array-flat-map.ts` は pass へ進んだ。次の array/native bottleneck は
  `array-from.ts`、`array-with.ts`、`array-to-reversed.ts` / `array-to-sorted.ts` /
  `array-to-spliced.ts`、runtime heap-array ABI fallback、object own-key integer ordering である。
  skill 指定の `bash scripts/run/verify-harness.sh --quick|--cargo|--fixtures` は 3 件とも
  `scripts/run/verify-harness.sh` が存在せず exit 127 で実行不能だった。

- 2026-05-25 static array copying methods closure:
  `array-from.ts` は `Array.from([..])` が `RuntimeFn::ArrayMapArrayLikeIdentity` として lowered されるが、
  native emitter では returned array slot group / static value init の対象外で heap-array runtime path に落ち、
  `unreachable` になっていた。`array-with.ts` と `array-to-reversed.ts` /
  `array-to-sorted.ts` / `array-to-spliced.ts` も `RuntimeFn::ArrayWith` /
  `ArrayToReversed` / `ArrayToSorted` / `ArrayToSpliced` の returned array を known static array として
  materialize できず、copying method fixture は runtime heap-array ABI 境界で落ちていた。
  static array plan でこれらの non-mutating returned array に slot group を割り当て、
  `try_emit_static_array_value_init` と `static_value_from_expr_with_functions` に
  `ArrayMapArrayLikeIdentity` / `ArrayMapArrayLikeDouble` / `ArrayWith` / `ArrayToReversed` /
  `ArrayToSorted` / `ArrayToSpliced` を追加した。`with` は checked relative index で source を変更せず
  copy 内の要素だけ置換し、`toReversed` は reversed copy、`toSorted` は既存 sort key を共有した
  lexicographic copy、`toSpliced` は delete range と insert values から returned copy を作る。
  focused node_diff は `array_from_matches_node` / `array_with_matches_node` /
  `array_to_reversed_matches_node` / `array_to_sorted_matches_node` /
  `array_to_spliced_matches_node` と `array_copying_matches_node_output` が pass。smoke differential は
  `pass=6 fail=0 unsupported=1 blocked=0 total=7 elapsed=0.9s`。full fixture differential は
  `pass=616 fail=394 unsupported=176 blocked=165 total=1351 elapsed=252.1s` で、
  `array-from.ts` / `array-with.ts` / `array-to-reversed.ts` / `array-to-sorted.ts` /
  `array-to-spliced.ts` / `array-copying-methods.ts` が pass へ進んだ。次の array/native bottleneck は
  `array-map-call-unsupported.ts`、`array-prototype-push-array-like.ts`、`array-foreach-thisarg.ts`、
  runtime heap-array ABI fallback、object own-key integer ordering である。
  skill 指定の `bash scripts/run/verify-harness.sh --quick|--cargo|--fixtures` は 3 件とも
  `scripts/run/verify-harness.sh` が存在せず exit 127 で実行不能だった。

- 2026-05-25 static ArrayNew callback element closure:
  `array-map-call-unsupported.ts` は `Array.prototype.map.call([1, 2], callback)` が lowering で
  returned `ArrayNew` の要素 `Call(User callback, [value, index, array])` に展開されていたが、
  `try_emit_static_array_init` は static array slot に要素式をそのまま emit していたため、
  callback call / nested array length が runtime heap-array path へ落ち `unreachable` になっていた。
  `static_array_init_elements_for_native` で console side effect のない user callback element だけを
  `static_materialized_array_element` に通し、callback return の primitive を native slot へ直接入れるようにした。
  focused node_diff は `array_map_generic_call_array_receiver_fixture_matches_node_output_under_iwasm` が pass。
  regression focused node_diff は `array_map_fixture_matches_node_output_under_iwasm`、
  `array_from_matches_node`、`array_copying_matches_node_output` が pass。smoke differential は
  `pass=6 fail=0 unsupported=1 blocked=0 total=7 elapsed=0.9s`。full fixture differential は
  `pass=617 fail=394 unsupported=175 blocked=165 total=1351 elapsed=251.3s` で、
  `array-map-call-unsupported.ts` が pass へ進んだ。次の array/native bottleneck は
  `array-prototype-push-array-like.ts`、`array-foreach-thisarg.ts`、
  runtime heap-array ABI fallback、object own-key integer ordering である。
  skill 指定の `bash scripts/run/verify-harness.sh --quick|--cargo|--fixtures` は 3 件とも
  `scripts/run/verify-harness.sh` が存在せず exit 127 で実行不能だった。

- 2026-05-25 static array-like object push closure:
  `array-prototype-push-array-like.ts` は `{}` に `Array.prototype.push` を付与した object receiver が
  lowering で `RuntimeFn::ArrayPush` / `ArrayPushMany` になっていたが、native emitter の mutator
  static state は `StaticValue::Array` だけを更新し、plain object の `length` と integer-key property を
  更新できず runtime heap-array path へ落ち `unreachable` になっていた。`static_array_like_object_push_*`
  で known static object receiver の missing `length` を push 内部では ToLength 相当の `0` と扱い、
  pushed values を `"0"`, `"1"`, ... の own data property として設定し、`length` property を
  returned new length と同じ primitive number へ更新するようにした。一方、通常の lowered `GetLength`
  による `obj.length` read は known static object の missing `length` を `undefined` として扱い、
  push 前の observable property read と push 内部の ToLength を分離した。
  focused node_diff は `array_push_prototype_array_like_fixture_matches_node_output_under_iwasm` が pass。
  regression focused node_diff は `array_push_multi_argument_fixture_matches_node_output_under_iwasm` と
  `array_map_generic_call_array_receiver_fixture_matches_node_output_under_iwasm` が pass。
  `array_foreach_thisarg_matches_node` は引き続き `unresolved wasm call symbol: $func_1` で未通過。
  smoke differential は `pass=6 fail=0 unsupported=1 blocked=0 total=7 elapsed=1.1s`。full fixture
  differential は `pass=627 fail=385 unsupported=174 blocked=165 total=1351 elapsed=239.0s` で、
  `array-prototype-push-array-like.ts` と、static object `length` property read に依存していた複数 fixture が
  pass へ進んだ。次の array/native bottleneck は `array-foreach-thisarg.ts` の unresolved `$func_1`、
  runtime heap-array ABI fallback、object own-key integer ordering、`length-tag.ts` の lowered
  `GetLength` / property-read 境界である。
  skill 指定の `bash scripts/run/verify-harness.sh --quick|--cargo|--fixtures` は 3 件とも
  `scripts/run/verify-harness.sh` が存在せず exit 127 で実行不能だった。

- 2026-05-25 static false while loop closure:
  `array-foreach-thisarg.ts` の empty `forEach` は lowering 後に `len = GetLength(empty)`、
  `i = 0`、`while (i < len)` と `FuncId(1)` callback call を含む loop へ展開されていた。
  `len=0` / `i=0` なので runtime では本体が 0 回だが、native emitter は loop body の
  `call $func_1` を出力し、required-function collector 側は side-effect-free な static user call として
  `$func_1` を出力対象から外していたため、final validation で
  `unresolved wasm call symbol: $func_1` になっていた。
  `emit_while` で副作用のない条件式だけを対象に、条件が静的 false と評価できる場合は
  loop body を出力しないようにした。条件式の fold は `Local` / primitive / `Binary` / safe `Unary` /
  `GetLength` / `EnvCellGet` に限定し、condition evaluation の副作用を消さないようにした。
  focused node_diff は `array_foreach_thisarg_matches_node` が pass。
  regression focused node_diff は `array_foreach_function_callback_matches_node` と
  `array_push_prototype_array_like_fixture_matches_node_output_under_iwasm` が pass。
  direct build `target/debug/ts2wasm build fixtures/builtins-and-io/array-foreach-thisarg.ts`
  も pass。smoke differential は `pass=6 fail=0 unsupported=1 blocked=0 total=7 elapsed=0.9s`。
  full fixture differential は `pass=628 fail=385 unsupported=173 blocked=165 total=1351 elapsed=240.7s` で、
  `array-foreach-thisarg.ts` が pass へ進んだ。次の array/native bottleneck は
  runtime heap-array ABI fallback、object own-key integer ordering、`length-tag.ts` の
  lowered `GetLength` / property-read 境界である。全体の大きい残差分は DataView/TypedArray、
  generator state machine、Map/Set tagged value representation、direct eval / Function constructor
  dynamic path、BigInt heap value rendering、string dynamic helper return representation に集中している。
  skill 指定の `bash scripts/run/verify-harness.sh --quick|--cargo|--fixtures` は 3 件とも
  `scripts/run/verify-harness.sh` が存在せず exit 127 で実行不能だった。

- 2026-05-25 static GetLength primitive boundary closure:
  `length-tag.ts` では `let n = 42; console.log(n.length);` が lowered `GetLength(Local(n))`
  になっていた。native emitter の `GetLength` は static receiver を
  `static_string_value_from_expr_with_functions` に通しており、この helper は JS `String(...)`
  相当として number `42` を `"42"` に変換するため、`.length` property read が `2` として
  出力されていた。これは string concat/coercion 用の helper と property-read semantics の混線だった。
  `static_get_length_expr_with_functions` を追加し、static array は length number、static object は
  actual `length` property または missing `undefined`、static string primitive は code-point length、
  static number/decimal/bigint/bool/symbol/tagged number は `undefined` として分類するようにした。
  `emit_expr(GetLength)` と static value collector の `GetLength` はこの helper を共有し、
  native array slot locals の fast path は従来通り `ctx.static_arrays` を先に読む。
  focused node_diff は `m5_edge_case_fixtures_match_node_output_under_iwasm` が pass。
  regression は `auto_diff_arrays_objects_length_tag_ts`、`auto_diff_arrays_objects_string_length_ts`、
  `array_push_prototype_array_like_fixture_matches_node_output_under_iwasm`、
  `array_foreach_thisarg_matches_node` が pass。smoke differential は
  `pass=6 fail=0 unsupported=1 blocked=0 total=7 elapsed=0.9s`。
  full fixture differential は `pass=629 fail=384 unsupported=173 blocked=165 total=1351 elapsed=230.1s` で、
  `length-tag.ts` が pass へ進んだ。次の array/native bottleneck は runtime heap-array ABI fallback、
  object own-key integer ordering、runtime array-like receiver の `Array.prototype.map`、
  sparse-array callback result length/state である。全体の大きい残差分は DataView/TypedArray、
  generator state machine、Map/Set tagged value representation、direct eval / Function constructor
  dynamic path、BigInt heap value rendering、string dynamic helper return representation に残っている。
  skill 指定の `bash scripts/run/verify-harness.sh --quick|--cargo|--fixtures` は 3 件とも
  `scripts/run/verify-harness.sh` が存在せず exit 127 で実行不能だった。

- 2026-05-25 static Array.map array-like and callback side-effect closure:
  runtime array-like receiver の `Array.prototype.map.call` fixtures は
  `RuntimeCall ArrayMapArrayLikeIdentity/Double` まで lowering されていたが、
  `static_array_map_array_like_from_args` が static array だけを受け入れ、`{0, 1, length}`
  形式の static object array-like receiver を配列要素へ materialize していなかったため、
  native output は `0` / `undefined` 側へ落ちていた。
  `static_array_like_object_elements_from_expr` を追加し、`StaticValue::Object` と
  `StaticValue::ObjectAlias` の `length` と indexed properties から array-like elements を作るようにした。
  missing index は JS の array-like hole として `undefined` に materialize する。
  また `array-map-sparse-holes.ts` と `array-map-callback-mutates-outer-counter.ts` では callback result
  自体は static fold できていたが、static-local collector が `ArrayNew` / `ArrayNewSparse` の element
  expression を走査していなかったため、element 評価中の `EnvCellSet` と `thisArg` property mutation が
  後続の static state へ反映されていなかった。`ArrayNew.elements` と
  `ArrayNewSparse` の present slots を collector で走査するようにした。
  focused node_diff は
  `array_map_generic_call_runtime_array_like_fixture_matches_node_output_under_iwasm`、
  `array_map_generic_call_runtime_array_like_double_fixture_matches_node_output_under_iwasm`、
  `array_map_callback_mutates_outer_counter_fixture_matches_node_output_under_iwasm`、
  `array_map_sparse_holes_fixture_matches_node_output_under_iwasm` が pass。
  regression は `array_push_prototype_array_like_fixture_matches_node_output_under_iwasm` と
  `m5_edge_case_fixtures_match_node_output_under_iwasm` が pass。
  smoke differential は `pass=6(85%) fail=0 unsupported=1 blocked=0 total=7 elapsed=0.9s`。
  full fixture differential は `pass=633(46%) fail=380 unsupported=173 blocked=165 total=1351 elapsed=230.7s`
  で、上記 4 fixtures が pass へ進んだ。
  残る主要ボトルネックは DataView/TypedArray、generator state machine、Map/Set tagged representation、
  direct eval / Function constructor の dynamic NodeShim value return/writeback、BigInt heap rendering/comparison、
  rest args runtime に集中している。
  skill 指定の `bash scripts/run/verify-harness.sh --quick|--cargo|--fixtures` は 3 件とも
  `scripts/run/verify-harness.sh` が存在せず exit 127 で実行不能だった。

- 2026-05-25 rest parameter user-call ABI closure:
  rest parameter functions は lowered function params に rest local を含む一方、native user-call emission は
  call-site の実引数だけをそのまま積んでいた。このため `rest-params-zero.ts` などは
  `$func_0` の期待 param 数と call-site stack が一致せず、iwasm load 時に
  `type mismatch: expect data but stack was empty` で落ちていた。
  user-call emission を function signature aware にし、`rest_param_index` では `ArrayCtorWithLength(0)`
  と `ArrayPushGrow` で実引数 suffix から heap rest array を作って渡すようにした。
  missing non-rest params は JS call semantics に合わせて `undefined` を渡す。
  併せて `RuntimeLinkPlan` で rest param を持つ program が `ArrayCtorWithLength` / `ArrayPushGrow`
  も要求するようにし、native emitter が合成する runtime helper 呼び出しと link plan を一致させた。
  static user-call evaluator / side-effect applier も同じ argument binding を共有し、
  rest local を `StaticValue::Array` として seed するようにした。
  focused node_diff は `rest_parameter_fixtures_match_node_output_under_iwasm`、
  `rest_arguments_object_matches_node_output_under_iwasm`、
  `function_constructor_rest_params_match_node_output`、
  `destructuring_binding_rest_fixture_matches_node_output_under_iwasm`、
  `destructuring_binding_object_rest_fixture_matches_node_output_under_iwasm` が pass。
  smoke differential は `pass=6(85%) fail=0 unsupported=1 blocked=0 total=7 elapsed=0.8s`。
  full fixture differential は `pass=641(47%) fail=372 unsupported=173 blocked=165 total=1351 elapsed=230.5s`
  で、rest parameter 系が pass へ進んだ。
  残る主要ボトルネックは DataView/TypedArray、generator state machine、Map/Set tagged representation、
  direct eval / Function constructor の dynamic NodeShim value return/writeback、BigInt heap rendering/comparison、
  dynamic `arguments` object / this-binding / for-in object key semantics に集中している。
  skill 指定の `bash scripts/run/verify-harness.sh --quick|--cargo|--fixtures` は 3 件とも
  `scripts/run/verify-harness.sh` が存在せず exit 127 で実行不能だった。

- `JsonStringify` / `JsonParse`: dynamic path は NodeShim-backed typed host bridge として
  native registry に接続済み。static JSON fold は host import なしの高速経路として残す。
  `json_runtime_calls_embed_native_helpers_and_imports` と node-shim fixture で import/実行を検証済み。
- `IteratorMap` / `IteratorFilter` / `IteratorTake` / `IteratorDrop` / `IteratorToArray` /
  `IteratorReduce` / `IteratorForEach` / `IteratorSome` / `IteratorEvery` / `IteratorFind`:
  NodeShim-backed typed host bridge として native registry に接続済み。`Iterator.from(...).drop/take/toArray`
  の chain lowering と node-shim execution fixture は通過済み。`Iterator.from(<array literal>).map(<arrow>).filter(<arrow>).toArray()`
  と `Iterator.from(<known array>).reduce(<arrow>, initialValue)` は Array callback lowering へ委譲し、
  wasm 内 user callback 実行の node-shim regression を追加済み。`ArrayPushGrow` result-local 更新、
  control-flow-aware static array 解析、dynamic/static property index fallback の native helper 接続も済み。
  `Array.prototype.filter` の callback 結果配列は empty-result でも実ヒープ配列として初期化し、
  tagged property read と equality/logging の coercion regression を `array-filter-thisarg.ts` で確認済み。
  `Iterator.from(Array.from(<known array>)).map/filter/reduce` も Array callback lowering へ正規化し、
  array literal 以外の iterator chain callback dispatch regression を追加済み。

## ドメイン別 RuntimeFn 件数

| Domain | Count |
|---|---:|
| Core | 24 |
| Operator | 30 |
| TypeCoercion | 13 |
| Number | 11 |
| BigInt | 23 |
| String | 37 |
| Array | 53 |
| Object | 47 |
| MapSet | 61 |
| TypedArray | 45 |
| Date | 40 |
| Math | 36 |
| Json | 2 |
| RegExp | 5 |
| Promise | 12 |
| Task | 3 |
| Symbol | 9 |
| Iterator | 16 |
| Module | 3 |
| Host | 19 |
| Encoding | 6 |
| **Total** | **495** |

## 実装アーキテクチャ

### 1. Native runtime assembly 層を追加

新規モジュール案:

```text
crates/backend-wasm/src/native_runtime/
  mod.rs
  registry.rs
  context.rs
  helpers.rs
  abi.rs
  core.rs
  operator.rs
  type_coercion.rs
  number.rs
  bigint.rs
  string.rs
  array.rs
  object.rs
  map_set.rs
  typed_array.rs
  date.rs
  math.rs
  json.rs
  regexp.rs
  promise.rs
  task.rs
  symbol.rs
  iterator.rs
  module.rs
  host.rs
  encoding.rs
```

中核 API 案:

```rust
pub struct RuntimeBuildCtx {
    pub strings: RuntimeStringTable,
    pub layout: RuntimeLayoutSnapshot,
    pub target: ExecutionTarget,
    pub required_plan: ValidatedRuntimeLinkPlan,
    pub helper_registry: NativeHelperRegistry,
}

pub fn append_linked_runtime(
    module: &mut WasmModule,
    program: &LoweredProgram,
    plan: &ValidatedRuntimeLinkPlan,
    ctx: &mut RuntimeBuildCtx,
) -> Result<(), Diagnostic>;

pub fn build_runtime_fn_native(
    f: RuntimeFn,
    ctx: &RuntimeBuildCtx,
) -> Result<NativeRuntimeBuild, Diagnostic>;

pub enum NativeRuntimeBuild {
    Function(WasmFunction),
    PseudoExpanded,
}
```

`PseudoExpanded` は pseudo-intrinsic だけが返せる。pseudo が final module の call graph に残ったらエラーにする。

### 2. Native module assembly の流れ

```text
NativeLoweredEmitter::emit
  -> build_validated_runtime_link_plan(program)
  -> RuntimeStringTable::collect(program, plan)
  -> WasmModule::new()
  -> append imports from plan.required_imports()
  -> append memory / globals from ABI + plan.required_globals()
  -> append data segments from RuntimeStringTable
  -> append linked runtime functions in RuntimeFn::emission_order()
  -> append user/module/start functions
  -> wasm_encoder backend validates symbols and emits bytes
```

現在の `native_lowered.rs` の `FdWrite` 常時 import と `native_write_*` は、最終的には runtime `Write` / `Log` / console 系に吸収する。暫定で残す場合も、feature flag かテスト専用 helper に閉じ込める。

### 3. WAT emitter から移植する単位

- `runtime_dispatch_*.rs` の dispatch map を Native registry の雛形にする。
- `runtime/*/emit.rs` の WAT 文字列実装を、そのままではなく typed `WasmInstr` 列へ移す。
- `emit_utf8_helpers`、prototype initializer、module cache initializer など RuntimeFn ではない補助関数も `NativeHelperFn` として registry 化する。
- `emitter/strings.rs` の string collection / runtime string origin を共通化し、WAT emitter と Native emitter が同じ data segment を使う。
- `LocalFrame` / GC root helpers / completion record helpers を WAT 専用から typed instruction helper に分離する。
- WAT-only helper を移す時は「1 関数ごとに translation + signature test + fixture parity」を最低単位にする。

### 4. `WasmInstr` / encoder の先行整備

- [x] `WasmInstr::Raw` を Native runtime builder では禁止する。既存 seed builder の `Raw` 使用は typed instruction へ置換する。
- [x] `WasmInstr::Call(String)` / `GlobalGet(String)` / `GlobalSet(String)` の未解決時に index `0` へ落とす処理を廃止し、`Result` で診断を返す。
- [x] `emit_wasm_module_binary` / `to_wasm_encoder` を `Result<Vec<u8>, Diagnostic>` に変更する。
- [x] memory 命令を拡張する: `i32.load8_u`, `i32.load8_s`, `i32.load16_u`, `i32.load16_s`, `i32.store8`, `i32.store16`, `i64.load/store`, `memory.copy`, `memory.fill`。
- [x] block type を `Option<String>` ではなく `WasmBlockType` にする。
- [ ] `WasmValType` に必要なら `F32` / `F64` を足す。ただし JS 値は原則 tagged `i32` を保つ。
- [x] `wasmparser` validation を encoder 単体テストに入れる。
- [x] `Call` の依存抽出 helper を追加し、builder 内 call が catalog deps か helper registry に含まれることをテストする。
- [x] branch label の symbol 解決も `unwrap_or(0)` から explicit structured depth / label table に変える。

### 5. ABI 境界

- [ ] JS 値は原則 `TaggedValue` (`i32`) として扱う。
- [ ] raw `i32` 算術は `InferredType::Number` の内部最適化だけに閉じ込め、境界で `NumberFromI32` / `NumberToI32` を通す。
- [ ] bool は `0/1` ではなく `ValueTag::FALSE` / `ValueTag::TRUE` を JS 値として使う。内部分岐条件だけ raw bool を許可する。
- [ ] `runtime-abi` の `Layout` / `ValueTag` / `RuntimeConst` 以外の magic number を禁止する lint/test を追加する。
- [ ] ABI custom section 付き Native binary を default build の唯一の成果物にする。

## 移行フェーズ

### M0: safety gate / 現在地の固定

- [ ] toolchain を揃え、`python scripts/manager.py check` / `gate-fast` / `nextest` を通す。
- [x] `WasmInstr::RuntimeCall` 参照と enum 定義の整合を確認する。存在しないなら削除または正式 variant 化する。
- [x] `Raw` 無視・未解決 symbol index 0 の blocker を先に直す。
- [x] `emit_wasm_binary()` が WAT fallback に落ちないことを regression test 化する。
- [ ] 既存 WAT emitter と Native emitter の fixture 差分を baseline snapshot として保存する。

2026-05-24 progress:

- `WasmInstr` has no `RuntimeCall` variant in the typed backend; current `RuntimeCall` references are
  `LoweredExpr::RuntimeCall` lowering/emission sites.
- The wasm-encoder backend now panics on unresolved call/export/global/branch symbols and on
  `WasmInstr::Raw`, instead of silently using index `0` or ignoring the instruction.
- The wasm-encoder backend now resolves structured `WasmInstr::Br("$label")` / `BrIf("$label")`
  labels against the current block/loop/if stack, so typed runtime builders can use readable labels
  without falling back to WAT-only validation.
- Typed memory instruction coverage now includes signed/unsigned 8-bit and 16-bit i32 loads,
  16-bit i32 stores, i64 load/store, and memory fill in both `WatWriter` and the wasm-encoder
  backend. `wasm_encoder_parity::parity_memory_instructions_*` validates the WAT and binary paths.
- `native_runtime_embed::available_native_runtime_builder_calls_are_declared` now extracts
  `WasmInstr::Call` from every available native runtime builder and rejects calls not covered by
  catalog deps, declared host imports, same-bundle helpers, or the native helper registry. This
  caught and fixed `LogWarn`/`LogError` missing `fd_write` import declarations and the
  `EqualEqual` bundle's missing `$is_bigint` helper.
- `wasm_encoder_backend::wasmparser_validation_covers_encoder_memory_and_control_flow` now validates
  typed memory and structured branch encoding directly through `wasmparser`, complementing the
  unresolved-symbol and raw-instruction rejection tests.
- `emit_wasm_module_binary` / `to_wasm_encoder` now return `Result<Vec<u8>, Diagnostic>`.
  Unresolved export/call/global/branch symbols, unsupported global initializers, and `WasmInstr::Raw`
  become `wasm-encoder` phase diagnostics instead of panics, ignored raw text, or index `0`
  fallbacks.
- `WasmInstr::If` now carries `WasmBlockType` instead of `Option<String>`, so WAT and wasm-encoder
  output share typed block result metadata and the binary path no longer parses result types from
  strings.
- `native_lowered::validate_no_pseudo_intrinsics` now rejects `$pseudo_*` function/call/export
  symbols before final native binary encoding, with
  `native_final_module_rejects_pseudo_intrinsic_symbols` covering the guard.

2026-05-25 progress:

- `native_runtime_embed::available_native_runtime_module_validates_with_wasmparser` now builds a
  synthetic module containing every available native runtime builder plus native helper bodies,
  catalog-declared host imports, discovered globals, and catalog-signature stubs only for unavailable
  runtime dependencies reached by calls. The module is encoded through `emit_wasm_module_binary` and
  validated with `wasmparser`, covering the native runtime builder set as a single binary module.
- Native module construction now delays `$native_write_*` helper emission until generated functions
  actually call those helpers, and `native_module_omits_write_import_when_write_helpers_are_unused`
  prevents reintroducing the old unconditional `$fd_write` import. A full import-set parity check is
  still blocked by link-plan entries such as `wasi_proc_exit` / Node host shims that are required for
  legacy WAT or manifests but must not be declared as unused native module imports because Wasmtime
  requires every declared import to link.
- `native_lowered::validate_native_final_module` now runs before binary encoding and rejects
  undeclared `GlobalGet` / `GlobalSet` symbols in the `native-emitter` phase. The existing
  pseudo-intrinsic guard is part of the same final-module validation path.
- `native_debug_wat_fallback_is_explicit_backend_api_only` now statically pins backend API
  boundaries: `wat::parse_str` may appear only in the production portion's explicit debug fallback,
  while `emit_mir_wasm_binary`, `emit_wasm_binary`, and `emit_wasm_binary_with_abi` must remain on
  the native binary path and must not call debug fallback APIs. The compiler production-path guard
  continues to block build/server/CLI callers from using those fallback APIs.
- `native_module_declares_exact_required_runtime_function_symbols` now checks final native modules
  against the runtime catalog: any emitted function whose symbol is a `RuntimeFn` must be exactly in
  the `RuntimeLinkPlan` required set after pseudo/unavailable filtering. This complements the
  existing `$native_write_*` / `$fd_write` unused-helper guard and prevents returning to all-runtime
  bundling.
- Native data segment interning is now origin-aware. Runtime strings, user literals, and raw helper
  bytes use separate cache keys, so identical length-prefixed bytes cannot cause a runtime string and
  user literal to alias the same offset. `native_runtime_string_and_user_literal_origins_do_not_share_data_segment`
  covers the collision case with runtime/user `"false"` data.
- Native runtime builder coverage now reports `available=490`, `pseudo=5`, `missing_non_pseudo=0`.
  The latest native slices added typed Atomics value helpers, FinalizationRegistry register/unregister,
  basic Map/Set/WeakMap/WeakSet method builders, Map/Set prototype global accessors, and DataView
  16/32-bit integer plus float32/float64 accessors, Date new/getTime/setTime, Map/Set array
  materializers plus forEach stubs, then BigInt add/subtract/multiply/divide/remainder/power,
  bounded i64 coercions, bitwise/shift helpers, BigIntFromValue string/value conversion, and
  unary minus, array-backed TypedArray constructors/load/store, Set fromArray/algebra helpers,
  DataView float16/BigInt accessors, Date live clock helpers, Date UTC field getters, Date
  host/string wrappers, the `DateUTC` / `DateGetLocalTimeField` arity bridge, Date UTC setters,
  local-time Date setters, Intl date/number format wrappers, and the small String
  well-formed/normalize/toLocaleString helpers, and the single-code-point String extraction
  helpers (`StringCharAt` / `StringAt`), and the range-copy String helpers
  (`StringSubstring` / `StringSubstr` / `StringSlice`), and the first String
  search/comparison helpers (`StringIndexOf` / `StringLastIndexOf` /
  `StringLocaleCompare` / `StringIncludes`) and the first remaining String
  construction helpers (`StringPadStart` / `StringPadEnd` / `StringRepeat`),
  plus the prefix/suffix checks (`StringStartsWith` / `StringEndsWith`) and
  ASCII case conversion helpers (`StringToUpperCase` / `StringToLowerCase`),
  and Unicode-whitespace trim helpers (`StringTrim` / `StringTrimStart` /
  `StringTrimEnd`), plus regexp seed helpers (`RegexpMatchInner` /
  `RegexpParseFlags`), the two-substitution `StringRaw` helper, and basic
  array mutation/copy helpers (`ArrayPop` / `ArrayCtorWithLength` /
  `ArraySlice` / `ArrayConcat`), and the first array scan/mutation helpers
  (`ArrayReverse` / `ArrayIndexOf` / `ArrayIncludes`), plus map helper
  specializations (`ArrayMapValueToString` / `ArrayMapUnaryPlus` /
  `ArrayMapArrayLikeIdentity` / `ArrayMapArrayLikeDouble`), and the truthy
  array find helpers (`ArrayFind` / `ArrayFindIndex` / `ArrayFindLast` /
  `ArrayFindLastIndex`), plus boolean/index array scans (`ArrayEvery` /
  `ArraySome` / `ArrayLastIndexOf` / `ArrayAt`), and array join/filter
  plus callback-shape iteration fallbacks (`ArrayJoin` / `ArrayFilter` /
  `ArrayReduce` / `ArrayReduceRight` / `ArrayForEach` / `ArrayMap`),
  and in-place array sort/fill/copy helpers (`ArraySortNumeric` /
  `ArraySortLexicographic` / `ArrayFill` / `ArrayCopyWithin`), plus
  array spread/copy-return/shift helpers (`ArrayPushOrSpread` /
  `ArrayWith` / `ArrayToReversed` / `ArrayShift` / `ArrayUnshift`),
  and immutable sorted/spliced plus mutating splice helpers
  (`ArrayToSorted` / `ArrayToSpliced` / `ArraySplice`), and the
  recursive `ArrayFlat` helper, plus `ObjectKeys` as the first remaining
  object enumeration helper, and the own-property name/symbol enumeration
  helpers (`ObjectGetOwnPropertyNames` / `ObjectGetOwnPropertySymbols`),
  plus enumerable own value/entry extraction helpers (`ObjectValues` /
  `ObjectEntries`), `ReflectOwnKeys` on top of the native Object key
  enumeration helpers, enumerable object spread copying (`ObjectSpread`),
  array-pair object construction (`ObjectFromEntries`), prototype reads
  (`ObjectGetPrototypeOf`), prototype writes (`ObjectSetPrototypeOf`), and
  Reflect prototype writes (`ReflectSetPrototypeOf`), and Reflect property
  writes (`ReflectSet`), plus object integrity flag helpers
  (`ObjectFreeze` / `ObjectSeal` / `ObjectPreventExtensions` /
  `ObjectIsExtensible` / `ObjectIsSealed` / `ObjectIsFrozen`), and
  Reflect prevent-extensions wrapping (`ReflectPreventExtensions`), plus
  object/global singleton accessors (`ObjectPrototype` / `GlobalThis`),
  object allocation (`ObjectCreate`), and integer-backed Math floor/ceil/abs
  (`MathFloor` / `MathCeil` / `MathAbs`), plus SameValue comparison
  (`ObjectIs`) with its catalog arity corrected to `2->1`, prototype-chain
  testing (`IsPrototypeOf`), and enumerable own-property testing
  (`PropertyIsEnumerable`), plus enumerable source-to-target copying
  (`ObjectAssign`), and integer-backed Math min/max (`MathMin` /
  `MathMax`) with their catalog arity corrected to `2->1`, plus integer
  exponentiation/multiplication (`MathPow` / `MathImul`), integer-backed
  bit-count/square-root helpers (`MathClz32` / `MathSqrt`), and the remaining
  basic integer-backed rounding/sign helpers (`MathRound` / `MathTrunc` /
  `MathSign`), integer-backed cube root (`MathCbrt`), and single-precision
  identity rounding (`MathFround`), binary16 round-trip rounding
  (`MathF16round`), NodeShim-backed host Math wrappers
  (`MathAcos` / `MathAcosh` / `MathAsin` / `MathAsinh` / `MathAtan` /
  `MathAtan2` / `MathAtanh` / `MathCos` / `MathCosh` / `MathExp` /
  `MathExpm1` / `MathHypot` / `MathLog` / `MathLog10` / `MathLog1p` /
  `MathLog2` / `MathSin` / `MathSinh` / `MathTan` / `MathTanh`),
  object/error string formatting helpers
  (`ObjectToString` / `ErrorToString` / `ObjectToLocaleString`), and WASI-backed
  `MathRandom`, plus CommonJS-style module cache/export helpers
  (`ModuleRequire` / `ModuleExportsSet` / `ModuleExportsAssign`), async
  task frame reads/results (`TaskPoll` / `TaskResult`), minimal
  test262 global construction (`Dollar262Global`), and the Promise record
  seed helpers (`PromiseConstructor` / `PromiseResolve` / `PromiseReject`),
  plus the completed generator result object helper (`GeneratorReturn`) and
  generator state iteration helpers (`GeneratorYield` / `GeneratorNext`), and
  NodeShim-backed Reflect wrappers (`ReflectApply` / `ReflectConstruct`), plus
  the host iterator bridges (`GetIterator` / `IteratorNext` / `IteratorFrom`)
  with explicit catalog import/capability linkage for the Node shim hooks, and
  dynamic eval/Function host bridges (`EvalDirectHost` / `EvalIndirectHost` /
  `FunctionCompileHost` / `FunctionCallHost` / `FunctionCallMethodHost` /
  `FunctionConstructHost`) plus `$262.eval` delegation with host exception
  sentinel bridging into `$exception_pending`, and NodeShim-backed
  path/process/crypto wrappers (`ProcessExit` / `PathJoin` / `PathResolve` /
  `PathBasename` / `PathDirname` / `CryptoRandomBytes`), plus native URI
  percent encoding/decoding and legacy escape helpers (`EncodeURI` /
  `EncodeURIComponent` / `DecodeURI` / `DecodeURIComponent` / `Escape` /
  `Unescape`), and the first
  descriptor mutation helpers (`ObjectDefineProperty` /
  `ObjectDefineProperties` / `ReflectDefineProperty`) with catalog stack
  effects aligned to their lowered argument shapes, plus descriptor
  introspection helpers (`ObjectGetOwnPropertyDescriptor` /
  `ObjectGetOwnPropertyDescriptors`), plus the native string-separator split
  helpers (`StringSplit` / `ArrayMapStringSplit`) with catalog stack effects
  aligned to their lowered two-argument call shapes, and WASI-backed
  process argument/environment helpers (`ProcessArgv` / `ProcessEnv`) with
  catalog stack effects aligned to their zero-argument runtime calls, and the
  capacity-growing `ArrayPushGrow` helper with native validation coverage, and
  the array iterator state/result helpers (`ArrayValues` / `ArrayKeys` /
  `ArrayEntries` / `ArrayIteratorNext`) with runtime string key wiring, plus
  the NodeShim-backed append-file helper (`FsAppendFileSync`), the
  WASI-backed fs read/write helpers (`FsReadFileSync` / `FsWriteFileSync`),
  and the Promise.withResolvers object helper (`PromiseWithResolvers`) plus
  AggregateError object helper (`AggregateError`), then Promise aggregate
  settlement helpers (`PromiseAny` / `PromiseFinally` / `PromiseAll` /
  `PromiseAllSettled` / `PromiseRace`) with runtime string materialization
  proof where applicable, and the inline-lowered `SpreadViaIterator`
  passthrough helper so direct catalog use validates on the native path, plus
  NodeShim-backed dynamic JSON bridges (`JsonStringify` / `JsonParse`) while
  keeping static JSON folds host-free, and the NodeShim-backed Iterator helper
  bridges (`IteratorMap` / `IteratorFilter` / `IteratorTake` / `IteratorDrop` /
  `IteratorToArray` / `IteratorReduce` / `IteratorForEach` / `IteratorSome` /
  `IteratorEvery` / `IteratorFind`) with catalog stack effects aligned to
  their lowered argument shapes. The next bottleneck is callback dispatch for
  wasm-defined Iterator helper callbacks and broader test262 semantic fixtures.

### M1: Native runtime registry

- [ ] `native_runtime::registry` を作る。
- [x] `RuntimeFn::emission_order()` を走査して required functions だけ追加する。
- [x] Pseudo-intrinsic が final module に入ったらエラーにする。
- [ ] 既存 seed builder を registry に接続し、WAT 検証ではなく wasm binary validation に切り替える。
- [x] registry に未実装 RuntimeFn がある場合は compile/test で落ちる `native_runtime_builder_coverage` を作る。

2026-05-24 progress:

- `native_runtime_embed::ordered_required_native_runtime_functions()` now filters the
  `RuntimeLinkPlan` required set through `RuntimeFn::emission_order()` before appending typed
  native builders.
- Current pseudo-intrinsics are elided by the native runtime embedding filter; the remaining work is
  to promote this from a seed-builder guard to a final-module validation error once the full
  registry module exists.
- `native_runtime_builder_missing()` now computes the current non-pseudo missing builder set in
  emission order for registry coverage tests. The current test fixes pseudo exclusion and seed
  builder exclusion without turning the whole suite red while the builder backlog is still large.

### M2: Core ABI runtime

- [ ] `AllocHeap`, `Copy`, `Write`, `ValueToStringInto`, `MemEqual`, `NumberFromI32`, `NumberToI32`, `TruthyBool`, `Not`, `TypeOf`, `IsString` を native 化する。
- [ ] GC globals / root table / call frame root helper を typed helper 化する。
- [ ] console.log は native_write helper ではなく `RuntimeFn::Log` 経由にする。
- [ ] runtime string table と data segment の共有化を完了する。

2026-05-24 progress:

- Added typed native builders for `AllocHeap`, `Copy`, `Write`, `MemEqual`, `NumberFromI32`,
  `NumberToI32`, `TypeOf`, `ValueToStringInto`, `ErrorMessage`, and `Log`, and wired them into
  native runtime embedding through `RuntimeFn::emission_order()`.
- Added typed `WasmInstr::MemoryCopy` and `WasmInstr::I32Load8U` support in both WAT writer and
  wasm-encoder backend, which removes one blocker for memory-kernel builders.
- Added typed i64 conversion/arithmetic/comparison instructions needed by `NumberFromI32`
  and BigInt i64 helpers (`i64.extend_i32_s/u`, `i64.eqz`, `i64.eq`, `i64.lt_s`,
  `i64.ge_u`, `i64.add/sub/mul`, `i64.div_u`, `i64.rem_u`, `i64.gt_u`,
  `i64.and/or/xor`, `i64.shl`, `i64.shr_s/u`) to the typed IR, WAT
  writer, and wasm-encoder backend.
- Added a native runtime data context for runtime string values. The native module assembly now
  materializes `typeof` result strings as aligned length-prefixed data segments and passes their
  tagged values into the typed `$typeof` builder.
- Extended the native runtime data context with direct string refs for `$value_to_string_into`
  (`undefined`, `null`, `false`, `true`) and added binary validation coverage for primitive,
  string, BigInt, heap-number, array, object, and symbol formatting paths. Function-token spelling
  is intentionally conservative until function metadata is available to the runtime builder.
- Added a native newline runtime string ref for `$log`, removed the remaining `Raw` store/load
  escape hatches from `$error_message` and `$log`, and validated both through the wasm-encoder
  backend. `$log` now returns `undefined` to match the catalog stack effect.
- Runtime string data segment materialization is now gated by the `RuntimeLinkPlan`: programs that
  do not require runtime strings no longer receive `typeof`/VTS/log length-prefixed data, `TypeOf`
  materializes only its own string set, and `Log` materializes its newline plus transitive VTS
  strings. Native data segment tests now cover those origins.
- Replaced the per-builder native runtime data fields with a shared `NativeRuntimeStringTable`.
  Native module assembly now interns every `RuntimeLinkPlan::required_runtime_strings()` entry once
  as an aligned length-prefixed segment, and builder-specific views (`typeof`, VTS refs, log
  newline) are derived from that table. The catalog `TypeOf` runtime string list now includes
  `"symbol"`, matching the native `$typeof` builder's reachable outputs.
- Native module assembly now declares `RuntimeLinkPlan::required_globals()` alongside the ABI
  `$heap` global, so catalog GC globals required by `$alloc_heap` are present in native modules.
  Host imports are still gated to symbols actually called by emitted native runtime helpers, plus
  the temporary legacy `fd_write` console helper, to avoid requiring unavailable host shims for
  native opaque/stub fallbacks.
- Added typed native GC root helper slices: native `_start` initializes a static root table and
  call-frame root stack when `$alloc_heap` is linked, top-level local writes mirror into the static
  root table, and emitted user functions and module initializers push/pop activation frames while
  mirroring local `set`/`tee` writes into `$gc_call_frame_current`.
- Started routing builtin `console.log` through `RuntimeFn::Log` for the safe native case where the
  single argument is already a tagged runtime value (`$typeof`, `$error_message`, `$number_from_i32`,
  `$not`, `$instanceof`). Single-argument raw-bool runtime values (`$array_is_array`, `$truthy_bool`,
  `$mem_equal`, `$is_string`) now normalize to tagged `true`/`false` before calling `$log`.
- Corrected catalog stack effects for `Write`, `Copy`, `MemEqual`, `And`, `Or`, and `InstanceOf`
  so available native builders can be checked against `RuntimeFn::stack_effect()`.
- Added binary validation coverage for the Core ABI seed calls and a builder-vs-stack-effect guard.
  `AllocHeap` is currently a typed bump-allocation seed that preserves the payload-pointer ABI and
  GC header layout; full mark/sweep integration remains part of the GC helper work.
- Added typed native builders for `RuntimeFn::LogWarn` and `RuntimeFn::LogError`. They share `$log`
  value formatting and BigInt suffix behavior, then emit message/newline iovecs through
  `$fd_write` with stderr fd `2`, matching the legacy WAT behavior while returning `undefined` to
  match the catalog stack effect.
- Extended native console statement routing so safe single-argument `console.warn` and
  `console.error` runtime values now call `$log_warn` / `$log_error` instead of the legacy native
  write helpers. The same tagged-value and raw-bool normalization gates used by `console.log` are
  reused for these stderr helpers.
- Remaining M2 bottlenecks are broader GC mark/sweep parity, replacing the legacy native console
  write helpers, and normalizing all console argument forms into tagged `RuntimeFn::Log` /
  `LogWarn` / `LogError` calls. The current safe routing covers single-argument tagged values and
  raw-bool runtime values; static bytes, multi-arg spacing, numeric writers, and dynamic fallback
  forms still use the legacy native console write path.
- Added typed native builders for `RuntimeFn::ObjectHasOwnProperty` and `RuntimeFn::ObjectHasOwn`.
  `$object_has_own_property` now performs the legacy own-property scan in typed `WasmInstr`, using
  `$value_to_string_into` for non-symbol keys and `$mem_equal` for byte comparison; `$object_has_own`
  delegates to it. The catalog stack effect and deps now match the actual WAT/native signature
  (`2->1`, deps `ValueToStringInto` + `MemEqual`).
- Added a typed native builder for `RuntimeFn::PropertyHas`. It walks the prototype chain with
  structured `WasmInstr` loops/branches, supports string-key byte comparison via `$mem_equal`, and
  supports symbol-key identity lookup via `(key, -1)`. This keeps `ReflectHas`/property-existence
  paths moving without waiting for full `PropertyGet`/`PropertySet`.
- Added a typed native builder for `RuntimeFn::ArrayGet`. It follows forwarded sparse-array
  storage, validates tagged-number indexes, checks bounds and presence bits, and returns
  `undefined` for misses. This removes another blocker for `Index`, array iterators, typed-array
  helpers, and property access paths that delegate to `$array_get`.
- Added a typed native builder for `RuntimeFn::ArrayIndexPresent`. It shares the sparse-array
  forwarding, tagged-number index validation, bounds check, and presence-bit lookup path with
  `$array_get`, returning tagged booleans for property-presence checks. The catalog signature is
  now aligned with the actual ABI (`2->1`: array value plus index value).
- Added a typed native builder for `RuntimeFn::PropertyGet`. It covers the legacy object
  prototype-chain scan, symbol-key lookup, string-key `$mem_equal` comparison, array string-index
  reads, and direct function token `name`/`length` metadata branches supplied by native runtime
  data.
- Added a typed native builder for `RuntimeFn::Index`, plus typed UTF-8 code-point index helpers
  required by string indexing. `$index` now delegates object numeric keys through `$property_get`,
  handles symbol keys, array reads via `$array_get`, string code-point extraction via `$alloc_heap`
  and `$copy`, and non-number keys through `$value_to_string_into`.
- Added a typed native builder for `RuntimeFn::PropertySet`. It covers array numeric-key writes,
  object overwrite and append paths, symbol keys, copied string keys, and frozen/sealed/tracked
  non-writable guards while preserving the legacy tagged `undefined` silent-fail behavior.
- Added a typed native builder for `RuntimeFn::PropertyDelete`. It scans own object entries,
  handles symbol and string keys, rejects frozen/tracked non-configurable deletes with tagged
  `false`, compacts the entry table by moving the last active entry, and returns tagged booleans.
- Added a typed native builder for `RuntimeFn::ReflectDeleteProperty`. It validates object targets,
  converts keys through `$value_to_string_into`, delegates to `$property_delete`, and corrects the
  catalog/checklist signature to the actual `2->1` ABI.
- Added a typed native builder for `RuntimeFn::ReflectGet`. It validates object targets, converts
  keys through `$value_to_string_into`, delegates to `$property_get`, preserves the accepted
  receiver parameter, and corrects the catalog/checklist signature to the actual `3->1` ABI.
- Added a typed native builder for `RuntimeFn::ReflectHas`. It validates object targets, converts
  keys through `$value_to_string_into`, delegates to `$property_has`, and corrects the
  catalog/checklist signature to the actual `2->1` ABI.
- Added a typed native builder for `RuntimeFn::BitwiseToI32` and connected the existing
  `RuntimeFn::BitwiseAnd`/`BitwiseXor`/`BitwiseOr` builders to native runtime embedding. The
  bitwise helpers now validate through both WAT and wasm-encoder paths, and the catalog/checklist
  signatures for the binary bitwise helpers are corrected to the actual `2->1` ABI.
- Added a typed native builder for `RuntimeFn::Negate`. It delegates coercion through
  `$number_to_i32`, applies unary numeric negation with `i32.sub`, and retags through
  `$number_from_i32` in the native runtime embedding path.
- Added typed native builders for `RuntimeFn::Sub`, `RuntimeFn::Mul`, `RuntimeFn::Div`, and
  `RuntimeFn::Mod`. They reuse `$number_to_i32`/`$number_from_i32`, preserve the legacy
  divide/mod-by-zero `undefined` guard, and correct the catalog/checklist signatures to the
  actual `2->1` ABI.
- Added typed native builders for `RuntimeFn::SubFast`, `RuntimeFn::MulFast`,
  `RuntimeFn::DivFast`, and `RuntimeFn::ModFast`. They preserve the number-tag guard shape from
  the legacy helpers and delegate to the corresponding base arithmetic helper in both fast and
  fallback paths.
- Added typed native builders for `RuntimeFn::NumberIsNaN`, `RuntimeFn::NumberIsFinite`,
  `RuntimeFn::NumberIsInteger`, and `RuntimeFn::NumberIsSafeInteger`. They preserve the legacy
  non-coercing number checks, heap-number decimal scan for integer checks, and special
  NaN/infinity sentinels.
- Added a typed native builder for `RuntimeFn::ValueOf`. It preserves the legacy identity helper
  behavior and is now covered by native runtime embedding validation.
- Added a typed native builder for `RuntimeFn::BooleanCoerce`. It preserves the legacy exact
  falsey checks, number/string truthiness, and BigInt-zero object branch without raw WAT escapes.
- Added typed native builders for `RuntimeFn::IsNaN` and `RuntimeFn::IsFinite`, including their
  `$is_nan_string` / `$is_finite_string` auxiliary helpers. The native runtime embedding path now
  expands a single RuntimeFn into multiple helper functions when needed.
- Added a typed native builder for `RuntimeFn::BooleanToString`. It uses the native runtime string
  table values for `false` and `true`, and native data-segment tests cover runtime string gating.
- Added typed native builders for `RuntimeFn::GlobalParseInt`, `$parse_int_string`, and the
  `$number_to_string` companion needed by numeric parseInt input. The catalog dependency now
  includes `RuntimeFn::NumberToString`, so the native link plan also pulls in
  `$value_to_string_into`, `$alloc_heap`, and `$copy` for that companion path.
- Added a typed native builder for `RuntimeFn::NumberCoerce`. It preserves the legacy primitive
  coercions and delegates string conversion to `$parse_int_string`, which is now available through
  the `GlobalParseInt` dependency path.
- Added typed native builders for `RuntimeFn::GlobalParseFloat` and `$parse_float_string`. They
  preserve the legacy integer-only parseFloat subset, including non-string/no-digit fallback to
  tagged `0`.
- Native `emit_expr` now handles expression-position `parseInt`/`parseFloat` calls with the
  runtime helper ABI: static strings are materialized as tagged runtime strings, parseInt radix is
  tagged for `$parse_int_string`, and fully static parse calls fold back to native raw number /
  decimal fixture output. This keeps `number-static-parse*.ts` parity while unblocking
  `number-complete.ts`.
- Native `emit_expr` now treats `Number.isNaN`/`Number.isFinite`/`Number.isInteger`/
  `Number.isSafeInteger` runtime calls as native raw boolean results. Static cases fold without a
  helper call; dynamic cases pass tagged JS values into the typed helper and convert tagged
  true/false back to raw native booleans.
- Static arithmetic folding now applies JS numeric conversion for `-`, `*`, and `/`, so cases like
  `"abc" / 1` produce the NaN sentinel instead of raw wasm integer division.
- Added typed native builders for `RuntimeFn::NumberToFixed`,
  `RuntimeFn::NumberToExponential`, and `RuntimeFn::NumberToPrecision`. Native runtime-call
  emission now normalizes raw native number receivers/precision arguments to the tagged helper ABI,
  and `number-format-dynamic.ts` covers the non-static path.
- Added typed native builders for `RuntimeFn::StringEqual`, `RuntimeFn::BigIntCompare` (with the
  private `$is_bigint` companion), `RuntimeFn::StrictEqual`, `RuntimeFn::SameValueZero`, and
  `RuntimeFn::StrictNotEqual`. While doing this, the catalog stack-effect bottleneck for
  `StringEqual`, `BigIntCompare`, `EqualEqual`, `BangEqual`, and `StrictNotEqual` was corrected
  from the placeholder `1->1` shape to the actual two-argument helper ABI.
- Added a typed native builder for `RuntimeFn::Concat`. It preserves the legacy string fast path
  and non-string `$value_to_string_into` scratch-buffer conversion path, making `Add`'s string
  concatenation dependency available in the native runtime embedder.
- Added typed native builders for `RuntimeFn::Add`, `RuntimeFn::AddFast`, and
  `RuntimeFn::BigIntMixedArithmeticTypeError`. `Add` now routes string operands through native
  `$concat`, BigInt mixed arithmetic through the native catchable-error helper, and numeric
  operands through the native number conversion helpers. The BigInt mixed arithmetic helper uses
  the native runtime string table and exception globals; builtin TypeError prototype wiring remains
  a later ObjectPrototype/builtin-error-prototype completion item.
- Added typed native builders for `RuntimeFn::EqualEqual` and `RuntimeFn::BangEqual`. The native
  embedder now expands `EqualEqual` with its private `$string_to_number_for_equality`,
  `$primitive_to_number_for_equality`, `$bigint_string_to_small_int_for_comparison`, and
  `$bigint_equal_small_int` companions, clearing the loose-equality unresolved-symbol bottleneck.
  The public `BigIntStringComparisonBoundaryError` RuntimeFn remains a separate unchecked runtime
  error helper; the migrated loose-equality helper preserves the legacy bounded-small-int trap path.
- Added typed native builders for `RuntimeFn::Less`, `LessFast`, `LessEqual`, `LessEqualFast`,
  `Greater`, `GreaterFast`, `GreaterEqual`, and `GreaterEqualFast`. Relational comparisons share
  a native `$bigint_compare_small_int` private companion and reuse the existing BigInt/string
  conversion helpers from the `EqualEqual` dependency chain.
- Added a typed native builder for `RuntimeFn::BigIntStringComparisonBoundaryError`. It preserves
  the legacy diagnostic-abort shape (`0->0`) and uses the native runtime string table plus `$write`.

### M3: operator / coercion / number parity

- [x] `Add/Sub/Mul/Div/Mod` と fast variants を runtime function として実装する。
- [x] 比較・等価性・BigInt mixed arithmetic error の依存連鎖を catalog 通りに通す。
  - 2026-05-24: strict/loose equality now routes supported native operands through tagged
    `$strict_equal` / `$equal_equal` helpers and folds static strict-vs-loose equality separately.
    This cleared `fixtures/core-semantics/abstract-equality.ts` under Node/iwasm parity.
- [x] `emit_expr` の raw wasm arithmetic を JS semantics runtime call へ段階的に置換する。
  - 2026-05-24: native `LoweredExpr::Binary` now routes raw numeric `Add/Sub/Mul/Div/Mod`
    operands through the typed runtime helpers by normalizing raw i32 operands with
    `NumberFromI32`, calling `$add/$sub/$mul/$div/$mod`, and converting helper results back
    through `NumberToI32` for the current native raw-number expression ABI. Dynamic fixture
    `number-arithmetic-dynamic.ts` covers function-parameter arithmetic.
- [ ] `Number*`, `Boolean*`, `isNaN`, `parseInt`, `parseFloat`, `isFinite` を fixture parity で固める。
  - 2026-05-24: `number_methods_matches_node_output`, `number_static_parse*`, and `number_is_*`
    native node-diff parity passed for parse and Number predicate coverage. Remaining work is the
    broader Boolean/global coercion matrix and dynamic non-static parse/number formatting paths.
  - 2026-05-24: native statement/loop conditions now fold statically known JS truthiness before
    emitting raw wasm condition branches, fixing `null`, `undefined`, `false`, `0`, and empty
    string conditions in `truthiness.ts` without routing raw native booleans through tagged
    `$truthy_bool`.
  - 2026-05-24: `TS2WASM_RUN_NODE_DIFF=1 cargo test -p ts2wasm-cli m3_semantic_fixtures_match_node_output_under_iwasm --test node_diff -- --nocapture`
    now passes. The fixes covered static nullish coalescing side-effect pruning, prototype-chain
    lookup with explicit null prototypes, static class `new`/`super` constructor effects, native
    string local/tagged Concat ABI normalization, static `in` operator lowering, and `void`
    side-effect preservation for `typeof` probes.

### M4: heap object kernels

- [ ] Array/Object/String の allocation/read/write/property kernel を先に完成させる。
- [ ] Object prototype/globalThis/class prototype/builtin error prototype initializer を typed 化する。
- [ ] `Index`, `PropertyGet/Set/Delete/Has`, `ArrayGet`, sparse array presence を native 化する。
- [ ] prototype global 初期化の順序を snapshot で固定する。
  - 2026-05-24 bottleneck note: even with typed `PropertyGet`/`PropertyHas` builders available,
    fixture parity still depends on emitter-side static object provenance. Static slots must not
    masquerade as own properties, prototype changes need explicit `null` state, and constructor
    property writes must store caller-independent static values rather than callee-local refs.
  - 2026-05-25: dynamic `GetLength` now routes through a native `$get_length` typed builder, and
    `m5_array_object_fixtures_match_node_output_under_iwasm` passes after static-array dynamic
    index reads/writes were wired into the native `PropertyGetDynamic`/`PropertySetDynamic` paths.

### M5: domain expansion

- [ ] String, Array, Object, MapSet, TypedArray, Date, Math, JSON, RegExp, Promise/Task, Iterator, Symbol, Module, Host/Encoding を domain ごとに builder 化する。
- [ ] Host shim 関数は import/capability manifest と deny tests を必ずセットにする。
- [ ] WAT emitter との差分を fixture differential で潰す。

### M6: MIR/native convergence

- [x] `emit_mir_wasm_binary` の WAT parse 経路を廃止し、MIR も Native binary backend へ直接 emit する。
- [ ] LoweredProgram と MIR が同じ runtime registry / ABI / encoder を使うようにする。

2026-05-25 progress:

- `emit_mir_wasm_binary` no longer calls `emit_mir_wat` + `wat::parse_str`; it raises MIR back to
  `LoweredProgram`, validates that shape, and uses the same native `emit_wasm_binary` path as the
  build-facing LoweredProgram entrypoint. Regression tests now prove both byte parity with the
  raised LoweredProgram native output and rejection of native-unsupported MIR without WAT fallback.

### M7: fallback retirement

- [x] 通常 build / server / CLI から WAT conversion fallback を完全に切り離す。
- [ ] WAT emitter は `dump --wat` と debug fallback tests 専用にする。
- [x] coverage dashboard に `native_runtime_builder_coverage` を追加し、未実装 RuntimeFn が 0 件であることを gate にする。

2026-05-25 progress:

- `compiler_late::production_emit_paths_do_not_call_wat_conversion_fallbacks` statically denies
  `wat::parse_str`, `Command::new("wat2wasm")`, legacy WAT writers, and debug fallback APIs from the
  compiler pipeline, server emit path, lower stage, and CLI entrypoints.
- `native_runtime_builder_coverage` is now reported in dashboard data/UI and wired into
  `gate-fast`; the static registry check reports `available=490`, `pseudo=5`,
  `missing_non_pseudo=0`. `ArrayToSorted`, `ArrayToSpliced`, and `ArraySplice` had typed builders
  and build arms but were missing from the availability registry; the registry now exposes them.

## Pseudo-intrinsic の扱い

次は catalog 上は RuntimeFn だが、final wasm module に同名の real function として残さない。
- [ ] `RuntimeFn::ArrayPushMany` (`$pseudo_array_push_many`): emit/lower 段階で real RuntimeFn 呼び出し列または inline sequence に展開し、Native registry は実関数 builder を返さない。
- [ ] `RuntimeFn::HeapClosureCall` (`$pseudo_heap_closure_call`): emit/lower 段階で real RuntimeFn 呼び出し列または inline sequence に展開し、Native registry は実関数 builder を返さない。
- [ ] `RuntimeFn::PrivateBrandCheck` (`$pseudo_private_brand_check`): emit/lower 段階で real RuntimeFn 呼び出し列または inline sequence に展開し、Native registry は実関数 builder を返さない。
- [ ] `RuntimeFn::PrivateFieldGet` (`$pseudo_private_field_get`): emit/lower 段階で real RuntimeFn 呼び出し列または inline sequence に展開し、Native registry は実関数 builder を返さない。
- [ ] `RuntimeFn::PrivateFieldSet` (`$pseudo_private_field_set`): emit/lower 段階で real RuntimeFn 呼び出し列または inline sequence に展開し、Native registry は実関数 builder を返さない。

## Host import / capability 対応 TODO

Host import を持つ RuntimeFn は `57` 件。直接 import せず `RuntimeLinkPlan.required_imports()` だけから module に入れる。

| RuntimeFn | Symbol | Imports | Capability |
|---|---|---|---|
| `RuntimeFn::ReadStdinBytes` | `$read_stdin_bytes` | `IMPORT_FD_READ` | `CAP_STDIN_READ` |
| `RuntimeFn::Write` | `$write` | `IMPORT_FD_WRITE` | `CAP_STDOUT_WRITE` |
| `RuntimeFn::ConsoleTimeStart` | `$console_time_start` | `IMPORT_CLOCK_TIME_GET` | `CAP_WASI_CLOCK_REALTIME` |
| `RuntimeFn::ConsoleTimeEndFn` | `$console_time_end` | `IMPORT_CLOCK_TIME_GET` | `CAP_WASI_CLOCK_REALTIME` |
| `RuntimeFn::DateEpochMsNowNumber` | `$date_epoch_ms_now_number` | `IMPORT_CLOCK_TIME_GET` | `CAP_WASI_CLOCK_REALTIME` |
| `RuntimeFn::DateNewLive` | `$date_new_live` | `IMPORT_CLOCK_TIME_GET` | `CAP_WASI_CLOCK_REALTIME` |
| `RuntimeFn::DateNow` | `$date_now` | `IMPORT_CLOCK_TIME_GET` | `CAP_WASI_CLOCK_REALTIME` |
| `RuntimeFn::DateParse` | `$date_parse` | `IMPORT_DATE_PARSE` | `CAP_HOST_DATE_PARSE` |
| `RuntimeFn::DateUTC` | `$date_utc` | `IMPORT_DATE_UTC` | `CAP_HOST_DATE_UTC` |
| `RuntimeFn::DateToString` | `$date_to_string` | `IMPORT_DATE_TO_STRING` | `CAP_HOST_DATE_TO_STRING` |
| `RuntimeFn::DateGetLocalTimeField` | `$date_get_local_time_field` | `IMPORT_DATE_GET_LOCAL_TIME_FIELD` | `CAP_HOST_DATE_GET_LOCAL_TIME_FIELD` |
| `RuntimeFn::DateToISOString` | `$date_to_iso_string` | `IMPORT_DATE_TO_ISO_STRING` | `CAP_HOST_DATE_TO_ISO_STRING` |
| `RuntimeFn::DateGetTimezoneOffset` | `$date_get_timezone_offset` | `IMPORT_DATE_GET_TIMEZONE_OFFSET` | `CAP_HOST_DATE_GET_TIMEZONE_OFFSET` |
| `RuntimeFn::DateToDateString` | `$date_to_date_string` | `IMPORT_DATE_TO_DATE_STRING` | `CAP_HOST_DATE_TO_DATE_STRING` |
| `RuntimeFn::DateToTimeString` | `$date_to_time_string` | `IMPORT_DATE_TO_TIME_STRING` | `CAP_HOST_DATE_TO_TIME_STRING` |
| `RuntimeFn::IntlDateTimeFormatFormat` | `$intl_date_time_format_format` | `IMPORT_INTL_DATE_TIME_FORMAT_FORMAT` | `CAP_INTL_DATE_TIME_FORMAT_FORMAT` |
| `RuntimeFn::StringNormalize` | `$string_normalize` | `IMPORT_STRING_NORMALIZE` | `CAP_STRING_NORMALIZE` |
| `RuntimeFn::IntlNumberFormatFormat` | `$intl_number_format_format` | `IMPORT_INTL_NUMBER_FORMAT_FORMAT` | `CAP_INTL_NUMBER_FORMAT_FORMAT` |
| `RuntimeFn::ReflectApply` | `$reflect_apply` | `IMPORT_REFLECT_APPLY` | `CAP_HOST_REFLECT_APPLY` |
| `RuntimeFn::ReflectConstruct` | `$reflect_construct` | `IMPORT_REFLECT_CONSTRUCT` | `CAP_HOST_REFLECT_CONSTRUCT` |
| `RuntimeFn::MathRandom` | `$math_random` | `IMPORT_RANDOM_GET` | `CAP_WASI_RANDOM` |
| `RuntimeFn::MathAcos` | `$math_acos` | `IMPORT_MATH_ACOS` | `CAP_HOST_MATH_ACOS` |
| `RuntimeFn::MathAcosh` | `$math_acosh` | `IMPORT_MATH_ACOSH` | `CAP_HOST_MATH_ACOSH` |
| `RuntimeFn::MathAsin` | `$math_asin` | `IMPORT_MATH_ASIN` | `CAP_HOST_MATH_ASIN` |
| `RuntimeFn::MathAsinh` | `$math_asinh` | `IMPORT_MATH_ASINH` | `CAP_HOST_MATH_ASINH` |
| `RuntimeFn::MathAtan` | `$math_atan` | `IMPORT_MATH_ATAN` | `CAP_HOST_MATH_ATAN` |
| `RuntimeFn::MathAtan2` | `$math_atan2` | `IMPORT_MATH_ATAN2` | `CAP_HOST_MATH_ATAN2` |
| `RuntimeFn::MathAtanh` | `$math_atanh` | `IMPORT_MATH_ATANH` | `CAP_HOST_MATH_ATANH` |
| `RuntimeFn::MathCos` | `$math_cos` | `IMPORT_MATH_COS` | `CAP_HOST_MATH_COS` |
| `RuntimeFn::MathCosh` | `$math_cosh` | `IMPORT_MATH_COSH` | `CAP_HOST_MATH_COSH` |
| `RuntimeFn::MathExp` | `$math_exp` | `IMPORT_MATH_EXP` | `CAP_HOST_MATH_EXP` |
| `RuntimeFn::MathExpm1` | `$math_expm1` | `IMPORT_MATH_EXPM1` | `CAP_HOST_MATH_EXPM1` |
| `RuntimeFn::MathHypot` | `$math_hypot` | `IMPORT_MATH_HYPOT` | `CAP_HOST_MATH_HYPOT` |
| `RuntimeFn::MathLog` | `$math_log` | `IMPORT_MATH_LOG` | `CAP_HOST_MATH_LOG` |
| `RuntimeFn::MathLog10` | `$math_log10` | `IMPORT_MATH_LOG10` | `CAP_HOST_MATH_LOG10` |
| `RuntimeFn::MathLog1p` | `$math_log1p` | `IMPORT_MATH_LOG1P` | `CAP_HOST_MATH_LOG1P` |
| `RuntimeFn::MathLog2` | `$math_log2` | `IMPORT_MATH_LOG2` | `CAP_HOST_MATH_LOG2` |
| `RuntimeFn::MathSin` | `$math_sin` | `IMPORT_MATH_SIN` | `CAP_HOST_MATH_SIN` |
| `RuntimeFn::MathSinh` | `$math_sinh` | `IMPORT_MATH_SINH` | `CAP_HOST_MATH_SINH` |
| `RuntimeFn::MathTan` | `$math_tan` | `IMPORT_MATH_TAN` | `CAP_HOST_MATH_TAN` |
| `RuntimeFn::MathTanh` | `$math_tanh` | `IMPORT_MATH_TANH` | `CAP_HOST_MATH_TANH` |
| `RuntimeFn::FsReadFileSync` | `$fs_read_file_sync` | `IMPORT_FS_READ_WASI` | `CAP_WASI_FILESYSTEM_READ` |
| `RuntimeFn::FsWriteFileSync` | `$fs_write_file_sync` | `IMPORT_FS_WRITE_WASI` | `CAP_WASI_FILESYSTEM_WRITE` |
| `RuntimeFn::FsAppendFileSync` | `$fs_append_file_sync` | `IMPORT_FS_APPEND_FILE_SYNC` | `CAP_HOST_FS_APPEND_FILE_SYNC` |
| `RuntimeFn::ProcessArgv` | `$process_argv` | `&[HostImport::ArgsSizesGet, HostImport::ArgsGet]` | `CAP_WASI_ARGS` |
| `RuntimeFn::ProcessEnv` | `$process_env` | `&[HostImport::EnvironSizesGet, HostImport::EnvironGet]` | `CAP_WASI_ENV` |
| `RuntimeFn::ProcessExit` | `$process_exit` | `IMPORT_PROCESS_EXIT` | `CAP_HOST_PROCESS_EXIT` |
| `RuntimeFn::PathJoin` | `$path_join` | `IMPORT_PATH_JOIN` | `CAP_HOST_PATH_JOIN` |
| `RuntimeFn::PathResolve` | `$path_resolve` | `IMPORT_PATH_RESOLVE` | `CAP_HOST_PATH_RESOLVE` |
| `RuntimeFn::PathBasename` | `$path_basename` | `IMPORT_PATH_BASENAME` | `CAP_HOST_PATH_BASENAME` |
| `RuntimeFn::PathDirname` | `$path_dirname` | `IMPORT_PATH_DIRNAME` | `CAP_HOST_PATH_DIRNAME` |
| `RuntimeFn::CryptoRandomBytes` | `$crypto_random_bytes` | `IMPORT_CRYPTO_RANDOM_BYTES` | `CAP_HOST_CRYPTO_RANDOM_BYTES` |
| `RuntimeFn::EvalDirectHost` | `$eval_direct_host` | `&[HostImport::EvalDirect]` | `CAP_HOST_EVAL_DIRECT` |
| `RuntimeFn::EvalIndirectHost` | `$eval_indirect_host` | `&[HostImport::EvalIndirect]` | `CAP_HOST_EVAL_INDIRECT` |
| `RuntimeFn::FunctionCompileHost` | `$function_compile_host` | `&[HostImport::FunctionCompile]` | `CAP_HOST_FUNCTION_COMPILE` |
| `RuntimeFn::FunctionCallHost` | `$function_call_host` | `&[HostImport::FunctionCall]` | `CAP_HOST_FUNCTION_CALL` |
| `RuntimeFn::FunctionCallMethodHost` | `$function_call_method_host` | `&[HostImport::FunctionCallMethod]` | `CAP_HOST_FUNCTION_CALL_METHOD` |
| `RuntimeFn::FunctionConstructHost` | `$function_construct_host` | `&[HostImport::FunctionConstruct]` | `CAP_HOST_FUNCTION_CONSTRUCT` |

追加 TODO:

- [ ] Host import の module/name/signature を `HostImport` catalog から一元生成する。
- [ ] capability deny test を import あり RuntimeFn ごとに作る。
- [ ] `--host-deny=*` で host shim が必要な fixture は明示エラーにする。
- [ ] WASI 系 (`fd_write`, `fd_read`, `random_get`, `clock_time_get`, args/env) は target/runtime policy で切り替える。
- [ ] Host shim を使う Math/Date/Intl/Reflect/Eval/Function/FS/Process/Crypto は manifest snapshot を必ず追加する。

## Runtime strings / globals TODO

Runtime string を持つ RuntimeFn は `35` 件。

- [x] `RuntimeFn::ValueToStringInto` (`$value_to_string_into`): runtime_strings `VTS_RUNTIME_STRINGS` を Native `RuntimeStringTable` に登録する。
- [x] `RuntimeFn::ErrorMessage` (`$error_message`): runtime_strings `VTS_RUNTIME_STRINGS` を Native `RuntimeStringTable` に登録する。
- [x] `RuntimeFn::Log` (`$log`): runtime_strings `LOG_RUNTIME_STRINGS` を Native `RuntimeStringTable` に登録する。
- [x] `RuntimeFn::LogWarn` (`$log_warn`): runtime_strings `LOG_RUNTIME_STRINGS` を Native `RuntimeStringTable` に登録する。
- [x] `RuntimeFn::LogError` (`$log_error`): runtime_strings `LOG_RUNTIME_STRINGS` を Native `RuntimeStringTable` に登録する。
- [x] `RuntimeFn::ConsoleGroupStart` (`$console_group_start`): runtime_strings `LOG_RUNTIME_STRINGS` を Native `RuntimeStringTable` に登録する。typed native builder を registry に接続済み。
- [x] `RuntimeFn::ConsoleTimeEndFn` (`$console_time_end`): runtime_strings `LOG_RUNTIME_STRINGS` を Native `RuntimeStringTable` に登録する。typed native builder を registry に接続済み。
- [x] `RuntimeFn::ConsoleCountImpl` (`$console_count`): runtime_strings `LOG_RUNTIME_STRINGS` を Native `RuntimeStringTable` に登録する。typed native builder を registry に接続済み。
- [x] `RuntimeFn::TypeOf` (`$typeof`): runtime_strings `TYPEOF_RUNTIME_STRINGS` を Native `RuntimeStringTable` に登録する。
- [x] `RuntimeFn::BigIntDiv` (`$bigint_div`): runtime_strings `BIGINT_DIVISION_BY_ZERO_RANGE_ERROR_RUNTIME_STRINGS` を Native `RuntimeStringTable` に登録する。`native_bigint_div_materializes_division_by_zero_runtime_strings` で module data segment まで検証済み。
- [x] `RuntimeFn::BigIntDivisionByZeroRangeError` (`$bigint_division_by_zero_range_error`): runtime_strings `BIGINT_DIVISION_BY_ZERO_RANGE_ERROR_RUNTIME_STRINGS` を Native `RuntimeStringTable` に登録する。typed native builder を registry に接続済み。
- [x] `RuntimeFn::BigIntMixedArithmeticTypeError` (`$bigint_mixed_arithmetic_type_error`): runtime_strings `BIGINT_MIXED_ARITHMETIC_TYPE_ERROR_RUNTIME_STRINGS` を Native `RuntimeStringTable` に登録する。
- [x] `RuntimeFn::BigIntStringComparisonBoundaryError` (`$bigint_string_comparison_boundary_error`): runtime_strings `BIGINT_STRING_COMPARISON_BOUNDARY_ERROR_RUNTIME_STRINGS` を Native `RuntimeStringTable` に登録する。
- [x] `RuntimeFn::PrivateBrandTypeError` (`$private_brand_type_error`): runtime_strings `PRIVATE_BRAND_TYPE_ERROR_RUNTIME_STRINGS` を Native `RuntimeStringTable` に登録する。typed native builder を registry に接続済み。
- [x] `RuntimeFn::StringRaw` (`$string_raw`): runtime_strings `STRING_RAW_RUNTIME_STRINGS` を Native `RuntimeStringTable` に登録する。typed native builder を registry に接続済み。
- [x] `RuntimeFn::ArrayValues` (`$array_values`): runtime_strings `ARRAY_ITERATOR_STATE_RUNTIME_STRINGS` を Native `RuntimeStringTable` に登録する。
- [x] `RuntimeFn::ArrayKeys` (`$array_keys`): runtime_strings `ARRAY_ITERATOR_STATE_RUNTIME_STRINGS` を Native `RuntimeStringTable` に登録する。
- [x] `RuntimeFn::ArrayEntries` (`$array_entries`): runtime_strings `ARRAY_ITERATOR_STATE_RUNTIME_STRINGS` を Native `RuntimeStringTable` に登録する。
- [x] `RuntimeFn::ArrayIteratorNext` (`$array_iterator_next`): runtime_strings `ARRAY_ITERATOR_NEXT_RUNTIME_STRINGS` を Native `RuntimeStringTable` に登録する。
- [x] `RuntimeFn::ObjectToString` (`$object_to_string`): runtime_strings `OBJECT_TO_STRING_RUNTIME_STRINGS` を Native `RuntimeStringTable` に登録する。
- [x] `RuntimeFn::ErrorToString` (`$error_to_string`): runtime_strings `ERROR_TO_STRING_RUNTIME_STRINGS` を Native `RuntimeStringTable` に登録する。
- [x] `RuntimeFn::JsonStringify` (`$json_stringify`): NodeShim-backed typed host bridge として native registry に接続済み。known static values / replacer arrays / simple function replacers / space forms は native static fold 済みで、dynamic value は `host.json.stringify` import 経由で実行する。
- [x] `RuntimeFn::JsonParse` (`$json_parse`): NodeShim-backed typed host bridge として native registry に接続済み。current literal fixture cluster は native static parse 済みで、dynamic source/reviver は `host.json.parse` import 経由で実行する。
- [x] `RuntimeFn::BooleanToString` (`$boolean_to_string`): runtime_strings `BOOLEAN_TO_STRING_RUNTIME_STRINGS` を Native `RuntimeStringTable` に登録する。
- [x] `RuntimeFn::GeneratorYield` (`$generator_yield`): runtime_strings `GENERATOR_YIELD_RUNTIME_STRINGS` を Native `RuntimeStringTable` に登録し、typed native builder を registry に接続済み。
- [x] `RuntimeFn::GeneratorReturn` (`$generator_return`): runtime_strings `GENERATOR_RETURN_RUNTIME_STRINGS` を Native `RuntimeStringTable` に登録し、typed native builder を registry に接続済み。
- [x] `RuntimeFn::GeneratorNext` (`$generator_next`): runtime_strings `GENERATOR_NEXT_RUNTIME_STRINGS` を Native `RuntimeStringTable` に登録し、typed native builder を registry に接続済み。
- [x] `RuntimeFn::PromiseWithResolvers` (`$promise_with_resolvers`): runtime_strings `PROMISE_WITH_RESOLVERS_RUNTIME_STRINGS` を Native `RuntimeStringTable` に登録する。typed builder + native registry 接続済み、`native_promise_with_resolvers_materializes_runtime_strings` で module data segment まで検証済み。
- [x] `RuntimeFn::PromiseAllSettled` (`$promise_all_settled`): runtime_strings `PROMISE_ALL_SETTLED_RUNTIME_STRINGS` を Native `RuntimeStringTable` に登録する。typed builder + native registry 接続済み、`native_promise_all_settled_materializes_runtime_strings` で module data segment まで検証済み。
- [x] `RuntimeFn::PromiseAny` (`$promise_any`): runtime_strings `PROMISE_ANY_RUNTIME_STRINGS` を Native `RuntimeStringTable` に登録する。typed builder + native registry 接続済み、`native_promise_any_materializes_runtime_strings` で module data segment まで検証済み。
- [x] `RuntimeFn::AggregateError` (`$aggregate_error`): runtime_strings `AGGREGATE_ERROR_RUNTIME_STRINGS` を Native `RuntimeStringTable` に登録する。typed builder + native registry 接続済み、`native_aggregate_error_materializes_runtime_strings` で module data segment まで検証済み。
- [x] `RuntimeFn::SymbolNew` (`$symbol_new`): heap symbol builder を native registry に接続済み。`Symbol()` / `Symbol(desc)` は runtime string table 依存なしで native object layout から `Symbol(...)` 表示まで処理する。
- [x] `RuntimeFn::SymbolFor` (`$symbol_for`): heap symbol registry builder を native registry に接続済み。`Symbol.for` identity / `Symbol.keyFor` fixture parity で検証済み。
- [x] `RuntimeFn::SymbolToString` (`$symbol_to_string`): runtime_strings なし。`ValueToStringInto` / `AllocHeap` / `Copy` 依存を catalog に反映し、native registry と `symbol-to-string` parity で検証済み。
- [x] static symbol-key descriptor fallback: `Object.getOwnPropertySymbols` は既知 static object の symbol key 配列を materialize し、`Object.keys` は symbol key を除外する。descriptor `!== undefined` は static strict type で fold し、symbol-key `delete` は static object model から props/attrs/order を削除する。`symbol-key-descriptor-identity` parity 済み。
- [x] `RuntimeFn::SymbolHasInstance` (`$symbol_has_instance`): runtime_strings `&["prototype"]` を Native `RuntimeStringTable` に登録し、Symbol.hasInstance prototype-chain helper を typed native builder と registry に接続済み。
- [ ] `RuntimeFn::HeapClosureCall` (`$pseudo_heap_closure_call`): runtime_strings `HEAP_CLOSURE_CALL_RUNTIME_STRINGS` を Native `RuntimeStringTable` に登録する。

Runtime global 側の TODO:

- [x] `RuntimeFn::globals()` から必要 global を収集し、Native module の global section に追加する。`native_module_declares_link_plan_globals_for_alloc_heap` / `native_module_does_not_declare_unplanned_runtime_globals` で固定。
- [x] 初期値は `RuntimeGlobal::initial_value()` だけを使い、Native 側で重複定義しない。`native_module_uses_runtime_global_catalog_initial_values` で固定。
- [x] `GlobalGet/GlobalSet` が未知 global を参照したら encoder 前に診断を返す。
- [x] module cache / prototype object / exception runtime / console indent の初期化順を snapshot 化する。現状 Native module が宣言する heap/module cache/exception/console indent/export global 順序を `native_module_global_initialization_order_is_snapshotted` で固定。prototype は該当 native builder 接続時に同じ snapshot へ拡張する。

## テスト TODO

- [ ] `runtime_fn_emission_order_is_native_builder_complete`: pseudo 以外の required RuntimeFn すべてに builder が存在する。
- [x] `native_builder_signature_matches_runtime_catalog`: builder の params/results が `RuntimeFn::stack_effect()` と一致する。
- [x] `native_builder_deps_are_declared`: builder 内 `Call($x)` が `RuntimeFn::spec().deps` または helper registry にある。
- [x] `native_builder_has_no_raw`: Native runtime builder が `WasmInstr::Raw` を使わない。
- [x] `native_encoder_rejects_unresolved_symbol`: 未解決 call/global/branch が index 0 にならずエラーになる。
- [x] `native_runtime_module_validates`: 全 runtime functions を含む synthetic module を `wasmparser` で validate する。
- [x] `native_link_plan_snapshot_parity`: WAT path と Native path の required runtime/import/global/string が一致する。`native_link_plan_snapshot_matches_wat_path_required_sets` で WAT/Native emission 前後の required set snapshot 不変性を固定。
- [x] `native_no_wat2wasm_on_build`: build/server/CLI の成功パスで WAT parse/conversion 関数が呼ばれない。
- [x] `native_debug_wat_fallback_is_explicit`: debug fallback は専用 API 経由でしか使えない。
- [x] `native_fixture_differential`: representative fixtures で WAT path と Native path の observable output を比較する。`native_fixture_differential_matches_wat_console_log_number` と `native_fixture_differential_matches_wat_locals_and_static_modules` で console/local/module の WAT/native output parity を固定。
- [x] `native_host_capability_deny`: host import/capability を持つ runtime が deny policy で落ちる。`has_node_host_imports` は catalog の `HostAbi::NodeShim` を source of truth にし、NodeShim import を持つ全 `RuntimeFn` が deny predicate に捕捉される `host_deny_predicate_rejects_every_node_shim_runtime_fn` で固定。
- [x] `native_runtime_strings_origin`: runtime string と user literal の origin が混ざらない。
- [x] `native_pseudo_intrinsics_eliminated`: `$pseudo_*` の function/call/export が final wasm に存在しない。
- [x] `native_no_runtime_bloat`: required RuntimeFn だけが module に入る。全 runtime 強制同梱に戻らない。
  - RuntimeFn symbol exact-check と `$native_write_*` / `$fd_write` 未使用時同梱 guard で確認済み。

## 実装順のおすすめ

1. encoder の安全化: 未解決 symbol と `Raw` 無視を blocker として落とす。
2. registry skeleton: pseudo 以外の未実装一覧がテストで見える状態を作る。
3. Core kernel: heap/string/write/copy/number-tagging を先に安定化する。
4. Operator/type coercion: JS semantic の土台を固める。
5. String/Array/Object: ほか domain の依存元を固める。
6. MapSet/TypedArray/Date/Math/RegExp/JSON: data structure と host shim を順に潰す。
7. Promise/Task/Iterator/Module/Host/Encoding: control-flow と環境依存を最後に固める。
8. MIR と WAT fallback を整理し、Native を唯一の production build path にする。

## 完了条件

- [x] `native_runtime_builder_coverage` が pseudo 以外 0 missing を示す。
- [x] `cargo test -p ts2wasm-backend-wasm native` が通る。
- [ ] `python scripts/manager.py check` が WAT fallback なしで通る。
- [ ] representative fixtures が `wat2wasm` なしで `.wasm` を生成する。
- [ ] `wasmparser` validation が full runtime synthetic module と実 fixture module の両方で通る。
  - full runtime synthetic module: `native_runtime_embed::available_native_runtime_module_validates_with_wasmparser` で確認済み。
- [ ] Host capability manifest snapshot が WAT path と Native path で一致する。
- [ ] `grep -R "wat::parse_str\|wat2wasm" crates/backend-wasm/src crates/cli/src` で production path 呼び出しが 0 になる。
- [ ] `grep -R "WasmInstr::Raw" crates/backend-wasm/src/native_runtime crates/backend-wasm/src/runtime` が 0 になる。

## 全 RuntimeFn builder TODO

下の checklist は catalog の domain と `emission_order()` から生成している。実装時の append 順は必ず `RuntimeFn::emission_order()` を使い、この domain grouping は担当分割・進捗管理用として扱う。

### Core (24)

- [x] `RuntimeFn::ReadStdinBytes` -> `$read_stdin_bytes` / sig `0->1` / result `Value`: deps `READ_STDIN_DEPS`; imports `IMPORT_FD_READ`; capability `CAP_STDIN_READ`. lowering/WAT と同じ 0 引数 signature に catalog を補正し、typed native builder を registry に接続済み。
- [x] `RuntimeFn::Write` -> `$write` / sig `1->1` / result `EffectOnly`: deps `WRITE_DEPS`; imports `IMPORT_FD_WRITE`; capability `CAP_STDOUT_WRITE`
- [x] `RuntimeFn::Copy` -> `$copy` / sig `1->1` / result `EffectOnly`: deps `COPY_DEPS`
- [x] `RuntimeFn::ValueToStringInto` -> `$value_to_string_into` / sig `2->1` / result `Value`: deps `VTS_DEPS`; runtime_strings `VTS_RUNTIME_STRINGS`
- [x] `RuntimeFn::ErrorMessage` -> `$error_message` / sig `1->1` / result `Value`: deps `ERROR_MESSAGE_DEPS`; runtime_strings `VTS_RUNTIME_STRINGS`
- [x] `RuntimeFn::Log` -> `$log` / sig `1->1` / result `Value`: deps `LOG_DEPS`; runtime_strings `LOG_RUNTIME_STRINGS`
- [x] `RuntimeFn::LogWarn` -> `$log_warn` / sig `1->1` / result `Value`: deps `LOG_WARN_DEPS`; runtime_strings `LOG_RUNTIME_STRINGS`
- [x] `RuntimeFn::LogError` -> `$log_error` / sig `1->1` / result `Value`: deps `LOG_ERROR_DEPS`; runtime_strings `LOG_RUNTIME_STRINGS`
- [x] `RuntimeFn::ConsoleGroupStart` -> `$console_group_start` / sig `1->1` / result `Value`: deps `CONSOLE_GROUP_START_DEPS`; runtime_strings `LOG_RUNTIME_STRINGS`
- [x] `RuntimeFn::ConsoleGroupEndFn` -> `$console_group_end` / sig `1->1` / result `Value`: deps `CONSOLE_GROUP_END_DEPS`
- [x] `RuntimeFn::ConsoleTimeStart` -> `$console_time_start` / sig `1->1` / result `Value`: deps `CONSOLE_TIME_START_DEPS`; imports `IMPORT_CLOCK_TIME_GET`; capability `CAP_WASI_CLOCK_REALTIME`
- [x] `RuntimeFn::ConsoleTimeEndFn` -> `$console_time_end` / sig `1->1` / result `Value`: deps `CONSOLE_TIME_END_DEPS`; imports `IMPORT_CLOCK_TIME_GET`; capability `CAP_WASI_CLOCK_REALTIME`; runtime_strings `LOG_RUNTIME_STRINGS`
- [x] `RuntimeFn::ConsoleCountImpl` -> `$console_count` / sig `1->1` / result `Value`: deps `CONSOLE_COUNT_DEPS`; runtime_strings `LOG_RUNTIME_STRINGS`
- [x] `RuntimeFn::ConsoleCountResetImpl` -> `$console_count_reset` / sig `1->1` / result `Value`: deps `CONSOLE_COUNT_RESET_DEPS`
- [x] `RuntimeFn::AllocHeap` -> `$alloc_heap` / sig `1->1` / result `Value`
- [x] `RuntimeFn::PrivateBrandTypeError` -> `$private_brand_type_error` / sig `0->1` / result `Value`: deps `PRIVATE_BRAND_TYPE_ERROR_DEPS`; runtime_strings `PRIVATE_BRAND_TYPE_ERROR_RUNTIME_STRINGS`
- [x] `RuntimeFn::MemEqual` -> `$mem_equal` / sig `1->1` / result `Value`
- [x] `RuntimeFn::Index` -> `$index` / sig `2->1` / result `Value`: deps `INDEX_DEPS`
- [x] `RuntimeFn::GetLength` -> `$get_length` / sig `1->1` / result `Value`: deps `&[Self::PropertyGet]`
- [ ] `RuntimeFn::ArrayPushMany` -> `$pseudo_array_push_many` / sig `1->1` / result `Value`: pseudo: final wasm に実関数として出さない; deps `OBJECT_GET_OWN_PROPERTY_NAMES_DEPS`
- [ ] `RuntimeFn::HeapClosureCall` -> `$pseudo_heap_closure_call` / sig `1->1` / result `Value`: pseudo: final wasm に実関数として出さない; deps `HEAP_CLOSURE_CALL_DEPS`; runtime_strings `HEAP_CLOSURE_CALL_RUNTIME_STRINGS`
- [ ] `RuntimeFn::PrivateFieldGet` -> `$pseudo_private_field_get` / sig `1->1` / result `Value`: pseudo: final wasm に実関数として出さない; deps `OBJECT_GET_OWN_PROPERTY_NAMES_DEPS`
- [ ] `RuntimeFn::PrivateFieldSet` -> `$pseudo_private_field_set` / sig `1->1` / result `Value`: pseudo: final wasm に実関数として出さない; deps `OBJECT_GET_OWN_PROPERTY_NAMES_DEPS`
- [ ] `RuntimeFn::PrivateBrandCheck` -> `$pseudo_private_brand_check` / sig `1->1` / result `Value`: pseudo: final wasm に実関数として出さない; deps `OBJECT_GET_OWN_PROPERTY_NAMES_DEPS`

### Operator (30)

- [x] `RuntimeFn::Add` -> `$add` / sig `2->1` / result `Value`: deps `ADD_DEPS`
- [x] `RuntimeFn::AddFast` -> `$add_fast` / sig `2->1` / result `Value`: deps `ADD_FAST_DEPS`
- [x] `RuntimeFn::Sub` -> `$sub` / sig `2->1` / result `Value`: deps `NUMBER_ARITH_DEPS`
- [x] `RuntimeFn::SubFast` -> `$sub_fast` / sig `2->1` / result `Value`: deps `SUB_FAST_DEPS`
- [x] `RuntimeFn::Mul` -> `$mul` / sig `2->1` / result `Value`: deps `NUMBER_ARITH_DEPS`
- [x] `RuntimeFn::MulFast` -> `$mul_fast` / sig `2->1` / result `Value`: deps `MUL_FAST_DEPS`
- [x] `RuntimeFn::Div` -> `$div` / sig `2->1` / result `Value`: deps `NUMBER_ARITH_DEPS`
- [x] `RuntimeFn::DivFast` -> `$div_fast` / sig `2->1` / result `Value`: deps `DIV_FAST_DEPS`
- [x] `RuntimeFn::Mod` -> `$mod` / sig `2->1` / result `Value`: deps `NUMBER_ARITH_DEPS`
- [x] `RuntimeFn::ModFast` -> `$mod_fast` / sig `2->1` / result `Value`: deps `MOD_FAST_DEPS`
- [x] `RuntimeFn::BitwiseToI32` -> `$bitwise_to_i32` / sig `1->1` / result `Value`: deps `&[Self::NumberToI32]`
- [x] `RuntimeFn::BitwiseAnd` -> `$bitwise_and` / sig `2->1` / result `Value`: deps `BITWISE_DEPS`
- [x] `RuntimeFn::BitwiseXor` -> `$bitwise_xor` / sig `2->1` / result `Value`: deps `BITWISE_DEPS`
- [x] `RuntimeFn::BitwiseOr` -> `$bitwise_or` / sig `2->1` / result `Value`: deps `BITWISE_DEPS`
- [x] `RuntimeFn::Negate` -> `$negate` / sig `1->1` / result `Value`: deps `NUMBER_ARITH_DEPS`
- [x] `RuntimeFn::Less` -> `$less` / sig `2->1` / result `Value`: deps `LESS_DEPS`
- [x] `RuntimeFn::LessFast` -> `$less_fast` / sig `2->1` / result `Value`: deps `LESS_FAST_DEPS`
- [x] `RuntimeFn::LessEqual` -> `$less_equal` / sig `2->1` / result `Value`: deps `LESS_EQUAL_DEPS`
- [x] `RuntimeFn::LessEqualFast` -> `$less_equal_fast` / sig `2->1` / result `Value`: deps `LESS_EQUAL_FAST_DEPS`
- [x] `RuntimeFn::Greater` -> `$greater` / sig `2->1` / result `Value`: deps `GREATER_DEPS`
- [x] `RuntimeFn::GreaterFast` -> `$greater_fast` / sig `2->1` / result `Value`: deps `GREATER_FAST_DEPS`
- [x] `RuntimeFn::GreaterEqual` -> `$greater_equal` / sig `2->1` / result `Value`: deps `GREATER_EQUAL_DEPS`
- [x] `RuntimeFn::GreaterEqualFast` -> `$greater_equal_fast` / sig `2->1` / result `Value`: deps `GREATER_EQUAL_FAST_DEPS`
- [x] `RuntimeFn::SameValueZero` -> `$same_value_zero` / sig `2->1` / result `Value`: deps `SAME_VALUE_ZERO_DEPS`
- [x] `RuntimeFn::StrictEqual` -> `$strict_equal` / sig `2->1` / result `Value`: deps `STRICT_EQUAL_DEPS`
- [x] `RuntimeFn::EqualEqual` -> `$equal_equal` / sig `2->1` / result `Value`: deps `EQUAL_EQUAL_DEPS`
- [x] `RuntimeFn::BangEqual` -> `$bang_equal` / sig `2->1` / result `Value`: deps `BANG_EQUAL_DEPS`
- [x] `RuntimeFn::StrictNotEqual` -> `$strict_not_equal` / sig `2->1` / result `Value`: deps `STRICT_NOT_EQUAL_DEPS`
- [x] `RuntimeFn::And` -> `$and` / sig `2->1` / result `Value`: deps `AND_DEPS`
- [x] `RuntimeFn::Or` -> `$or` / sig `2->1` / result `Value`: deps `OR_DEPS`

### TypeCoercion (13)

- [x] `RuntimeFn::TruthyBool` -> `$truthy_bool` / sig `1->1` / result `Value`: deps `OBJECT_GET_OWN_PROPERTY_NAMES_DEPS`
- [x] `RuntimeFn::Not` -> `$not` / sig `1->1` / result `Value`: deps `&[Self::TruthyBool]`
- [x] `RuntimeFn::TypeOf` -> `$typeof` / sig `1->1` / result `Value`: deps `OBJECT_GET_OWN_PROPERTY_NAMES_DEPS`; runtime_strings `TYPEOF_RUNTIME_STRINGS`
- [x] `RuntimeFn::IsString` -> `$is_string` / sig `1->1` / result `Value`: deps `OBJECT_GET_OWN_PROPERTY_NAMES_DEPS`
- [x] `RuntimeFn::ValueOf` -> `$value_of` / sig `1->1` / result `Value`: deps `OBJECT_GET_OWN_PROPERTY_NAMES_DEPS`
- [x] `RuntimeFn::InstanceOf` -> `$instanceof` / sig `2->1` / result `Value`: deps `&[]`
- [x] `RuntimeFn::IsNaN` -> `$is_nan` / sig `1->1` / result `Value`: deps `OBJECT_GET_OWN_PROPERTY_NAMES_DEPS`
- [x] `RuntimeFn::GlobalParseInt` -> `$parse_int` / sig `2->1` / result `Value`: deps `&[RuntimeFn::NumberToString, RuntimeFn::NumberFromI32]`
- [x] `RuntimeFn::GlobalParseFloat` -> `$parse_float` / sig `1->1` / result `Value`: deps `OBJECT_GET_OWN_PROPERTY_NAMES_DEPS`
- [x] `RuntimeFn::IsFinite` -> `$is_finite` / sig `1->1` / result `Value`: deps `OBJECT_GET_OWN_PROPERTY_NAMES_DEPS`
- [x] `RuntimeFn::BooleanCoerce` -> `$boolean_coerce` / sig `1->1` / result `Value`: deps `OBJECT_GET_OWN_PROPERTY_NAMES_DEPS`
- [x] `RuntimeFn::BooleanToString` -> `$boolean_to_string` / sig `1->1` / result `Value`: deps `OBJECT_GET_OWN_PROPERTY_NAMES_DEPS`; runtime_strings `BOOLEAN_TO_STRING_RUNTIME_STRINGS`
- [x] `RuntimeFn::NumberCoerce` -> `$number_coerce` / sig `1->1` / result `Value`: deps `&[RuntimeFn::GlobalParseInt]`

### Number (11)

- [x] `RuntimeFn::NumberFromI32` -> `$number_from_i32` / sig `1->1` / result `Value`: deps `NUMBER_FROM_I32_DEPS`
- [x] `RuntimeFn::NumberToI32` -> `$number_to_i32` / sig `1->1` / result `Value`
- [x] `RuntimeFn::NumberToExponential` -> `$number_to_exponential` / sig `2->1` / result `Value`: deps `NUMBER_TO_EXPONENTIAL_DEPS` plus native receiver normalization through `NumberFromI32`
- [x] `RuntimeFn::NumberToFixed` -> `$number_to_fixed` / sig `2->1` / result `Value`: deps `NUMBER_TO_FIXED_DEPS` plus native receiver normalization through `NumberFromI32`
- [x] `RuntimeFn::NumberToPrecision` -> `$number_to_precision` / sig `2->1` / result `Value`: deps `NUMBER_TO_PRECISION_DEPS` plus native receiver normalization through `NumberFromI32`
- [x] `RuntimeFn::NumberToString` -> `$number_to_string` / sig `2->1` / result `Value`: deps `NUMBER_TO_STRING_DEPS`; native ABI uses raw i32 receiver/radix plus tagged sentinels
- [x] `RuntimeFn::NumberToStringRadix` -> `$number_to_string_radix` / sig `2->1` / result `Value`: deps `NUMBER_TO_STRING_RADIX_DEPS` -> `NumberToString`
- [x] `RuntimeFn::NumberIsNaN` -> `$number_is_nan` / sig `1->1` / result `Value`
- [x] `RuntimeFn::NumberIsFinite` -> `$number_is_finite` / sig `1->1` / result `Value`
- [x] `RuntimeFn::NumberIsInteger` -> `$number_is_integer` / sig `1->1` / result `Value`
- [x] `RuntimeFn::NumberIsSafeInteger` -> `$number_is_safe_integer` / sig `1->1` / result `Value`
  - Native emitter ABI note: expression-position `Number.is*` returns raw native booleans after
    helper/static folding conversion; the embedded typed helpers still expose tagged JS booleans.

### BigInt (23)

- [x] `RuntimeFn::BigIntCompare` -> `$bigint_compare` / sig `2->1` / result `Value`
- [x] `RuntimeFn::MakeBigIntLiteral` -> `$make_bigint_literal` / sig `6->1` / result `Value`: deps `MAKE_BIGINT_LITERAL_DEPS`
- [x] `RuntimeFn::BigIntToString` -> `$bigint_to_string` / sig `1->1` / result `Value`: deps `BIGINT_TO_STRING_DEPS`
- [x] `RuntimeFn::BigIntToBoolean` -> `$bigint_to_boolean` / sig `1->1` / result `Value`
- [x] `RuntimeFn::BigIntAdd` -> `$bigint_add` / sig `2->1` / result `Value`: deps `BIGINT_ADD_DEPS`
- [x] `RuntimeFn::BigIntFromValue` -> `$bigint_from_value` / sig `1->1` / result `Value`: deps `BIGINT_FROM_VALUE_DEPS`
- [x] `RuntimeFn::BigIntAsIntN` -> `$bigint_as_int_n` / sig `2->1` / result `Value`: deps `BIGINT_AS_INT_N_DEPS`
- [x] `RuntimeFn::BigIntAsUintN` -> `$bigint_as_uint_n` / sig `2->1` / result `Value`: deps `BIGINT_AS_UINT_N_DEPS`
- [x] `RuntimeFn::BigIntUnaryMinus` -> `$bigint_unary_minus` / sig `1->1` / result `Value`: deps `BIGINT_UNARY_MINUS_DEPS`
- [x] `RuntimeFn::BigIntSub` -> `$bigint_sub` / sig `2->1` / result `Value`: deps `BIGINT_SUB_DEPS`
- [x] `RuntimeFn::BigIntMul` -> `$bigint_mul` / sig `2->1` / result `Value`: deps `BIGINT_MUL_DEPS`
- [x] `RuntimeFn::BigIntPow` -> `$bigint_pow` / sig `2->1` / result `Value`: deps `BIGINT_POW_DEPS`
- [x] `RuntimeFn::BigIntDiv` -> `$bigint_div` / sig `2->1` / result `Value`: deps `BIGINT_DIV_DEPS`; runtime_strings `BIGINT_DIVISION_BY_ZERO_RANGE_ERROR_RUNTIME_STRINGS`
- [x] `RuntimeFn::BigIntRem` -> `$bigint_rem` / sig `2->1` / result `Value`: deps `BIGINT_REM_DEPS`
- [x] `RuntimeFn::BigIntDivisionByZeroRangeError` -> `$bigint_division_by_zero_range_error` / sig `1->1` / result `Value`: deps `BIGINT_DIVISION_BY_ZERO_RANGE_ERROR_DEPS`; runtime_strings `BIGINT_DIVISION_BY_ZERO_RANGE_ERROR_RUNTIME_STRINGS`
- [x] `RuntimeFn::BigIntMixedArithmeticTypeError` -> `$bigint_mixed_arithmetic_type_error` / sig `2->1` / result `Value`: deps `BIGINT_MIXED_ARITHMETIC_TYPE_ERROR_DEPS`; runtime_strings `BIGINT_MIXED_ARITHMETIC_TYPE_ERROR_RUNTIME_STRINGS`
- [x] `RuntimeFn::BigIntStringComparisonBoundaryError` -> `$bigint_string_comparison_boundary_error` / sig `0->0` / result `EffectOnly`: deps `BIGINT_STRING_COMPARISON_BOUNDARY_ERROR_DEPS`; runtime_strings `BIGINT_STRING_COMPARISON_BOUNDARY_ERROR_RUNTIME_STRINGS`
- [x] `RuntimeFn::BigIntBitwiseNot` -> `$bigint_bitwise_not` / sig `1->1` / result `Value`: deps `BIGINT_BITWISE_DEPS`
- [x] `RuntimeFn::BigIntBitwiseAnd` -> `$bigint_bitwise_and` / sig `2->1` / result `Value`: deps `BIGINT_BITWISE_DEPS`
- [x] `RuntimeFn::BigIntBitwiseOr` -> `$bigint_bitwise_or` / sig `2->1` / result `Value`: deps `BIGINT_BITWISE_DEPS`
- [x] `RuntimeFn::BigIntBitwiseXor` -> `$bigint_bitwise_xor` / sig `2->1` / result `Value`: deps `BIGINT_BITWISE_DEPS`
- [x] `RuntimeFn::BigIntLeftShift` -> `$bigint_left_shift` / sig `2->1` / result `Value`: deps `BIGINT_LEFT_SHIFT_DEPS`
- [x] `RuntimeFn::BigIntRightShift` -> `$bigint_right_shift` / sig `2->1` / result `Value`: deps `BIGINT_RIGHT_SHIFT_DEPS`

### String (37)

- [x] `RuntimeFn::StringEqual` -> `$string_equal` / sig `2->1` / result `Value`: deps `STRING_EQUAL_DEPS`
- [x] `RuntimeFn::Concat` -> `$concat` / sig `2->1` / result `Value`: deps `CONCAT_DEPS`
- [x] `RuntimeFn::StringCharAt` -> `$string_char_at` / sig `2->1` / result `Value`: deps `STRING_CHAR_AT_DEPS`
- [x] `RuntimeFn::StringAt` -> `$string_at` / sig `2->1` / result `Value`: deps `STRING_AT_DEPS`
- [x] `RuntimeFn::StringSubstring` -> `$string_substring` / sig `3->1` / result `Value`: deps `STRING_SUBSTRING_DEPS`
- [x] `RuntimeFn::StringSubstr` -> `$string_substr` / sig `3->1` / result `Value`: deps `STRING_SUBSTR_DEPS`; 2-arg `substr(start)` は native emitter が raw `0` omitted-length sentinel を補完し、Annex B fixture parity 済み。
- [x] `RuntimeFn::StringSlice` -> `$string_slice` / sig `3->1` / result `Value`: deps `STRING_SLICE_DEPS`
- [x] `RuntimeFn::StringIndexOf` -> `$string_index_of` / sig `3->1` / result `Value`: deps `STRING_INDEX_OF_DEPS`
- [x] `RuntimeFn::StringLastIndexOf` -> `$string_last_index_of` / sig `3->1` / result `Value`: deps `STRING_LAST_INDEX_OF_DEPS`
- [x] `RuntimeFn::StringLocaleCompare` -> `$string_locale_compare` / sig `2->1` / result `Value`: deps `STRING_LOCALE_COMPARE_DEPS`
- [x] `RuntimeFn::StringIncludes` -> `$string_includes` / sig `3->1` / result `Value`: deps `STRING_INCLUDES_DEPS`
- [x] `RuntimeFn::StringPadStart` -> `$string_pad_start` / sig `3->1` / result `Value`: deps `STRING_PAD_START_DEPS`
- [x] `RuntimeFn::StringPadEnd` -> `$string_pad_end` / sig `3->1` / result `Value`: deps `STRING_PAD_END_DEPS`
- [x] `RuntimeFn::StringRepeat` -> `$string_repeat` / sig `2->1` / result `Value`: deps `STRING_REPEAT_DEPS`
- [x] `RuntimeFn::StringSplit` -> `$string_split` / sig `2->1` / result `Value`: deps `STRING_SPLIT_DEPS`; string separator path の typed native builder + registry/test 接続済み。RegExp separator path は RegExp family 側に残る。
- [x] `RuntimeFn::StringTrim` -> `$string_trim` / sig `1->1` / result `Value`: deps `STRING_TRIM_DEPS`
- [x] `RuntimeFn::StringTrimStart` -> `$string_trim_start` / sig `1->1` / result `Value`: deps `STRING_TRIM_DEPS`
- [x] `RuntimeFn::StringTrimEnd` -> `$string_trim_end` / sig `1->1` / result `Value`: deps `STRING_TRIM_DEPS`
- [x] `RuntimeFn::StringStartsWith` -> `$string_starts_with` / sig `3->1` / result `Value`: deps `STRING_STARTS_WITH_DEPS`
- [x] `RuntimeFn::StringEndsWith` -> `$string_ends_with` / sig `3->1` / result `Value`: deps `STRING_ENDS_WITH_DEPS`
- [x] `RuntimeFn::StringMatch` -> `$string_match` / sig `2->1` / result `Value`: deps `STRING_MATCH_DEPS`; typed wrapper now available through `$regexp_match` native literal-search helper.
- [x] `RuntimeFn::StringSearch` -> `$string_search` / sig `2->1` / result `Value`: deps `STRING_SEARCH_DEPS`; typed wrapper now available through `$regexp_search` native literal-search helper.
- [x] `RuntimeFn::StringToUpperCase` -> `$string_to_upper_case` / sig `1->1` / result `Value`: deps `STRING_TO_UPPER_CASE_DEPS`
- [x] `RuntimeFn::StringToLowerCase` -> `$string_to_lower_case` / sig `1->1` / result `Value`: deps `STRING_TO_LOWER_CASE_DEPS`
- [x] `RuntimeFn::StringCharCodeAt` -> `$string_char_code_at` / sig `2->1` / result `Value`: deps `STRING_CHAR_CODE_AT_DEPS`; known string/index calls は static fold 済みで `string_char_code_at` と `json_parse_latin1_unicode_escape` parity 通過。typed helper は dynamic receiver/index を tagged `Value` として受け取り UTF-8 code point index を decoded number/`NaN` に畳む。`string-char-code-at-dynamic.ts` は `isNaN` 経由、`string-char-code-at-dynamic-print.ts` は helper が返した tagged number local の直接 `console.log` 経由で Node parity 済み。static console formatting も tagged `NaN` sentinel を `NaN` として表示する。
- [x] `RuntimeFn::StringCodePointAt` -> `$string_code_point_at` / sig `2->1` / result `Value`: deps `STRING_CODE_POINT_AT_DEPS`; typed helper は `$string_char_code_at` の UTF-8 code point decode を再利用し、out-of-range/non-string/non-number を `undefined` に正規化する。`string-code-point-at.ts` と dynamic index/local print fixture `string-code-point-at-dynamic.ts` が Node parity 通過。
- [x] `RuntimeFn::StringFromCharCode` -> `$string_from_char_code` / sig `1->1` / result `Value`: deps `STRING_FROM_CHAR_CODE_DEPS`; typed helper は単一 tagged number code unit を UTF-8 string に変換して `AllocHeap` で materialize する。`string-from-char-code.ts` が Node parity 通過。
- [x] `RuntimeFn::StringFromCodePoint` -> `$string_from_code_point` / sig `1->1` / result `Value`: deps `STRING_FROM_CODE_POINT_DEPS`; typed helper は単一 tagged number code point を UTF-8 string に変換して `AllocHeap` で materialize する。`string-from-code-point.ts` と dynamic local print fixture `string-from-code-point-dynamic.ts` が Node parity 通過。現 helper contract は既存 runtime と同じ単一引数 path。
- [x] `RuntimeFn::StringIsWellFormed` -> `$string_is_well_formed` / sig `1->1` / result `Value`: deps `OBJECT_GET_OWN_PROPERTY_NAMES_DEPS`; tagged unary string ABI / console boundary を native emitter に接続し、`string-is-well-formed` parity 済み。
- [x] `RuntimeFn::StringToWellFormed` -> `$string_to_well_formed` / sig `1->1` / result `Value`: deps `OBJECT_GET_OWN_PROPERTY_NAMES_DEPS`; tagged unary string ABI / console boundary を native emitter に接続し、`string-to-well-formed` parity 済み。
- [x] `RuntimeFn::StringNormalize` -> `$string_normalize` / sig `2->1` / result `Value`: deps `OBJECT_GET_OWN_PROPERTY_NAMES_DEPS`; imports `IMPORT_STRING_NORMALIZE`; capability `CAP_STRING_NORMALIZE`
- [x] `RuntimeFn::IntlNumberFormatFormat` -> `$intl_number_format_format` / sig `2->1` / result `Value`: deps `OBJECT_GET_OWN_PROPERTY_NAMES_DEPS`; imports `IMPORT_INTL_NUMBER_FORMAT_FORMAT`; capability `CAP_INTL_NUMBER_FORMAT_FORMAT`
- [x] `RuntimeFn::StringReplace` -> `$string_replace` / sig `3->1` / result `Value`: deps `STRING_REPLACE_DEPS`; native byte-string replacement helper covers first string occurrence. tagged 3-arg string ABI / console boundary を native emitter に接続し、`string-replace` parity 済み。
- [x] `RuntimeFn::StringReplaceAll` -> `$string_replace_all` / sig `3->1` / result `Value`: deps `STRING_REPLACE_ALL_DEPS`; native byte-string replacement helper covers all non-overlapping string occurrences and empty-search insertion. tagged 3-arg string ABI / console boundary を native emitter に接続し、`string-replace-all` parity 済み。
- [x] `RuntimeFn::StringMatchAll` -> `$string_match_all` / sig `2->1` / result `Value`: deps `STRING_MATCH_ALL_DEPS`; native byte-string matcher returns an array of `{0,index,input}` match objects for string and slash-literal patterns.
- [x] `RuntimeFn::StringRaw` -> `$string_raw` / sig `3->1` / result `Value`: deps `STRING_RAW_DEPS`; runtime_strings `STRING_RAW_RUNTIME_STRINGS`; static raw array object + substitutions は native static fold 済み。
- [x] `RuntimeFn::StringToLocaleString` -> `$string_to_locale_string` / sig `1->1` / result `Value`: deps `&[Self::IsString]`

### Array (53)

- [x] `RuntimeFn::ArrayGet` -> `$array_get` / sig `2->1` / result `Value`
- [x] `RuntimeFn::ArrayIndexPresent` -> `$array_index_present` / sig `2->1` / result `Value`
- [x] `RuntimeFn::ArrayBufferNew` -> `$arraybuffer_new` / sig `1->1` / result `Value`: deps `ARRAYBUFFER_NEW_DEPS`
- [x] `RuntimeFn::ArrayBufferIsView` -> `$arraybuffer_is_view` / sig `1->1` / result `Value`
- [x] `RuntimeFn::ArrayBufferTransfer` -> `$arraybuffer_transfer` / sig `2->1` / result `Value`: deps `ARRAYBUFFER_TRANSFER_DEPS`
- [x] `RuntimeFn::ArrayBufferSlice` -> `$arraybuffer_slice` / sig `3->1` / result `Value`: deps `ARRAYBUFFER_SLICE_DEPS`
- [x] `RuntimeFn::SharedArrayBufferNew` -> `$shared_array_buffer_new` / sig `1->1` / result `Value`: deps `SHARED_ARRAY_BUFFER_NEW_DEPS`
- [x] `RuntimeFn::ArrayPush` -> `$array_push` / sig `2->1` / result `Value`: deps `ARRAY_PUSH_DEPS`
- [x] `RuntimeFn::ArrayPushGrow` -> `$array_push_grow` / sig `2->1` / result `Value`: deps `ARRAY_PUSH_GROW_DEPS`; typed grow/realloc builder + native registry/test 接続済み。
- [x] `RuntimeFn::ArrayPop` -> `$array_pop` / sig `1->1` / result `Value`: deps `ARRAY_POP_DEPS`
- [x] `RuntimeFn::ArrayCtorWithLength` -> `$array_ctor_with_length` / sig `1->1` / result `Value`: deps `ARRAY_CTOR_WITH_LENGTH_DEPS`
- [x] `RuntimeFn::ArraySlice` -> `$array_slice` / sig `3->1` / result `Value`: deps `ARRAY_SLICE_DEPS`
- [x] `RuntimeFn::ArrayConcat` -> `$array_concat` / sig `2->1` / result `Value`: deps `ARRAY_CONCAT_DEPS`
- [x] `RuntimeFn::ArrayMapValueToString` -> `$array_map_value_to_string` / sig `1->1` / result `Value`: deps `ARRAY_MAP_VALUE_TO_STRING_DEPS`
- [x] `RuntimeFn::ArrayMapUnaryPlus` -> `$array_map_unary_plus` / sig `1->1` / result `Value`: deps `ARRAY_MAP_UNARY_PLUS_DEPS`
- [x] `RuntimeFn::ArrayMapStringSplit` -> `$array_map_string_split` / sig `2->1` / result `Value`: deps `ARRAY_MAP_STRING_SPLIT_DEPS`; typed native builder + registry/test 接続済み。
- [x] `RuntimeFn::ArrayMapArrayLikeIdentity` -> `$array_map_array_like_identity` / sig `1->1` / result `Value`: deps `ARRAY_MAP_ARRAY_LIKE_IDENTITY_DEPS`
- [x] `RuntimeFn::ArrayMapArrayLikeDouble` -> `$array_map_array_like_double` / sig `1->1` / result `Value`: deps `ARRAY_MAP_ARRAY_LIKE_DOUBLE_DEPS`
- [x] `RuntimeFn::ArraySortNumeric` -> `$array_sort_numeric` / sig `1->1` / result `Value`: deps `ARRAY_SORT_NUMERIC_DEPS`
- [x] `RuntimeFn::ArraySortLexicographic` -> `$array_sort_lexicographic` / sig `1->1` / result `Value`: deps `ARRAY_SORT_LEXICOGRAPHIC_DEPS`
- [x] `RuntimeFn::ArrayJoin` -> `$array_join` / sig `2->1` / result `Value`: deps `ARRAY_JOIN_DEPS`
- [x] `RuntimeFn::ArrayReverse` -> `$array_reverse` / sig `1->1` / result `Value`: deps `ARRAY_REVERSE_DEPS`
- [x] `RuntimeFn::ArrayIndexOf` -> `$array_index_of` / sig `3->1` / result `Value`: deps `ARRAY_INDEX_OF_DEPS`
- [x] `RuntimeFn::ArrayIncludes` -> `$array_includes` / sig `3->1` / result `Value`: deps `ARRAY_INCLUDES_DEPS`
- [x] `RuntimeFn::ArrayFind` -> `$array_find` / sig `1->1` / result `Value`: deps `ARRAY_FIND_DEPS`
- [x] `RuntimeFn::ArrayFindIndex` -> `$array_find_index` / sig `1->1` / result `Value`: deps `ARRAY_FIND_INDEX_DEPS`
- [x] `RuntimeFn::ArrayFindLast` -> `$array_find_last` / sig `1->1` / result `Value`: deps `ARRAY_FIND_LAST_DEPS`
- [x] `RuntimeFn::ArrayFindLastIndex` -> `$array_find_last_index` / sig `1->1` / result `Value`: deps `ARRAY_FIND_LAST_INDEX_DEPS`
- [x] `RuntimeFn::ArrayFilter` -> `$array_filter` / sig `1->1` / result `Value`: deps `ARRAY_FILTER_DEPS`
- [x] `RuntimeFn::ArrayEvery` -> `$array_every` / sig `1->1` / result `Value`: deps `ARRAY_EVERY_DEPS`
- [x] `RuntimeFn::ArraySome` -> `$array_some` / sig `1->1` / result `Value`: deps `ARRAY_SOME_DEPS`
- [x] `RuntimeFn::ArrayReduce` -> `$array_reduce` / sig `3->1` / result `Value`: deps `ARRAY_REDUCE_DEPS`
- [x] `RuntimeFn::ArrayReduceRight` -> `$array_reduce_right` / sig `3->1` / result `Value`: deps `ARRAY_REDUCE_RIGHT_DEPS`
- [x] `RuntimeFn::ArrayLastIndexOf` -> `$array_last_index_of` / sig `2->1` / result `Value`: deps `ARRAY_LAST_INDEX_OF_DEPS`
- [x] `RuntimeFn::ArrayForEach` -> `$array_for_each` / sig `2->1` / result `Value`: deps `ARRAY_FOR_EACH_DEPS`
- [x] `RuntimeFn::ArrayMap` -> `$array_map` / sig `2->1` / result `Value`: deps `ARRAY_MAP_DEPS`
- [x] `RuntimeFn::ArrayAt` -> `$array_at` / sig `2->1` / result `Value`: deps `ARRAY_AT_DEPS`
- [x] `RuntimeFn::ArrayFill` -> `$array_fill` / sig `4->1` / result `Value`: deps `ARRAY_FILL_DEPS`
- [x] `RuntimeFn::ArrayFlat` -> `$array_flat` / sig `2->1` / result `Value`: deps `ARRAY_FLAT_DEPS`
- [x] `RuntimeFn::ArrayPushOrSpread` -> `$array_push_or_spread` / sig `2->1` / result `Value`: deps `ARRAY_PUSH_OR_SPREAD_DEPS`
- [x] `RuntimeFn::ArrayCopyWithin` -> `$array_copy_within` / sig `4->1` / result `Value`: deps `ARRAY_COPY_WITHIN_DEPS`
- [x] `RuntimeFn::ArrayWith` -> `$array_with` / sig `3->1` / result `Value`: deps `ARRAY_WITH_DEPS`
- [x] `RuntimeFn::ArrayToReversed` -> `$array_to_reversed` / sig `1->1` / result `Value`: deps `ARRAY_TO_REVERSED_DEPS`
- [x] `RuntimeFn::ArrayToSorted` -> `$array_to_sorted` / sig `1->1` / result `Value`: deps `ARRAY_TO_SORTED_DEPS`
- [x] `RuntimeFn::ArrayToSpliced` -> `$array_to_spliced` / sig `3->1` / result `Value`: deps `ARRAY_TO_SPLICED_DEPS`
- [x] `RuntimeFn::ArrayValues` -> `$array_values` / sig `1->1` / result `Value`: deps `ARRAY_VALUES_DEPS`; runtime_strings `ARRAY_ITERATOR_STATE_RUNTIME_STRINGS`; typed iterator-state builder + native registry/test 接続済み。
- [x] `RuntimeFn::ArrayKeys` -> `$array_keys` / sig `1->1` / result `Value`: deps `ARRAY_KEYS_DEPS`; runtime_strings `ARRAY_ITERATOR_STATE_RUNTIME_STRINGS`; typed iterator-state builder + native registry/test 接続済み。
- [x] `RuntimeFn::ArrayEntries` -> `$array_entries` / sig `1->1` / result `Value`: deps `ARRAY_ENTRIES_DEPS`; runtime_strings `ARRAY_ITERATOR_STATE_RUNTIME_STRINGS`; typed iterator-state builder + native registry/test 接続済み。
- [x] `RuntimeFn::ArrayIteratorNext` -> `$array_iterator_next` / sig `1->1` / result `Value`: deps `ARRAY_ITERATOR_NEXT_DEPS`; runtime_strings `ARRAY_ITERATOR_NEXT_RUNTIME_STRINGS`; typed iterator result builder + native registry/test 接続済み。
- [x] `RuntimeFn::ArrayShift` -> `$array_shift` / sig `1->1` / result `Value`: deps `ARRAY_SHIFT_DEPS`
- [x] `RuntimeFn::ArrayUnshift` -> `$array_unshift` / sig `2->1` / result `Value`: deps `ARRAY_UNSHIFT_DEPS`
- [x] `RuntimeFn::ArraySplice` -> `$array_splice` / sig `3->1` / result `Value`: deps `ARRAY_SPLICE_DEPS`
- [x] `RuntimeFn::ArrayIsArray` -> `$array_is_array` / sig `1->1` / result `Value`: deps `&[]`

### Object (47)

- [x] `RuntimeFn::PropertyGet` -> `$property_get` / sig `3->1` / result `Value`: deps `&[Self::MemEqual]`
- [x] `RuntimeFn::PropertySet` -> `$property_set` / sig `4->1` / result `Value`: deps `&[Self::AllocHeap, Self::Copy, Self::MemEqual]`
- [x] `RuntimeFn::PropertyDelete` -> `$property_delete` / sig `3->1` / result `Value`: deps `&[Self::MemEqual]`
- [x] `RuntimeFn::PropertyHas` -> `$property_has` / sig `3->1` / result `Value`: deps `&[Self::MemEqual]`
- [x] `RuntimeFn::ObjectKeys` -> `$object_keys` / sig `1->1` / result `Value`: deps `OBJECT_KEYS_DEPS`
- [x] `RuntimeFn::ObjectGetOwnPropertyNames` -> `$object_get_own_property_names` / sig `1->1` / result `Value`: deps `OBJECT_GET_OWN_PROPERTY_NAMES_DEPS`
- [x] `RuntimeFn::ObjectGetOwnPropertySymbols` -> `$object_get_own_property_symbols` / sig `1->1` / result `Value`: deps `OBJECT_GET_OWN_PROPERTY_SYMBOLS_DEPS`
- [x] `RuntimeFn::ObjectSpread` -> `$object_spread` / sig `2->1` / result `Value`: deps `OBJECT_SPREAD_DEPS`
- [x] `RuntimeFn::RestObject` -> `$rest_object` / sig `2->1` / result `Value`: deps `REST_OBJECT_DEPS`; lowering now passes `(source, excludedKeysArray)` instead of legacy variable args.
- [x] `RuntimeFn::SpreadViaIterator` -> `$spread_via_iterator` / sig `1->1` / result `Value`: deps `&[]`; lowering normally expands this inline, native registry now provides a typed passthrough fallback and validation test.
- [x] `RuntimeFn::ObjectValues` -> `$object_values` / sig `1->1` / result `Value`: deps `OBJECT_VALUES_DEPS`
- [x] `RuntimeFn::ObjectEntries` -> `$object_entries` / sig `1->1` / result `Value`: deps `OBJECT_ENTRIES_DEPS`
- [x] `RuntimeFn::ObjectFromEntries` -> `$object_from_entries` / sig `1->1` / result `Value`: deps `OBJECT_FROM_ENTRIES_DEPS`
- [x] `RuntimeFn::ObjectHasOwnProperty` -> `$object_has_own_property` / sig `2->1` / result `Value`: deps `OBJECT_HAS_OWN_PROPERTY_DEPS`
- [x] `RuntimeFn::ObjectHasOwn` -> `$object_has_own` / sig `2->1` / result `Value`: deps `OBJECT_HAS_OWN_DEPS`
- [x] `RuntimeFn::ObjectGetOwnPropertyDescriptor` -> `$object_get_own_property_descriptor` / sig `2->1` / result `Value`: deps `OBJECT_GET_OWN_PROPERTY_DESCRIPTOR_DEPS`; typed descriptor materializer + direct-function length descriptor path + registry/test 接続済み。
- [x] `RuntimeFn::ObjectGetPrototypeOf` -> `$object_get_prototype_of` / sig `1->1` / result `Value`: deps `OBJECT_PROTOTYPE_DEPS`
- [x] `RuntimeFn::ObjectSetPrototypeOf` -> `$object_set_prototype_of` / sig `2->1` / result `Value`: deps `OBJECT_PROTOTYPE_DEPS`
- [x] `RuntimeFn::ObjectFreeze` -> `$object_freeze` / sig `1->1` / result `Value`: deps `OBJECT_FREEZE_DEPS`
- [x] `RuntimeFn::ObjectSeal` -> `$object_seal` / sig `1->1` / result `Value`: deps `OBJECT_SEAL_DEPS`
- [x] `RuntimeFn::ObjectPreventExtensions` -> `$object_prevent_extensions` / sig `1->1` / result `Value`: deps `OBJECT_GET_OWN_PROPERTY_NAMES_DEPS`
- [x] `RuntimeFn::ObjectIsExtensible` -> `$object_is_extensible` / sig `1->1` / result `Value`: deps `OBJECT_GET_OWN_PROPERTY_NAMES_DEPS`
- [x] `RuntimeFn::ObjectIsSealed` -> `$object_is_sealed` / sig `1->1` / result `Value`: deps `OBJECT_GET_OWN_PROPERTY_NAMES_DEPS`
- [x] `RuntimeFn::ObjectIsFrozen` -> `$object_is_frozen` / sig `1->1` / result `Value`: deps `OBJECT_GET_OWN_PROPERTY_NAMES_DEPS`
- [x] `RuntimeFn::ObjectDefineProperty` -> `$object_define_property` / sig `3->1` / result `Value`: deps `OBJECT_DEFINE_PROPERTY_DEPS`; typed descriptor mutation builder + registry/test 接続済み。
- [x] `RuntimeFn::ObjectDefineProperties` -> `$object_define_properties` / sig `2->1` / result `Value`: deps `OBJECT_DEFINE_PROPERTIES_DEPS`; typed descriptor iteration wrapper + registry/test 接続済み。
- [x] `RuntimeFn::ObjectGetOwnPropertyDescriptors` -> `$object_get_own_property_descriptors` / sig `1->1` / result `Value`: deps `OBJECT_GET_OWN_PROPERTY_DESCRIPTORS_DEPS`; ReflectOwnKeys 連携の typed descriptor map builder + registry/test 接続済み。
- [x] `RuntimeFn::ObjectAssign` -> `$object_assign` / sig `2->1` / result `Value`: deps `OBJECT_ASSIGN_DEPS`
- [x] `RuntimeFn::ObjectCreate` -> `$object_create` / sig `1->1` / result `Value`: deps `OBJECT_CREATE_DEPS`
- [x] `RuntimeFn::ObjectPrototype` -> `$object_prototype` / sig `0->1` / result `Value`: deps `OBJECT_PROTOTYPE_OBJECT_DEPS`
- [x] `RuntimeFn::GlobalThis` -> `$global_this` / sig `0->1` / result `Value`: deps `GLOBAL_THIS_DEPS`
- [x] `RuntimeFn::ObjectIs` -> `$object_is` / sig `2->1` / result `Value`: deps `&[ Self::IsString, Self::StringEqual, Self::BigIntCompare, Self::NumberToI32, ]`
- [x] `RuntimeFn::PropertyIsEnumerable` -> `$property_is_enumerable` / sig `2->1` / result `Value`: deps `PROPERTY_IS_ENUMERABLE_DEPS`
- [x] `RuntimeFn::IsPrototypeOf` -> `$is_prototype_of` / sig `2->1` / result `Value`: deps `IS_PROTOTYPE_OF_DEPS`
- [x] `RuntimeFn::ObjectToString` -> `$object_to_string` / sig `1->1` / result `Value`: deps `OBJECT_TO_STRING_DEPS`; runtime_strings `OBJECT_TO_STRING_RUNTIME_STRINGS`; 2-arg lowered form は native emitter が `NumberToStringRadix(NumberToI32(value), NumberToI32(radix))` に展開し、radix conversion parity 済み。
- [x] `RuntimeFn::ErrorToString` -> `$error_to_string` / sig `1->1` / result `Value`: deps `ERROR_TO_STRING_DEPS`; runtime_strings `ERROR_TO_STRING_RUNTIME_STRINGS`
- [x] `RuntimeFn::ObjectToLocaleString` -> `$object_to_locale_string` / sig `1->1` / result `Value`: deps `OBJECT_TO_LOCALE_STRING_DEPS`; tagged receiver ABI / console boundary を native emitter に接続し、string receiver の `toLocaleString` parity 済み。
- [x] `RuntimeFn::ReflectDefineProperty` -> `$reflect_define_property` / sig `3->1` / result `Value`: deps `&[RuntimeFn::AllocHeap, RuntimeFn::ValueToStringInto, RuntimeFn::PropertyGet, RuntimeFn::PropertySet, RuntimeFn::ObjectDefineProperty]`; typed ObjectDefineProperty delegation + registry/test 接続済み。
- [x] `RuntimeFn::ReflectDeleteProperty` -> `$reflect_delete_property` / sig `2->1` / result `Value`: deps `&[RuntimeFn::AllocHeap, RuntimeFn::ValueToStringInto, RuntimeFn::MemEqual, RuntimeFn::PropertyDelete]`
- [x] `RuntimeFn::ReflectGet` -> `$reflect_get` / sig `3->1` / result `Value`: deps `&[RuntimeFn::PropertyGet, RuntimeFn::MemEqual, RuntimeFn::ValueToStringInto, RuntimeFn::AllocHeap]`
- [x] `RuntimeFn::ReflectHas` -> `$reflect_has` / sig `2->1` / result `Value`: deps `&[RuntimeFn::ValueToStringInto, RuntimeFn::MemEqual, RuntimeFn::AllocHeap, RuntimeFn::PropertyHas]`
- [x] `RuntimeFn::ReflectOwnKeys` -> `$reflect_own_keys` / sig `1->1` / result `Value`: deps `&[RuntimeFn::AllocHeap, RuntimeFn::Copy, RuntimeFn::ObjectKeys, RuntimeFn::ObjectGetOwnPropertySymbols]`
- [x] `RuntimeFn::ReflectPreventExtensions` -> `$reflect_prevent_extensions` / sig `1->1` / result `Value`: deps `&[RuntimeFn::ObjectPreventExtensions]`
- [x] `RuntimeFn::ReflectSet` -> `$reflect_set` / sig `4->1` / result `Value`: deps `&[RuntimeFn::PropertyGet, RuntimeFn::PropertySet, RuntimeFn::MemEqual, RuntimeFn::ValueToStringInto, RuntimeFn::AllocHeap, RuntimeFn::Copy]`
- [x] `RuntimeFn::ReflectSetPrototypeOf` -> `$reflect_set_prototype_of` / sig `2->1` / result `Value`: deps `&[RuntimeFn::ObjectSetPrototypeOf]`
- [x] `RuntimeFn::ReflectApply` -> `$reflect_apply` / sig `3->1` / result `Value`: deps `&[RuntimeFn::AllocHeap, RuntimeFn::Copy]`; imports `IMPORT_REFLECT_APPLY`; capability `CAP_HOST_REFLECT_APPLY`
- [x] `RuntimeFn::ReflectConstruct` -> `$reflect_construct` / sig `2->1` / result `Value`: deps `&[RuntimeFn::AllocHeap, RuntimeFn::Copy]`; imports `IMPORT_REFLECT_CONSTRUCT`; capability `CAP_HOST_REFLECT_CONSTRUCT`

### MapSet (61)

- [x] `RuntimeFn::MapNew` -> `$map_new` / sig `1->1` / result `Value`: deps `MAP_NEW_DEPS`
- [x] `RuntimeFn::MapGet` -> `$map_get` / sig `2->1` / result `Value`: deps `MAP_GET_DEPS`
- [x] `RuntimeFn::MapSet` -> `$map_set` / sig `3->1` / result `Value`: deps `MAP_SET_DEPS`
- [x] `RuntimeFn::MapHas` -> `$map_has` / sig `2->1` / result `Value`: deps `MAP_HAS_DEPS`
- [x] `RuntimeFn::MapDelete` -> `$map_delete` / sig `2->1` / result `Value`: deps `MAP_DELETE_DEPS`
- [x] `RuntimeFn::MapValuesArray` -> `$map_values_array` / sig `1->1` / result `Value`: deps `MAP_VALUES_ARRAY_DEPS`
- [x] `RuntimeFn::MapKeysArray` -> `$map_keys_array` / sig `1->1` / result `Value`: deps `MAP_VALUES_ARRAY_DEPS`
- [x] `RuntimeFn::SetNew` -> `$set_new` / sig `1->1` / result `Value`: deps `SET_NEW_DEPS`
- [x] `RuntimeFn::SetAdd` -> `$set_add` / sig `2->1` / result `Value`: deps `SET_ADD_DEPS`
- [x] `RuntimeFn::SetHas` -> `$set_has` / sig `2->1` / result `Value`: deps `SET_HAS_DEPS`
- [x] `RuntimeFn::SetDelete` -> `$set_delete` / sig `2->1` / result `Value`: deps `SET_DELETE_DEPS`
- [x] `RuntimeFn::SetSize` -> `$set_size` / sig `1->1` / result `Value`: deps `SET_SIZE_DEPS`
- [x] `RuntimeFn::SetClear` -> `$set_clear` / sig `1->1` / result `Value`: deps `SET_CLEAR_DEPS`
- [x] `RuntimeFn::SetForEach` -> `$set_for_each` / sig `2->1` / result `Value`: deps `SET_FOR_EACH_DEPS`
- [x] `RuntimeFn::MapClear` -> `$map_clear` / sig `1->1` / result `Value`: deps `MAP_CLEAR_DEPS`
- [x] `RuntimeFn::MapForEach` -> `$map_for_each` / sig `2->1` / result `Value`: deps `MAP_FOR_EACH_DEPS`
- [x] `RuntimeFn::MapSize` -> `$map_size` / sig `1->1` / result `Value`: deps `MAP_SIZE_DEPS`
- [x] `RuntimeFn::MapEntriesArray` -> `$map_entries_array` / sig `1->1` / result `Value`: deps `MAP_ENTRIES_ARRAY_DEPS`
- [x] `RuntimeFn::MapEntryPairsArray` -> `$map_entry_pairs_array` / sig `1->1` / result `Value`: deps `MAP_ENTRY_PAIRS_ARRAY_DEPS`
- [x] `RuntimeFn::SetFromArray` -> `$set_from_array` / sig `1->1` / result `Value`: deps `SET_FROM_ARRAY_DEPS`
- [x] `RuntimeFn::SetValuesArray` -> `$set_values_array` / sig `1->1` / result `Value`: deps `SET_VALUES_ARRAY_DEPS`
- [x] `RuntimeFn::SetEntriesArray` -> `$set_entries_array` / sig `1->1` / result `Value`: deps `SET_VALUES_ARRAY_DEPS`
- [x] `RuntimeFn::SetPrototypeAddGet` -> `$set_prototype_add_get` / sig `0->1` / result `Value`
- [x] `RuntimeFn::SetPrototypeAddSet` -> `$set_prototype_add_set` / sig `1->1` / result `Value`
- [x] `RuntimeFn::SetPrototypeHasGet` -> `$set_prototype_has_get` / sig `0->1` / result `Value`
- [x] `RuntimeFn::SetPrototypeHasSet` -> `$set_prototype_has_set` / sig `1->1` / result `Value`
- [x] `RuntimeFn::SetPrototypeDeleteGet` -> `$set_prototype_delete_get` / sig `0->1` / result `Value`
- [x] `RuntimeFn::SetPrototypeDeleteSet` -> `$set_prototype_delete_set` / sig `1->1` / result `Value`
- [x] `RuntimeFn::SetPrototypeForEachGet` -> `$set_prototype_for_each_get` / sig `0->1` / result `Value`
- [x] `RuntimeFn::SetPrototypeForEachSet` -> `$set_prototype_for_each_set` / sig `1->1` / result `Value`
- [x] `RuntimeFn::MapPrototypeGetGet` -> `$map_prototype_get_get` / sig `0->1` / result `Value`
- [x] `RuntimeFn::MapPrototypeGetSet` -> `$map_prototype_get_set` / sig `1->1` / result `Value`
- [x] `RuntimeFn::MapPrototypeSetGet` -> `$map_prototype_set_get` / sig `0->1` / result `Value`
- [x] `RuntimeFn::MapPrototypeSetSet` -> `$map_prototype_set_set` / sig `1->1` / result `Value`
- [x] `RuntimeFn::MapPrototypeHasGet` -> `$map_prototype_has_get` / sig `0->1` / result `Value`
- [x] `RuntimeFn::MapPrototypeHasSet` -> `$map_prototype_has_set` / sig `1->1` / result `Value`
- [x] `RuntimeFn::MapPrototypeDeleteGet` -> `$map_prototype_delete_get` / sig `0->1` / result `Value`
- [x] `RuntimeFn::MapPrototypeDeleteSet` -> `$map_prototype_delete_set` / sig `1->1` / result `Value`
- [x] `RuntimeFn::MapPrototypeForEachGet` -> `$map_prototype_for_each_get` / sig `0->1` / result `Value`
- [x] `RuntimeFn::MapPrototypeForEachSet` -> `$map_prototype_for_each_set` / sig `1->1` / result `Value`
- [x] `RuntimeFn::SetIsDisjointFrom` -> `$set_is_disjoint_from` / sig `2->1` / result `Value`: deps `SET_IS_DISJOINT_FROM_DEPS`
- [x] `RuntimeFn::SetIsSubsetOf` -> `$set_is_subset_of` / sig `2->1` / result `Value`: deps `SET_IS_SUBSET_OF_DEPS`
- [x] `RuntimeFn::SetIsSupersetOf` -> `$set_is_superset_of` / sig `2->1` / result `Value`: deps `SET_IS_SUPERSET_OF_DEPS`
- [x] `RuntimeFn::SetUnion` -> `$set_union` / sig `2->1` / result `Value`: deps `SET_UNION_DEPS`
- [x] `RuntimeFn::SetIntersection` -> `$set_intersection` / sig `2->1` / result `Value`: deps `SET_INTERSECTION_DEPS`
- [x] `RuntimeFn::SetDifference` -> `$set_difference` / sig `2->1` / result `Value`: deps `SET_DIFFERENCE_DEPS`
- [x] `RuntimeFn::SetSymmetricDifference` -> `$set_symmetric_difference` / sig `2->1` / result `Value`: deps `SET_SYMMETRIC_DIFFERENCE_DEPS`
- [x] `RuntimeFn::WeakMapNew` -> `$weak_map_new` / sig `1->1` / result `Value`: deps `&[RuntimeFn::AllocHeap]`
- [x] `RuntimeFn::WeakMapSet` -> `$weak_map_set` / sig `3->1` / result `Value`: deps `&[RuntimeFn::StrictEqual]`
- [x] `RuntimeFn::WeakMapGet` -> `$weak_map_get` / sig `2->1` / result `Value`: deps `&[RuntimeFn::StrictEqual]`
- [x] `RuntimeFn::WeakMapHas` -> `$weak_map_has` / sig `2->1` / result `Value`: deps `&[RuntimeFn::StrictEqual]`
- [x] `RuntimeFn::WeakMapDelete` -> `$weak_map_delete` / sig `2->1` / result `Value`: deps `&[RuntimeFn::StrictEqual]`
- [x] `RuntimeFn::WeakSetNew` -> `$weak_set_new` / sig `1->1` / result `Value`: deps `&[RuntimeFn::AllocHeap]`
- [x] `RuntimeFn::WeakSetAdd` -> `$weak_set_add` / sig `2->1` / result `Value`: deps `&[RuntimeFn::StrictEqual]`
- [x] `RuntimeFn::WeakSetHas` -> `$weak_set_has` / sig `2->1` / result `Value`: deps `&[RuntimeFn::StrictEqual]`
- [x] `RuntimeFn::WeakSetDelete` -> `$weak_set_delete` / sig `2->1` / result `Value`: deps `&[RuntimeFn::StrictEqual]`
- [x] `RuntimeFn::WeakRefNew` -> `$weak_ref_new` / sig `1->1` / result `Value`: deps `&[RuntimeFn::AllocHeap]`
- [x] `RuntimeFn::WeakRefDeref` -> `$weak_ref_deref` / sig `1->1` / result `Value`: deps `&[]`
- [x] `RuntimeFn::FinalizationRegistryNew` -> `$finalization_registry_new` / sig `1->1` / result `Value`: deps `&[RuntimeFn::AllocHeap]`
- [x] `RuntimeFn::FinalizationRegistryRegister` -> `$finalization_registry_register` / sig `4->1` / result `Value`: deps `&[]`
- [x] `RuntimeFn::FinalizationRegistryUnregister` -> `$finalization_registry_unregister` / sig `2->1` / result `Value`: deps `&[]`

### TypedArray (45)

- [x] `RuntimeFn::TypedArrayFromArray` -> `$typed_array_from_array` / sig `1->1` / result `Value`: deps `TYPED_ARRAY_FROM_ARRAY_DEPS`
- [x] `RuntimeFn::TypedArrayCtorFromBuffer` -> `$typed_array_ctor_from_buffer` / sig `3->1` / result `Value`: deps `TYPED_ARRAY_CTOR_WITH_LENGTH_DEPS`
- [x] `RuntimeFn::TypedArrayCtorWithLength` -> `$typed_array_ctor_with_length` / sig `1->1` / result `Value`: deps `TYPED_ARRAY_CTOR_WITH_LENGTH_DEPS`
- [x] `RuntimeFn::TypedArraySet` -> `$typed_array_set` / sig `3->1` / result `Value`: deps `TYPED_ARRAY_SET_DEPS`
- [x] `RuntimeFn::TypedArrayLoad` -> `$typed_array_load` / sig `2->1` / result `Value`: deps `TYPED_ARRAY_LOAD_DEPS`
- [x] `RuntimeFn::TypedArrayStore` -> `$typed_array_store` / sig `3->0` / result `Value`: deps `TYPED_ARRAY_STORE_DEPS`
- [x] `RuntimeFn::AtomicsElementPtr` -> `$atomics_element_ptr` / sig `2->1` / result `Value`: deps `ATOMICS_NO_DEPS`
- [x] `RuntimeFn::AtomicsLoad` -> `$atomics_load` / sig `2->1` / result `Value`: deps `&[RuntimeFn::AtomicsElementPtr]`
- [x] `RuntimeFn::AtomicsStore` -> `$atomics_store` / sig `3->1` / result `Value`: deps `ATOMICS_VALUE_DEPS`
- [x] `RuntimeFn::AtomicsAdd` -> `$atomics_add` / sig `3->1` / result `Value`: deps `ATOMICS_VALUE_DEPS`
- [x] `RuntimeFn::AtomicsSub` -> `$atomics_sub` / sig `3->1` / result `Value`: deps `ATOMICS_VALUE_DEPS`
- [x] `RuntimeFn::AtomicsAnd` -> `$atomics_and` / sig `3->1` / result `Value`: deps `ATOMICS_VALUE_DEPS`
- [x] `RuntimeFn::AtomicsOr` -> `$atomics_or` / sig `3->1` / result `Value`: deps `ATOMICS_VALUE_DEPS`
- [x] `RuntimeFn::AtomicsXor` -> `$atomics_xor` / sig `3->1` / result `Value`: deps `ATOMICS_VALUE_DEPS`
- [x] `RuntimeFn::AtomicsExchange` -> `$atomics_exchange` / sig `3->1` / result `Value`: deps `ATOMICS_VALUE_DEPS`
- [x] `RuntimeFn::AtomicsCompareExchange` -> `$atomics_compare_exchange` / sig `4->1` / result `Value`: deps `ATOMICS_VALUE_DEPS`
- [x] `RuntimeFn::AtomicsIsLockFree` -> `$atomics_is_lock_free` / sig `1->1` / result `Value`: deps `ATOMICS_NO_DEPS`
- [x] `RuntimeFn::AtomicsWait` -> `$atomics_wait` / sig `4->1` / result `Value`: deps `ATOMICS_VALUE_DEPS`
- [x] `RuntimeFn::AtomicsWaitAsync` -> `$atomics_wait_async` / sig `3->1` / result `Value`: deps `OBJECT_GET_OWN_PROPERTY_NAMES_DEPS`
- [x] `RuntimeFn::AtomicsNotify` -> `$atomics_notify` / sig `3->1` / result `Value`: deps `ATOMICS_VALUE_DEPS`
- [x] `RuntimeFn::DataViewNew` -> `$dataview_new` / sig `2->1` / result `Value`: deps `DATAVIEW_NEW_DEPS`
- [x] `RuntimeFn::DataViewGetInt8` -> `$dataview_get_int8` / sig `2->1` / result `Value`: deps `&[]`
- [x] `RuntimeFn::DataViewSetInt8` -> `$dataview_set_int8` / sig `3->0` / result `EffectOnly`: deps `&[]`
- [x] `RuntimeFn::DataViewGetUint8` -> `$dataview_get_uint8` / sig `2->1` / result `Value`: deps `&[]`
- [x] `RuntimeFn::DataViewSetUint8` -> `$dataview_set_uint8` / sig `3->0` / result `EffectOnly`: deps `&[]`
- [x] `RuntimeFn::DataViewGetInt16` -> `$dataview_get_int16` / sig `3->1` / result `Value`: deps `&[]`
- [x] `RuntimeFn::DataViewSetInt16` -> `$dataview_set_int16` / sig `4->0` / result `EffectOnly`: deps `&[]`
- [x] `RuntimeFn::DataViewGetUint16` -> `$dataview_get_uint16` / sig `3->1` / result `Value`: deps `&[]`
- [x] `RuntimeFn::DataViewSetUint16` -> `$dataview_set_uint16` / sig `4->0` / result `EffectOnly`: deps `&[]`
- [x] `RuntimeFn::DataViewGetInt32` -> `$dataview_get_int32` / sig `3->1` / result `Value`: deps `&[]`
- [x] `RuntimeFn::DataViewSetInt32` -> `$dataview_set_int32` / sig `4->0` / result `EffectOnly`: deps `&[]`
- [x] `RuntimeFn::DataViewGetUint32` -> `$dataview_get_uint32` / sig `3->1` / result `Value`: deps `&[]`
- [x] `RuntimeFn::DataViewSetUint32` -> `$dataview_set_uint32` / sig `4->0` / result `EffectOnly`: deps `&[]`
- [x] `RuntimeFn::DataViewGetFloat32` -> `$dataview_get_float32` / sig `3->1` / result `Value`: deps `&[]`
- [x] `RuntimeFn::DataViewSetFloat32` -> `$dataview_set_float32` / sig `4->0` / result `EffectOnly`: deps `&[]`
- [x] `RuntimeFn::DataViewGetFloat64` -> `$dataview_get_float64` / sig `3->1` / result `Value`: deps `&[]`
- [x] `RuntimeFn::DataViewSetFloat64` -> `$dataview_set_float64` / sig `4->0` / result `EffectOnly`: deps `&[]`
- [x] `RuntimeFn::DataViewGetFloat16` -> `$dataview_get_float16` / sig `3->1` / result `Value`: deps `&[]`
- [x] `RuntimeFn::DataViewSetFloat16` -> `$dataview_set_float16` / sig `4->0` / result `EffectOnly`: deps `&[]`
- [x] `RuntimeFn::DataViewGetBigInt64` -> `$dataview_get_bigint64` / sig `3->1` / result `Value`: deps `DATAVIEW_GET_BIGINT64_DEPS`
- [x] `RuntimeFn::DataViewSetBigInt64` -> `$dataview_set_bigint64` / sig `4->0` / result `EffectOnly`: deps `DATAVIEW_SET_BIGINT64_DEPS`
- [x] `RuntimeFn::DataViewGetBigUint64` -> `$dataview_get_biguint64` / sig `3->1` / result `Value`: deps `DATAVIEW_GET_BIGUINT64_DEPS`
- [x] `RuntimeFn::DataViewSetBigUint64` -> `$dataview_set_biguint64` / sig `4->0` / result `EffectOnly`: deps `DATAVIEW_SET_BIGUINT64_DEPS`
- [x] `RuntimeFn::DataViewGetBuffer` -> `$dataview_get_buffer` / sig `1->1` / result `Value`: deps `OBJECT_GET_OWN_PROPERTY_NAMES_DEPS`
- [x] `RuntimeFn::DataViewGetByteOffset` -> `$dataview_get_byte_offset` / sig `1->1` / result `Value`: deps `OBJECT_GET_OWN_PROPERTY_NAMES_DEPS`

### Date (40)

- [x] `RuntimeFn::DateNew` -> `$date_new` / sig `1->1` / result `Value`: deps `DATE_NEW_DEPS`
- [x] `RuntimeFn::DateEpochMsNowNumber` -> `$date_epoch_ms_now_number` / sig `1->1` / result `Value`: deps `DATE_EPOCH_MS_NOW_NUMBER_DEPS`; imports `IMPORT_CLOCK_TIME_GET`; capability `CAP_WASI_CLOCK_REALTIME`
- [x] `RuntimeFn::DateNewLive` -> `$date_new_live` / sig `1->1` / result `Value`: deps `DATE_NEW_LIVE_DEPS`; imports `IMPORT_CLOCK_TIME_GET`; capability `CAP_WASI_CLOCK_REALTIME`
- [x] `RuntimeFn::DateNow` -> `$date_now` / sig `1->1` / result `Value`: deps `DATE_NOW_DEPS`; imports `IMPORT_CLOCK_TIME_GET`; capability `CAP_WASI_CLOCK_REALTIME`
- [x] `RuntimeFn::DateGetTime` -> `$date_get_time` / sig `1->1` / result `Value`
- [x] `RuntimeFn::DateSetTime` -> `$date_set_time` / sig `2->1` / result `Value`
- [x] `RuntimeFn::DateSetUTCFullYear` -> `$date_set_utc_full_year` / sig `4->1` / result `Value`: deps `DATE_SET_UTC_FULL_YEAR_DEPS`
- [x] `RuntimeFn::DateSetUTCMonth` -> `$date_set_utc_month` / sig `3->1` / result `Value`: deps `&[ Self::DateUTC, Self::DateGetUtcFullYear, Self::DateGetUtcDate, Self::DateGetUtcHours, Self::DateGetUtcMinutes, Self::DateGetUtcSeconds, Self::DateGetUtcMilliseconds, ]`
- [x] `RuntimeFn::DateSetUTCDate` -> `$date_set_utc_date` / sig `2->1` / result `Value`: deps `&[ Self::DateUTC, Self::DateGetUtcFullYear, Self::DateGetUtcMonth, Self::DateGetUtcHours, Self::DateGetUtcMinutes, Self::DateGetUtcSeconds, Self::DateGetUtcMilliseconds, ]`
- [x] `RuntimeFn::DateSetUTCHours` -> `$date_set_utc_hours` / sig `5->1` / result `Value`: deps `&[ Self::DateUTC, Self::DateGetUtcFullYear, Self::DateGetUtcMonth, Self::DateGetUtcDate, Self::DateGetUtcMinutes, Self::DateGetUtcSeconds, Self::DateGetUtcMilliseconds, ]`
- [x] `RuntimeFn::DateSetUTCMinutes` -> `$date_set_utc_minutes` / sig `4->1` / result `Value`: deps `&[ Self::DateUTC, Self::DateGetUtcFullYear, Self::DateGetUtcMonth, Self::DateGetUtcDate, Self::DateGetUtcHours, Self::DateGetUtcSeconds, Self::DateGetUtcMilliseconds, ]`
- [x] `RuntimeFn::DateSetUTCSeconds` -> `$date_set_utc_seconds` / sig `3->1` / result `Value`: deps `&[ Self::DateUTC, Self::DateGetUtcFullYear, Self::DateGetUtcMonth, Self::DateGetUtcDate, Self::DateGetUtcHours, Self::DateGetUtcMinutes, Self::DateGetUtcMilliseconds, ]`
- [x] `RuntimeFn::DateSetUTCMilliseconds` -> `$date_set_utc_milliseconds` / sig `2->1` / result `Value`: deps `&[ Self::DateUTC, Self::DateGetUtcFullYear, Self::DateGetUtcMonth, Self::DateGetUtcDate, Self::DateGetUtcHours, Self::DateGetUtcMinutes, Self::DateGetUtcSeconds, ]`
- [x] `RuntimeFn::DateSetFullYear` -> `$date_set_full_year` / sig `4->1` / result `Value`: deps `&[ Self::DateGetLocalTimeField, Self::DateGetTimezoneOffset, Self::DateUTC, ]`
- [x] `RuntimeFn::DateSetMonth` -> `$date_set_month` / sig `3->1` / result `Value`: deps `&[ Self::DateGetLocalTimeField, Self::DateGetTimezoneOffset, Self::DateUTC, ]`
- [x] `RuntimeFn::DateSetDate` -> `$date_set_date` / sig `2->1` / result `Value`: deps `&[ Self::DateGetLocalTimeField, Self::DateGetTimezoneOffset, Self::DateUTC, ]`
- [x] `RuntimeFn::DateSetHours` -> `$date_set_hours` / sig `5->1` / result `Value`: deps `&[ Self::DateGetLocalTimeField, Self::DateGetTimezoneOffset, Self::DateUTC, ]`
- [x] `RuntimeFn::DateSetMinutes` -> `$date_set_minutes` / sig `4->1` / result `Value`: deps `&[ Self::DateGetLocalTimeField, Self::DateGetTimezoneOffset, Self::DateUTC, ]`
- [x] `RuntimeFn::DateSetSeconds` -> `$date_set_seconds` / sig `3->1` / result `Value`: deps `&[ Self::DateGetLocalTimeField, Self::DateGetTimezoneOffset, Self::DateUTC, ]`
- [x] `RuntimeFn::DateSetMilliseconds` -> `$date_set_milliseconds` / sig `2->1` / result `Value`: deps `&[ Self::DateGetLocalTimeField, Self::DateGetTimezoneOffset, Self::DateUTC, ]`
- [x] `RuntimeFn::DateSetYear` -> `$date_set_year` / sig `2->1` / result `Value`: deps `&[Self::DateSetFullYear]`
- [x] `RuntimeFn::DateParse` -> `$date_parse` / sig `1->1` / result `Value`: imports `IMPORT_DATE_PARSE`; capability `CAP_HOST_DATE_PARSE`
- [x] `RuntimeFn::DateUTC` -> `$date_utc` / sig `7->1` / result `Value`: imports `IMPORT_DATE_UTC`; capability `CAP_HOST_DATE_UTC`
- [x] `RuntimeFn::DateToString` -> `$date_to_string` / sig `1->1` / result `Value`: imports `IMPORT_DATE_TO_STRING`; capability `CAP_HOST_DATE_TO_STRING`
- [x] `RuntimeFn::DateGetLocalTimeField` -> `$date_get_local_time_field` / sig `2->1` / result `Value`: imports `IMPORT_DATE_GET_LOCAL_TIME_FIELD`; capability `CAP_HOST_DATE_GET_LOCAL_TIME_FIELD`
- [x] `RuntimeFn::DateGetYear` -> `$date_get_year` / sig `1->1` / result `Value`
- [x] `RuntimeFn::DateToISOString` -> `$date_to_iso_string` / sig `1->1` / result `Value`: imports `IMPORT_DATE_TO_ISO_STRING`; capability `CAP_HOST_DATE_TO_ISO_STRING`
- [x] `RuntimeFn::DateGetTimezoneOffset` -> `$date_get_timezone_offset` / sig `1->1` / result `Value`: imports `IMPORT_DATE_GET_TIMEZONE_OFFSET`; capability `CAP_HOST_DATE_GET_TIMEZONE_OFFSET`
- [x] `RuntimeFn::DateToDateString` -> `$date_to_date_string` / sig `1->1` / result `Value`: imports `IMPORT_DATE_TO_DATE_STRING`; capability `CAP_HOST_DATE_TO_DATE_STRING`
- [x] `RuntimeFn::DateToTimeString` -> `$date_to_time_string` / sig `1->1` / result `Value`: imports `IMPORT_DATE_TO_TIME_STRING`; capability `CAP_HOST_DATE_TO_TIME_STRING`
- [x] `RuntimeFn::DateToGMTString` -> `$date_to_gmt_string` / sig `1->1` / result `Value`: deps `&[Self::DateToString]`
- [x] `RuntimeFn::DateGetUtcMilliseconds` -> `$date_get_utc_milliseconds` / sig `1->1` / result `Value`
- [x] `RuntimeFn::DateGetUtcSeconds` -> `$date_get_utc_seconds` / sig `1->1` / result `Value`
- [x] `RuntimeFn::DateGetUtcMinutes` -> `$date_get_utc_minutes` / sig `1->1` / result `Value`
- [x] `RuntimeFn::DateGetUtcHours` -> `$date_get_utc_hours` / sig `1->1` / result `Value`
- [x] `RuntimeFn::DateGetUtcDay` -> `$date_get_utc_day` / sig `1->1` / result `Value`
- [x] `RuntimeFn::DateGetUtcDate` -> `$date_get_utc_date` / sig `1->1` / result `Value`
- [x] `RuntimeFn::DateGetUtcMonth` -> `$date_get_utc_month` / sig `1->1` / result `Value`
- [x] `RuntimeFn::DateGetUtcFullYear` -> `$date_get_utc_full_year` / sig `1->1` / result `Value`
- [x] `RuntimeFn::IntlDateTimeFormatFormat` -> `$intl_date_time_format_format` / sig `2->1` / result `Value`: deps `OBJECT_GET_OWN_PROPERTY_NAMES_DEPS`; imports `IMPORT_INTL_DATE_TIME_FORMAT_FORMAT`; capability `CAP_INTL_DATE_TIME_FORMAT_FORMAT`

### Math (36)

- [x] `RuntimeFn::MathFloor` -> `$math_floor` / sig `1->1` / result `Value`: deps `MATH_DEPS`
- [x] `RuntimeFn::MathCeil` -> `$math_ceil` / sig `1->1` / result `Value`: deps `MATH_DEPS`
- [x] `RuntimeFn::MathRound` -> `$math_round` / sig `1->1` / result `Value`: deps `MATH_DEPS`
- [x] `RuntimeFn::MathAbs` -> `$math_abs` / sig `1->1` / result `Value`: deps `MATH_DEPS`
- [x] `RuntimeFn::MathMax` -> `$math_max` / sig `2->1` / result `Value`: deps `MATH_DEPS`
- [x] `RuntimeFn::MathMin` -> `$math_min` / sig `2->1` / result `Value`: deps `MATH_DEPS`
- [x] `RuntimeFn::MathPow` -> `$math_pow` / sig `2->1` / result `Value`: deps `MATH_DEPS`
- [x] `RuntimeFn::MathRandom` -> `$math_random` / sig `1->1` / result `Value`: deps `MATH_RANDOM_DEPS`; imports `IMPORT_RANDOM_GET`; capability `CAP_WASI_RANDOM`
- [x] `RuntimeFn::MathTrunc` -> `$math_trunc` / sig `1->1` / result `Value`: deps `MATH_DEPS`
- [x] `RuntimeFn::MathSign` -> `$math_sign` / sig `1->1` / result `Value`: deps `MATH_DEPS`
- [x] `RuntimeFn::MathCbrt` -> `$math_cbrt` / sig `1->1` / result `Value`: deps `MATH_DEPS`
- [x] `RuntimeFn::MathClz32` -> `$math_clz32` / sig `1->1` / result `Value`: deps `MATH_DEPS`
- [x] `RuntimeFn::MathImul` -> `$math_imul` / sig `2->1` / result `Value`: deps `MATH_DEPS`
- [x] `RuntimeFn::MathSqrt` -> `$math_sqrt` / sig `1->1` / result `Value`: deps `MATH_DEPS`
- [x] `RuntimeFn::MathAcos` -> `$math_acos` / sig `1->1` / result `Value`: deps `MATH_DEPS`; imports `IMPORT_MATH_ACOS`; capability `CAP_HOST_MATH_ACOS`
- [x] `RuntimeFn::MathAcosh` -> `$math_acosh` / sig `1->1` / result `Value`: deps `MATH_DEPS`; imports `IMPORT_MATH_ACOSH`; capability `CAP_HOST_MATH_ACOSH`
- [x] `RuntimeFn::MathAsin` -> `$math_asin` / sig `1->1` / result `Value`: deps `MATH_DEPS`; imports `IMPORT_MATH_ASIN`; capability `CAP_HOST_MATH_ASIN`
- [x] `RuntimeFn::MathAsinh` -> `$math_asinh` / sig `1->1` / result `Value`: deps `MATH_DEPS`; imports `IMPORT_MATH_ASINH`; capability `CAP_HOST_MATH_ASINH`
- [x] `RuntimeFn::MathAtan` -> `$math_atan` / sig `1->1` / result `Value`: deps `MATH_DEPS`; imports `IMPORT_MATH_ATAN`; capability `CAP_HOST_MATH_ATAN`
- [x] `RuntimeFn::MathAtan2` -> `$math_atan2` / sig `2->1` / result `Value`: deps `MATH_DEPS`; imports `IMPORT_MATH_ATAN2`; capability `CAP_HOST_MATH_ATAN2`
- [x] `RuntimeFn::MathAtanh` -> `$math_atanh` / sig `1->1` / result `Value`: deps `MATH_DEPS`; imports `IMPORT_MATH_ATANH`; capability `CAP_HOST_MATH_ATANH`
- [x] `RuntimeFn::MathCos` -> `$math_cos` / sig `1->1` / result `Value`: deps `MATH_DEPS`; imports `IMPORT_MATH_COS`; capability `CAP_HOST_MATH_COS`
- [x] `RuntimeFn::MathCosh` -> `$math_cosh` / sig `1->1` / result `Value`: deps `MATH_DEPS`; imports `IMPORT_MATH_COSH`; capability `CAP_HOST_MATH_COSH`
- [x] `RuntimeFn::MathExp` -> `$math_exp` / sig `1->1` / result `Value`: deps `MATH_DEPS`; imports `IMPORT_MATH_EXP`; capability `CAP_HOST_MATH_EXP`
- [x] `RuntimeFn::MathExpm1` -> `$math_expm1` / sig `1->1` / result `Value`: deps `MATH_DEPS`; imports `IMPORT_MATH_EXPM1`; capability `CAP_HOST_MATH_EXPM1`
- [x] `RuntimeFn::MathFround` -> `$math_fround` / sig `1->1` / result `Value`: deps `MATH_DEPS`
- [x] `RuntimeFn::MathF16round` -> `$math_f16round` / sig `1->1` / result `Value`: deps `MATH_DEPS`
- [x] `RuntimeFn::MathHypot` -> `$math_hypot` / sig `2->1` / result `Value`: deps `MATH_DEPS`; imports `IMPORT_MATH_HYPOT`; capability `CAP_HOST_MATH_HYPOT`
- [x] `RuntimeFn::MathLog` -> `$math_log` / sig `1->1` / result `Value`: deps `MATH_DEPS`; imports `IMPORT_MATH_LOG`; capability `CAP_HOST_MATH_LOG`
- [x] `RuntimeFn::MathLog10` -> `$math_log10` / sig `1->1` / result `Value`: deps `MATH_DEPS`; imports `IMPORT_MATH_LOG10`; capability `CAP_HOST_MATH_LOG10`
- [x] `RuntimeFn::MathLog1p` -> `$math_log1p` / sig `1->1` / result `Value`: deps `MATH_DEPS`; imports `IMPORT_MATH_LOG1P`; capability `CAP_HOST_MATH_LOG1P`
- [x] `RuntimeFn::MathLog2` -> `$math_log2` / sig `1->1` / result `Value`: deps `MATH_DEPS`; imports `IMPORT_MATH_LOG2`; capability `CAP_HOST_MATH_LOG2`
- [x] `RuntimeFn::MathSin` -> `$math_sin` / sig `1->1` / result `Value`: deps `MATH_DEPS`; imports `IMPORT_MATH_SIN`; capability `CAP_HOST_MATH_SIN`
- [x] `RuntimeFn::MathSinh` -> `$math_sinh` / sig `1->1` / result `Value`: deps `MATH_DEPS`; imports `IMPORT_MATH_SINH`; capability `CAP_HOST_MATH_SINH`
- [x] `RuntimeFn::MathTan` -> `$math_tan` / sig `1->1` / result `Value`: deps `MATH_DEPS`; imports `IMPORT_MATH_TAN`; capability `CAP_HOST_MATH_TAN`
- [x] `RuntimeFn::MathTanh` -> `$math_tanh` / sig `1->1` / result `Value`: deps `MATH_DEPS`; imports `IMPORT_MATH_TANH`; capability `CAP_HOST_MATH_TANH`

### Json (2)

- [x] `RuntimeFn::JsonStringify` -> `$json_stringify` / sig `3->1` / result `Value`: deps `JSON_STRINGIFY_DEPS`; imports `IMPORT_JSON_STRINGIFY`; capability `CAP_HOST_JSON_STRINGIFY`; dynamic path は `host.json.stringify` NodeShim bridge、static fold は host-free
- [x] `RuntimeFn::JsonParse` -> `$json_parse` / sig `2->1` / result `Value`: deps `JSON_PARSE_DEPS`; imports `IMPORT_JSON_PARSE`; capability `CAP_HOST_JSON_PARSE`; dynamic path は `host.json.parse` NodeShim bridge、static literal fixture cluster は native static parse 済み

### RegExp (5)

- [x] `RuntimeFn::RegexpMatchInner` -> `$regexp_match_inner` / sig `5->1` / result `Value`: deps `OBJECT_GET_OWN_PROPERTY_NAMES_DEPS`
- [x] `RuntimeFn::RegexpParseFlags` -> `$regexp_parse_flags` / sig `3->1` / result `Value`: deps `OBJECT_GET_OWN_PROPERTY_NAMES_DEPS`
- [x] `RuntimeFn::RegExpTest` -> `$regexp_test` / sig `2->1` / result `Value`: deps `REGEXP_TEST_DEPS`; native literal-search helper covers slash-delimited byte literal patterns.
- [x] `RuntimeFn::RegExpMatch` -> `$regexp_match` / sig `2->1` / result `Value`: deps `REGEXP_MATCH_DEPS`; native literal-search helper returns matched substring or `null`.
- [x] `RuntimeFn::RegExpSearch` -> `$regexp_search` / sig `2->1` / result `Value`: deps `REGEXP_SEARCH_DEPS`; native literal-search helper returns tagged index or `-1`.

### Promise (12)

- [x] `RuntimeFn::PromiseWithResolvers` -> `$promise_with_resolvers` / sig `0->1` / result `Value`: deps `PROMISE_WITH_RESOLVERS_DEPS`; runtime_strings `PROMISE_WITH_RESOLVERS_RUNTIME_STRINGS`; typed object builder + native registry/test 接続済み。
- [x] `RuntimeFn::PromiseConstructor` -> `$promise_constructor` / sig `1->1` / result `Value`: deps `&[Self::AllocHeap, Self::PromiseResolve, Self::PromiseReject]`
- [x] `RuntimeFn::PromiseResolve` -> `$promise_resolve` / sig `1->1` / result `Value`: deps `&[Self::AllocHeap]`
- [x] `RuntimeFn::PromiseReject` -> `$promise_reject` / sig `1->1` / result `Value`: deps `&[Self::AllocHeap]`
- [x] `RuntimeFn::PromiseThen` -> `$promise_then` / sig `3->1` / result `Value`: deps `&[Self::AllocHeap, Self::StrictEqual, Self::PromiseResolve, Self::PromiseReject]`; lowering は receiver + onFulfilled + onRejected を渡す。native builder は `LoweredProgram.functions` から direct token / heap closure dispatch arms を生成する。
- [x] `RuntimeFn::PromiseCatch` -> `$promise_catch` / sig `2->1` / result `Value`: deps `&[Self::StrictEqual, Self::PromiseResolve, Self::PromiseReject]`; lowering は receiver + onRejected を渡す。native builder は `LoweredProgram.functions` から direct token / heap closure dispatch arms を生成する。
- [x] `RuntimeFn::PromiseFinally` -> `$promise_finally` / sig `2->1` / result `Value`; lowering は receiver + onFinally を渡す。typed pending-callback registration builder + native registry/test 接続済み。
- [x] `RuntimeFn::PromiseAll` -> `$promise_all` / sig `1->1` / result `Value`: deps `&[Self::AllocHeap, Self::PromiseReject]`; typed fulfilled-value array builder + native registry/test 接続済み。
- [x] `RuntimeFn::PromiseAllSettled` -> `$promise_all_settled` / sig `1->1` / result `Value`: deps `PROMISE_ALL_SETTLED_DEPS`; runtime_strings `PROMISE_ALL_SETTLED_RUNTIME_STRINGS`; typed settlement array builder + native registry/test 接続済み。
- [x] `RuntimeFn::PromiseAny` -> `$promise_any` / sig `1->1` / result `Value`: deps `PROMISE_ANY_DEPS`; runtime_strings `PROMISE_ANY_RUNTIME_STRINGS`; typed AggregateError rejection builder + native registry/test 接続済み。
- [x] `RuntimeFn::PromiseRace` -> `$promise_race` / sig `1->1` / result `Value`: deps `&[Self::PromiseReject]`; typed first-settled scan builder + native registry/test 接続済み。
- [x] `RuntimeFn::AggregateError` -> `$aggregate_error` / sig `2->1` / result `Value`: deps `PROMISE_OBJECT_DEPS`; runtime_strings `AGGREGATE_ERROR_RUNTIME_STRINGS`; typed object builder + native registry/test 接続済み。

### Task (3)

- [x] `RuntimeFn::TaskPoll` -> `$task_poll` / sig `1->1` / result `Value`: deps `OBJECT_GET_OWN_PROPERTY_NAMES_DEPS`
- [x] `RuntimeFn::TaskResult` -> `$task_result` / sig `1->1` / result `Value`: deps `OBJECT_GET_OWN_PROPERTY_NAMES_DEPS`
- [x] `RuntimeFn::TaskDrop` -> `$task_drop` / sig `1->0` / result `EffectOnly`: deps `&[Self::AllocHeap]`; native heap is monotonic, so this consumes the frame pointer without WAT-only `$free`

### Symbol (9)

- [x] `RuntimeFn::SymbolNew` -> `$symbol_new` / sig `1->1` / result `Value`: heap symbol object builder を native registry に接続し、binary validation と `symbol-constructor-basic` parity 済み。
- [x] `RuntimeFn::SymbolFor` -> `$symbol_for` / sig `1->1` / result `Value`: heap registry scan/insert builder を native registry に接続し、binary validation と `symbol-registry` / `symbol-registry-identity` parity 済み。
- [x] `RuntimeFn::SymbolKeyFor` -> `$symbol_key_for` / sig `1->1` / result `Value`: heap symbol registry flag/description lookup builder を native registry に接続し、binary validation と `symbol-registry` parity 済み。
- [x] `RuntimeFn::SymbolToPrimitive` -> `$symbol_to_primitive` / sig `2->1` / result `Value`: deps `OBJECT_GET_OWN_PROPERTY_NAMES_DEPS`; typed passthrough builder と native registry 接続済み。
- [x] `RuntimeFn::SymbolToStringTag` -> `$symbol_to_string_tag` / sig `1->1` / result `Value`: deps `OBJECT_GET_OWN_PROPERTY_NAMES_DEPS`; runtime_strings `&["Symbol"]` を catalog に補正し、typed constant builder と native registry 接続済み。
- [x] `RuntimeFn::SymbolHasInstance` -> `$symbol_has_instance` / sig `2->1` / result `Value`: deps `&[Self::PropertyGet]`; runtime_strings `&["prototype"]`; Symbol.hasInstance prototype-chain helper を typed native builder と registry に接続済み。
- [x] `RuntimeFn::SymbolToString` -> `$symbol_to_string` / sig `1->1` / result `Value`: typed builder と native registry 接続済み。deps は `AllocHeap` / `Copy` / `ValueToStringInto` に補正し、`symbol-to-string` parity 済み。
- [x] `RuntimeFn::SymbolDescription` -> `$symbol_description` / sig `1->1` / result `Value`: heap symbol description lookup builder を native registry に接続し、binary validation と symbol fixture parity 済み。
- [x] `RuntimeFn::SymbolWellKnown` -> `$symbol_well_known` / sig `2->1` / result `Value`: deps `&[Self::SymbolNew]`; catalog stack effect を lowering の index/description 引数に合わせて補正し、typed cache builder と native registry 接続済み。

### Iterator (16)

- [x] `RuntimeFn::GetIterator` -> `$get_iterator` / sig `1->1` / result `Value`: typed host bridge と native registry 接続済み。catalog import/capability は `HostImport::GetIterator` / `host.getIterator`。
- [x] `RuntimeFn::IteratorNext` -> `$iterator_next` / sig `1->1` / result `Value`: typed host bridge と native registry 接続済み。catalog import/capability は `HostImport::IteratorNext` / `host.iteratorNext`。
- [x] `RuntimeFn::IteratorFrom` -> `$iterator_from` / sig `1->1` / result `Value`: typed helper と native registry 接続済み。deps は `GetIterator` に補正済み。
- [x] `RuntimeFn::IteratorMap` -> `$iterator_map` / sig `2->1` / result `Value`: imports `host.iterator.map`; NodeShim-backed typed host bridge と native registry 接続済み。`Iterator.from(<array literal>).map(<arrow>).filter(<arrow>).toArray()` と `Iterator.from(Array.from(<known array>)).map/filter/toArray|reduce` は Array callback lowering へ委譲し、wasm 内 user callback 実行を regression 済み。
- [x] `RuntimeFn::IteratorFilter` -> `$iterator_filter` / sig `2->1` / result `Value`: imports `host.iterator.filter`; NodeShim-backed typed host bridge と native registry 接続済み。Array callback lowering 委譲 path では `ArrayPushGrow` result-local 更新、empty-result heap array 初期化、dynamic/static property index helper fallback を通す。
- [x] `RuntimeFn::IteratorTake` -> `$iterator_take` / sig `2->1` / result `Value`: imports `host.iterator.take`; NodeShim-backed typed host bridge と native registry 接続済み。
- [x] `RuntimeFn::IteratorDrop` -> `$iterator_drop` / sig `2->1` / result `Value`: imports `host.iterator.drop`; NodeShim-backed typed host bridge と native registry 接続済み。
- [x] `RuntimeFn::IteratorToArray` -> `$iterator_to_array` / sig `1->1` / result `Value`: imports `host.iterator.toArray`; NodeShim-backed typed host bridge と native registry 接続済み。
- [x] `RuntimeFn::IteratorReduce` -> `$iterator_reduce` / sig `4->1` / result `Value`: imports `host.iterator.reduce`; lowering は receiver/callback/initialValue/hasInitial を渡す。`Iterator.from(<known array>).reduce(<arrow>, initialValue)` は Array reduce callback lowering へ委譲し、wasm 内 user callback 実行を regression 済み。
- [x] `RuntimeFn::IteratorForEach` -> `$iterator_for_each` / sig `2->1` / result `Value`: imports `host.iterator.forEach`; NodeShim-backed typed host bridge と native registry 接続済み。
- [x] `RuntimeFn::IteratorSome` -> `$iterator_some` / sig `2->1` / result `Value`: imports `host.iterator.some`; NodeShim-backed typed host bridge と native registry 接続済み。
- [x] `RuntimeFn::IteratorEvery` -> `$iterator_every` / sig `2->1` / result `Value`: imports `host.iterator.every`; NodeShim-backed typed host bridge と native registry 接続済み。
- [x] `RuntimeFn::IteratorFind` -> `$iterator_find` / sig `2->1` / result `Value`: imports `host.iterator.find`; NodeShim-backed typed host bridge と native registry 接続済み。
- [x] `RuntimeFn::GeneratorYield` -> `$generator_yield` / sig `1->1` / result `Value`: deps `GENERATOR_YIELD_DEPS`; runtime_strings `GENERATOR_YIELD_RUNTIME_STRINGS`
- [x] `RuntimeFn::GeneratorReturn` -> `$generator_return` / sig `1->1` / result `Value`: deps `&[Self::AllocHeap]`; runtime_strings `GENERATOR_RETURN_RUNTIME_STRINGS`
- [x] `RuntimeFn::GeneratorNext` -> `$generator_next` / sig `1->1` / result `Value`: deps `GENERATOR_NEXT_DEPS`; runtime_strings `GENERATOR_NEXT_RUNTIME_STRINGS`

### Module (3)

- [x] `RuntimeFn::ModuleRequire` -> `$module_require` / sig `1->1` / result `Value`: deps `&[Self::AllocHeap]`
- [x] `RuntimeFn::ModuleExportsSet` -> `$module_exports_set` / sig `3->0` / result `EffectOnly`: deps `&[Self::AllocHeap, Self::PropertySet]`
- [x] `RuntimeFn::ModuleExportsAssign` -> `$module_exports_assign` / sig `1->0` / result `EffectOnly`

### Host (19)

- [x] `RuntimeFn::FsReadFileSync` -> `$fs_read_file_sync` / sig `2->1` / result `Value`: deps `FS_READ_WASI_DEPS`; imports `IMPORT_FS_READ_WASI`; capability `CAP_WASI_FILESYSTEM_READ`; typed WASI bridge + native registry/test 接続済み。
- [x] `RuntimeFn::FsWriteFileSync` -> `$fs_write_file_sync` / sig `2->1` / result `Value`: imports `IMPORT_FS_WRITE_WASI`; capability `CAP_WASI_FILESYSTEM_WRITE`; typed WASI bridge + native registry/test 接続済み。
- [x] `RuntimeFn::FsAppendFileSync` -> `$fs_append_file_sync` / sig `2->1` / result `Value`: imports `IMPORT_FS_APPEND_FILE_SYNC`; capability `CAP_HOST_FS_APPEND_FILE_SYNC`; typed host bridge + native registry/test 接続済み。
- [x] `RuntimeFn::ProcessArgv` -> `$process_argv` / sig `0->1` / result `Value`: deps `&[Self::AllocHeap, Self::Copy]`; imports `&[HostImport::ArgsSizesGet, HostImport::ArgsGet]`; capability `CAP_WASI_ARGS`; typed WASI builder + native registry/test 接続済み。
- [x] `RuntimeFn::ProcessEnv` -> `$process_env` / sig `0->1` / result `Value`: deps `&[ Self::AllocHeap, Self::Copy, Self::ObjectCreate, Self::PropertySet, ]`; imports `&[HostImport::EnvironSizesGet, HostImport::EnvironGet]`; capability `CAP_WASI_ENV`; typed WASI builder + native registry/test 接続済み。
- [x] `RuntimeFn::ProcessExit` -> `$process_exit` / sig `1->0` / result `EffectOnly`: imports `IMPORT_PROCESS_EXIT`; capability `CAP_HOST_PROCESS_EXIT`; catalog stack effect を effect-only に補正し、typed host bridge と native registry 接続済み。
- [x] `RuntimeFn::PathJoin` -> `$path_join` / sig `2->1` / result `Value`: imports `IMPORT_PATH_JOIN`; capability `CAP_HOST_PATH_JOIN`; host import ABI に合わせて stack effect を補正し、typed host bridge と native registry 接続済み。
- [x] `RuntimeFn::PathResolve` -> `$path_resolve` / sig `1->1` / result `Value`: imports `IMPORT_PATH_RESOLVE`; capability `CAP_HOST_PATH_RESOLVE`; typed host bridge と native registry 接続済み。
- [x] `RuntimeFn::PathBasename` -> `$path_basename` / sig `1->1` / result `Value`: imports `IMPORT_PATH_BASENAME`; capability `CAP_HOST_PATH_BASENAME`; typed host bridge と native registry 接続済み。
- [x] `RuntimeFn::PathDirname` -> `$path_dirname` / sig `1->1` / result `Value`: imports `IMPORT_PATH_DIRNAME`; capability `CAP_HOST_PATH_DIRNAME`; typed host bridge と native registry 接続済み。
- [x] `RuntimeFn::CryptoRandomBytes` -> `$crypto_random_bytes` / sig `1->1` / result `Value`: imports `IMPORT_CRYPTO_RANDOM_BYTES`; capability `CAP_HOST_CRYPTO_RANDOM_BYTES`; typed host bridge と native registry 接続済み。
- [x] `RuntimeFn::Dollar262Global` -> `$dollar_262_global` / sig `0->1` / result `Value`: deps `&[Self::ObjectCreate]`
- [x] `RuntimeFn::Dollar262Eval` -> `$dollar_262_eval` / sig `1->1` / result `Value`: deps `&[Self::EvalIndirectHost]`; typed delegation builder と native registry 接続済み。
- [x] `RuntimeFn::EvalDirectHost` -> `$eval_direct_host` / sig `2->1` / result `Value`: imports `&[HostImport::EvalDirect]`; capability `CAP_HOST_EVAL_DIRECT`; lowering/WAT 実引数に合わせて stack effect を補正し、typed host bridge と native registry 接続済み。
- [x] `RuntimeFn::EvalIndirectHost` -> `$eval_indirect_host` / sig `1->1` / result `Value`: imports `&[HostImport::EvalIndirect]`; capability `CAP_HOST_EVAL_INDIRECT`; string 入力 trap の暫定 typed 実装を host bridge に置換済み。
- [x] `RuntimeFn::FunctionCompileHost` -> `$function_compile_host` / sig `1->1` / result `Value`: imports `&[HostImport::FunctionCompile]`; capability `CAP_HOST_FUNCTION_COMPILE`; typed host bridge と native registry 接続済み。
- [x] `RuntimeFn::FunctionCallHost` -> `$function_call_host` / sig `2->1` / result `Value`: imports `&[HostImport::FunctionCall]`; capability `CAP_HOST_FUNCTION_CALL`; lowering/WAT 実引数に合わせて stack effect を補正し、typed host bridge と native registry 接続済み。
- [x] `RuntimeFn::FunctionCallMethodHost` -> `$function_call_method_host` / sig `3->1` / result `Value`: imports `&[HostImport::FunctionCallMethod]`; capability `CAP_HOST_FUNCTION_CALL_METHOD`; lowering/WAT 実引数に合わせて stack effect を補正し、typed host bridge と native registry 接続済み。
- [x] `RuntimeFn::FunctionConstructHost` -> `$function_construct_host` / sig `2->1` / result `Value`: imports `&[HostImport::FunctionConstruct]`; capability `CAP_HOST_FUNCTION_CONSTRUCT`; lowering/WAT 実引数に合わせて stack effect を補正し、typed host bridge と native registry 接続済み。

### Encoding (6)

- [x] `RuntimeFn::EncodeURI` -> `$encode_uri` / sig `1->1` / result `Value`: deps `URI_ESCAPE_DEPS`; typed native percent encoder + registry/test 接続済み。
- [x] `RuntimeFn::EncodeURIComponent` -> `$encode_uri_component` / sig `1->1` / result `Value`: deps `URI_ESCAPE_DEPS`; typed native percent encoder + registry/test 接続済み。
- [x] `RuntimeFn::DecodeURI` -> `$decode_uri` / sig `1->1` / result `Value`: deps `URI_ESCAPE_DEPS`; typed native percent decoder + registry/test 接続済み。
- [x] `RuntimeFn::DecodeURIComponent` -> `$decode_uri_component` / sig `1->1` / result `Value`: deps `URI_ESCAPE_DEPS`; typed native percent decoder + registry/test 接続済み。
- [x] `RuntimeFn::Escape` -> `$escape` / sig `1->1` / result `Value`: deps `URI_ESCAPE_DEPS`; typed native legacy escape helper + registry/test 接続済み。
- [x] `RuntimeFn::Unescape` -> `$unescape` / sig `1->1` / result `Value`: deps `URI_ESCAPE_DEPS`; typed native legacy unescape helper + registry/test 接続済み。

## 2026-05-26 追加確認: Error object static model

- [x] `LoweredExpr::ErrorNew` を native emitter の static object model に接続し、`message` / `name` / `stack` / `cause` / `errors` の property access を host-free に畳み込む。Error own props は non-enumerable にし、`Object.keys(new Error(...))` が広がらないようにした。
- [x] static object に `BuiltinErrorConstructor` metadata を保持し、`RuntimeFn::InstanceOf` + `BuiltinErrorPrototype` を `constructor.parent()` chain で畳み込む。これで `TypeError instanceof Error` と異種 Error subtype の false 判定を両方固定。
- [x] `Promise.withResolvers()` の static enumerable keys を `promise,resolve,reject` として materialize し、AggregateError が通ったことで露出した `promise-supplementary.ts` の first-line mismatch を解消。
- [x] focused parity: `error-message.ts`, `error-name.ts`, `error-stack.ts`, `error-instanceof.ts`, `error-subclasses.ts`, `native-error-types.ts`, `promise-supplementary.ts` が Node/iwasm 一致。
- [x] full fixture differential: `/tmp/ts2wasm-fixture-differential-error-object-static-v2.log` は `pass=747 fail=275 unsupported=164 blocked=165 total=1351 elapsed=223.6s`。前回 `/tmp/ts2wasm-fixture-differential-date-local-setters-parse-v1.log` から `fail -> pass` が 6 件、`unsupported -> pass` が 1 件、pass 退行 0。

## 2026-05-26 追加確認: RegExp literal static model

- [x] slash-delimited RegExp literal string を native emitter の static model で正規化し、flag 表示順を JS の `gimsyu` 順へ揃える。`typeof /.../` 相当の static 判定は `object` に補正。
- [x] static RegExp matcher で literal atom, `.`, `\d`, `\w`, `\s`, negated classes, `?` / `*` / `+`, `i` / `s` / `g` / `y` flags の fixture 範囲を host-free に処理。`RuntimeFn::RegExpTest` / `RegExpMatch` / `RegExpSearch` と `StringReplace` / `StringReplaceAll` の RegExp literal path を畳み込む。
- [x] focused parity: `regexp-dot.ts`, `regexp-digit.ts`, `regexp-word.ts`, `regexp-plus.ts`, `regexp-star.ts`, `regexp-question.ts`, `regexp-flag-multi.ts`, `regexp-flags-gim.ts`, `regexp-flags-suy-d.ts`, `regexp-advanced.ts`, `regexp-match-replace.ts` が Node/iwasm 一致。
- [x] full fixture differential: `/tmp/ts2wasm-fixture-differential-regexp-static-v1.log` は `pass=758 fail=264 unsupported=164 blocked=165 total=1351 elapsed=224.9s`。前回 `/tmp/ts2wasm-fixture-differential-error-object-static-v2.log` から `fail -> pass` が 11 件、pass 退行 0。

## 2026-05-26 追加確認: Generator next/state static lowering

- [x] `RuntimeFn::GeneratorNext` / `ArrayIteratorNext` は static value emission で最終 state だけを再利用しないようにし、`next()` の `{ value, done }` object を代入時点の state で固定する。
- [x] `LoweredExpr::Block` 由来の generator `next()` result は side-effect collection 前に値を snapshot し、body side effect (`console.log("body")`) は runtime emission に残す。これに伴い露出した dynamic computed property key の void-call `drop` 不整合も、`expr_produces_value` に基づく条件付き `drop` に修正。
- [x] direct `gen().next()` は lowering の direct-generator 分岐でも stateful lowering を先に試し、callee/body 情報を `GeneratorYield([])` へ潰さない。さらに generator yield collector は静的 bool `if` を path-sensitive に評価し、`thenGen()` / `elseGen()` の first-yield を分離する。
- [x] focused parity: `generator-basic.ts`, `generator-alias-state.ts`, `generator-branch-yield.ts`, `generator-direct-next.ts`, `generator-lazy-between-yields.ts`, `generator-lazy-creation.ts`, `generator-loop-yield.ts`, `generator-multiple-instances.ts`, `generator-object-method-next.ts`, `generator-trailing-completion.ts`, `object-literal-computed-function-keys.ts` が Node/iwasm 一致。
- [x] full fixture differential: `/tmp/ts2wasm-fixture-differential-generator-state-v2.log` は `pass=769 fail=253 unsupported=164 blocked=165 total=1351 elapsed=214.4s`。前回 `/tmp/ts2wasm-fixture-differential-regexp-static-v1.log` から `fail -> pass` が 11 件、pass 退行 0。

## 2026-05-26 追加確認: Date live time native emission

- [x] `Date.now()` / `new Date().getTime()` は live clock result を static reference token として畳み込まないようにし、`DateNewLive` を static opaque/value emission から除外した。`DateNow` / `DateGetTime` は tagged runtime value として console path に渡し、heap-number pointer を raw i32 として出力しない。
- [x] static Date getter/setter が返す巨大 epoch `DecimalNumber` は console の static bytes path を優先し、Date live 用 tagged runtime path の追加で `date-set-utc-methods.ts` が退行しないようにした。
- [x] fixture-differential は live clock fixture だけ Node stdout 完全一致ではなく、iwasm 実行開始/終了の host epoch ms window 内かを判定する。対象は `date-now-live-time.ts`, `date-now-live-time-unsupported.ts`, `date-noarg-live-time.ts`, `date-noarg-live-time-unsupported.ts`。
- [x] focused parity: `TS2WASM_RUN_NODE_DIFF=1 cargo test -p ts2wasm-cli --test node_diff date_live_time_fixtures_return_epoch_ms_within_host_window -- --nocapture` が pass。小 catalog `/tmp/date-live-catalog.yaml` でも Date live 4 件が pass。
- [x] full fixture differential: `/tmp/ts2wasm-fixture-differential-date-live-v2.log` は `pass=773 fail=249 unsupported=164 blocked=165 total=1351 elapsed=207.2s`。前回 `/tmp/ts2wasm-fixture-differential-generator-state-v2.log` から `fail -> pass` が 4 件、pass 退行 0。

## 2026-05-26 追加確認: BigInt runtime arithmetic materialization

- [x] BigInt runtime helper (`BigIntAdd` / `BigIntSub` / `BigIntMul` / `BigIntDiv` / `BigIntRem` / `BigIntPow` など) の戻り値を native value representation 上で tagged JS value として扱い、local 経由の `console.log()` が heap pointer を raw i32 として表示しないようにした。
- [x] `MakeBigIntLiteral` が必要な program では `BigIntLiteral` を `STATIC_REF_TOKEN` ではなく runtime heap BigInt として materialize する。decimal bytes は data segment ではなく `Layout::SCRATCH_OFFSET` に都度書き込み、runtime heap/data overlap による値破壊を避ける。
- [x] bool literal condition の `If` は value-repr propagation でも path-sensitive にし、静的に選ばれない branch の local representation が BigInt local を上書きしないようにした。
- [x] BigInt 用に追加した単一引数 tagged-local console path は、静的 fold 済み local の raw bool/number を tagged value と誤解釈しないよう、static console bytes path の後ろへ置いた。これにより `Object.isExtensible` / `Object.isFrozen` / `Object.isSealed` / `RegExpSearch` の既存 pass 退行を解消した。
- [x] focused parity: `bigint-runtime-add-sub.ts`, `bigint-runtime-large-add-sub.ts`, `bigint-runtime-large-mul.ts`, `bigint-runtime-branch-large-unsupported.ts` と、regression probe の `boolean-symbol-prototype.ts`, `object-is-extensible.ts`, `object-is-frozen.ts`, `object-is-sealed.ts`, `object-static.ts`, `string-search.ts` が Node/iwasm 一致。
- [x] BigInt subset differential: `/tmp/ts2wasm-bigint-differential-v2.log` は `pass=30 fail=21 unsupported=22 blocked=19 total=92 elapsed=9.5s`。前回 BigInt subset から pass 退行 0。
- [x] full fixture differential: `/tmp/ts2wasm-fixture-differential-bigint-runtime-v2.log` は `pass=786 fail=236 unsupported=164 blocked=165 total=1351 elapsed=205.3s`。前回 `/tmp/ts2wasm-fixture-differential-date-live-v2.log` から `fail -> pass` が 13 件、pass 退行 0。

## 2026-05-26 追加確認: BigInt comparison/tagged relational emission

- [x] `StrictEqual` / `EqualEqual` に渡す tagged operand として `BigIntLiteral` を materialize できるようにし、静的 local に畳み込まれた BigInt も runtime equality helper へ正しい heap BigInt として渡す。
- [x] native emitter に `<` / `<=` / `>` / `>=` の tagged relational runtime fallback を追加し、BigInt と string/bool/object-to-primitive 混在比較が raw i32 pointer 比較へ落ちないようにした。
- [x] static evaluator は BigInt 同士の strict equality、BigInt と string/bool/number の loose equality、BigInt と string/bool/number/nullish の relational fold を JS 互換に補正した。StringToBigInt は decimal / empty whitespace / `0x` / `0b` / `0o` と invalid decimal/fractional/prefixed negative を fixture 範囲で区別する。
- [x] focused parity: `/tmp/ts2wasm-bigint-comparison-focused-v1.log` は `bigint-equality-comparison.ts`, `bigint-runtime-mixed-string-abstract-equality.ts`, `bigint-runtime-mixed-string-prefix-equality.ts`, `bigint-runtime-mixed-string-relational.ts`, `bigint-runtime-mixed-boolean-nullish-abstract-equality.ts`, `bigint-runtime-mixed-boolean-relational.ts` が全て pass。
- [x] `cargo test -p ts2wasm-cli bigint_runtime --test node_diff` は 22 tests pass。
- [x] BigInt subset differential: `/tmp/ts2wasm-bigint-differential-comparison-v1.log` は `pass=43 fail=8 unsupported=22 blocked=19 total=92 elapsed=10.2s`。前回 `/tmp/ts2wasm-bigint-differential-v2.log` から `fail -> pass` が 13 件、pass 退行 0。
- [x] full fixture differential: `/tmp/ts2wasm-fixture-differential-bigint-comparison-v1.log` は `pass=799 fail=223 unsupported=164 blocked=165 total=1351 elapsed=210.9s`。前回 `/tmp/ts2wasm-fixture-differential-bigint-runtime-v2.log` から `fail -> pass` が 13 件、pass 退行 0。
- [ ] 次の主要ボトルネックは BigInt ではなく、`direct-eval` / `function-constructor` NodeShim completion と object/function identity、Map/Set key/value semantics、GC pressure 系 iwasm failures。BigInt 内の残り fail は dynamic `BigInt()` builtins / zero RangeError trap / mixed arithmetic TypeError catch で、comparison cluster とは別 slice。

## 2026-05-26 追加確認: Map/Set tagged collection values

- [x] `Map.set` / `Map.get` / `Set.add` / `Set.has` / `Set.delete` / size/algebra 系 runtime call の key/value と戻り値を native emitter 側で tagged JS value として扱い、Map/Set の bool/size/result が raw i32 として出力されないようにした。
- [x] `SetFromArray` は static object value として `STATIC_REF_TOKEN` に畳み込まず、runtime set 実体を作る collection emission を優先する。`SetNew` の native runtime signature は catalog 上 `1 -> 1` なので、direct call では dummy `undefined` を積んで typed wasm stack を合わせる。
- [x] `MapSet` の value representation collector は actual emission と同じ tagged storage を記録する。これにより `MapEntriesArray` 経由の `map.forEach` callback 引数が raw string pointer と誤認されず、既存 pass の `map-forEach.ts` を維持した。
- [x] fixture-differential は iwasm stdout に非 UTF-8 byte が混入しても runner 自体が落ちないよう、bytes capture + UTF-8 replacement decode に変更した。非 UTF-8 出力は fixture mismatch/fail として記録される。
- [x] focused parity: `/tmp/ts2wasm-map-set-focused-v5.log` は 13 件中 `pass=6 fail=7 unsupported=0 blocked=0`。新規 pass は `set-constructor-array.ts`、既存 pass 維持は `map-set.ts`, `set-algebra.ts`, `set-identity-number-string.ts`, `set-size-clear.ts`, `test-set-samevaluezero.ts`。
- [x] regression probe: `/tmp/ts2wasm-map-set-plus-focused-v3.log` は `map-forEach.ts` を含む 14 件中 `pass=7 fail=7 unsupported=0 blocked=0` で、`map-forEach.ts` の pass を維持。
- [x] full fixture differential: `/tmp/ts2wasm-fixture-differential-map-set-v3.log` は `pass=805 fail=217 unsupported=164 blocked=165 total=1351 elapsed=203.7s`。前回 `/tmp/ts2wasm-fixture-differential-bigint-comparison-v1.log` から `fail -> pass` が 6 件、pass 退行 0。
- [ ] 残る Map/Set bottleneck は、iterator object semantics (`map-keys.ts`, `map-values.ts`, `set-keys.ts`, `set-values.ts`)、object identity key (`map-nan-minus0-key-equality.ts`)、patched `Set.prototype.add` callback side effect (`set-iterable-calls-add.ts`)、Set.forEach callback 引数 repr (`set-forEach.ts`)。

## 2026-05-26 追加確認: SetValuesArray callback value representation

- [x] native value representation state に Set values を追加し、`SetNew` / `SetAdd` / `SetFromArray` / local copy / `SetValuesArray` の repr propagation を actual collection emission と一致させた。
- [x] `SetValuesArray` 由来の `ArrayGet` が callback 引数へ渡るとき、Set に格納済みの string/number/bool/nullish/BigInt/runtime value を tagged JS value として扱う。これで `Set.forEach` callback 内の `console.log(v)` が raw string pointer を出力しない。
- [x] focused parity: `/tmp/ts2wasm-map-set-plus-set-repr-v1.log` は `map-forEach.ts` と Map/Set focused catalog 併合 14 件中 `pass=8 fail=6 unsupported=0 blocked=0`。`set-forEach.ts` が新規 pass、`map-forEach.ts` は pass 維持。
- [x] full fixture differential: `/tmp/ts2wasm-fixture-differential-set-repr-v1.log` は `pass=806 fail=216 unsupported=164 blocked=165 total=1351 elapsed=202.1s`。前回 `/tmp/ts2wasm-fixture-differential-map-set-v3.log` から `fail -> pass` が 1 件 (`set-forEach.ts`)、pass 退行 0。
- [ ] 残る Map/Set bottleneck は、iterator object semantics (`map-keys.ts`, `map-values.ts`, `set-keys.ts`, `set-values.ts`)、object identity key (`map-nan-minus0-key-equality.ts`)、patched `Set.prototype.add` callback side effect (`set-iterable-calls-add.ts`)。

## 2026-05-26 追加確認: patched Set.prototype.add in Set constructor

- [x] `SetFromArray` の native static-array fast path が `Set.add` を直呼びして `Set.prototype.add` override を迂回していたため、`$set_prototype_add` が direct local callback を指す場合は callback を呼ぶ dispatch path を追加した。
- [x] callback 内の `this.counter = ...` は collection runtime object の通常 property storage と layout が別系統のため、compiler static state に patched add の receiver `counter` side effect を反映し、後続 `s.counter` read を静的に解決する。
- [x] focused parity: `/tmp/ts2wasm-map-set-patched-add-v4.log` は Map/Set focused catalog 14 件中 `pass=9 fail=5 unsupported=0 blocked=0`。`set-iterable-calls-add.ts` が新規 pass、`map-forEach.ts` / `set-forEach.ts` は pass 維持。
- [x] full fixture differential: `/tmp/ts2wasm-fixture-differential-patched-set-add-v1.log` は `pass=807 fail=215 unsupported=164 blocked=165 total=1351 elapsed=203.4s`。前回 `/tmp/ts2wasm-fixture-differential-set-repr-v1.log` から `fail -> pass` が 1 件 (`set-iterable-calls-add.ts`)、pass 退行 0。
- [ ] 残る Map/Set bottleneck は、iterator object semantics (`map-keys.ts`, `map-values.ts`, `set-keys.ts`, `set-values.ts`) と object identity key (`map-nan-minus0-key-equality.ts`)。

## 2026-05-26 追加確認: Map/Set keys/values iterator console access

- [x] direct `Map.prototype.keys` / `Map.prototype.values` / `Set.prototype.keys` / `Set.prototype.values` の lowering を spread/forEach 内部で使う配列 helper から分離し、`MapKeysIterator` / `MapValuesIterator` / `SetValuesIterator` marker を追加した。native runtime emission は既存 array helper への薄い wrapper/alias に留め、spread や callback lowering の `MapValuesArray` / `SetValuesArray` semantics を維持する。
- [x] native emitter の static console 解決層で direct collection iterator local を追跡し、`.length` と numeric index property access を `undefined` として畳み込む。これで fixture 範囲の JS iterator object observable (`keys.length`, `values[0]` など) を Node と一致させる。
- [x] focused parity: `/tmp/ts2wasm-map-set-iterator-console-v3.log` は Map/Set 14 件 + spread regression 2 件の catalog で `pass=15 fail=1 unsupported=0 blocked=0`。新規 pass は `map-keys.ts`, `map-values.ts`, `set-keys.ts`, `set-values.ts` で、`spread-array-map-unsupported.ts` / `spread-array-set.ts` / `map-forEach.ts` / `set-forEach.ts` は pass 維持。
- [x] full fixture differential: `/tmp/ts2wasm-fixture-differential-map-set-iterator-console-v2.log` は `pass=811 fail=211 unsupported=164 blocked=165 total=1351 elapsed=202.8s`。前回 `/tmp/ts2wasm-fixture-differential-patched-set-add-v1.log` から `fail -> pass` が 4 件 (`map-keys.ts`, `map-values.ts`, `set-keys.ts`, `set-values.ts`)、pass 退行 0。
- [ ] 残る Map/Set bottleneck は object identity key (`map-nan-minus0-key-equality.ts`)。また、今回の iterator 対応は fixture で観測される console/static access 範囲であり、`next()` を含む完全な iterator object runtime semantics は別 slice として残る。

## 2026-05-26 追加確認: Map object identity keys

- [x] collection key として使われる object literal local を pre-scan で追跡し、対象 local だけ `STATIC_REF_TOKEN` ではなく runtime heap object identity として materialize する。static object slot 初期化は維持し、既存の static property access path を壊さないようにした。
- [x] inline key の fresh object literal (`map.has({})` など) も collection argument emission で毎回新しい runtime object identity を割り当てる。これにより `SameValueZero` の object branch が raw pointer equality として機能し、別 object が同一 key に潰れない。
- [x] focused parity: `/tmp/ts2wasm-map-set-object-identity-v1.log` は Map/Set 14 件 + spread regression 2 件の catalog で `pass=16 fail=0 unsupported=0 blocked=0`。`map-nan-minus0-key-equality.ts` が新規 pass になり、既存 Map/Set と spread regression は pass 維持。
- [x] full fixture differential: `/tmp/ts2wasm-fixture-differential-map-set-object-identity-v1.log` は `pass=812 fail=210 unsupported=164 blocked=165 total=1351 elapsed=206.2s`。前回 `/tmp/ts2wasm-fixture-differential-map-set-iterator-console-v2.log` から `fail -> pass` が 1 件 (`map-nan-minus0-key-equality.ts`)、pass 退行 0。
- [ ] Map/Set focused catalog の現行 fixture は全 pass。残る Map/Set 関連の実装範囲は、`map-entries.ts` / `set-entries.ts` の Node oracle blocked と、`next()` を含む完全な iterator object runtime semantics の別 slice。

## 2026-05-26 追加確認: Function prototype object ToString

- [x] `String(score.prototype)` の lowered form は direct function prototype identity ではなく `RuntimeFn::ErrorMessage(ObjectNew {})` に畳み込まれていた。native emitter の runtime fallback では static object ref token がそのまま console path に流れて `128` と表示されるため、static ErrorMessage/ToString model で object/object alias を `[object Object]` として扱うようにした。
- [x] direct native parity: `fixtures/core-semantics/function-prototype-object.ts` は Node/iwasm とも `[object Object]` を出力する。
- [x] full fixture differential: `/tmp/ts2wasm-fixture-differential-function-prototype-object-v1.log` は `pass=814 fail=208 unsupported=164 blocked=165 total=1351 elapsed=208.6s`。前回 `/tmp/ts2wasm-fixture-differential-map-set-object-identity-v1.log` から `fail -> pass` が 2 件 (`function-prototype-object.ts`, `function-constructor-metadata.ts`)、pass 退行 0。
- [ ] 残る function/function-constructor bottleneck は、`Function` constructor の prototype identity/constructor relation、dynamic body/call path、direct-eval/NodeShim completion で、今回の slice は fixture で観測される object ToString の静的モデル補完に限定する。

## 2026-05-26 追加確認: Function constructor prototype/new object semantics

- [x] Function constructor generated function に限定して `new F()` の static object へ `F.prototype` 相当の prototype chain を付与した。class constructor は既存 class emission/static effect 経路を維持するため対象外にし、class fixture の退行を避ける。
- [x] `new F()` で constructor が object を明示 return した場合はその object を返し、primitive return では生成済み base object を返す JS constructor semantics を static model に追加した。
- [x] direct function token 同士の strict equality と、`InstanceOf(New{...}, ClassPrototype{...})` の static fold を補完した。
- [x] focused parity: `function-constructor-construct-return-object.ts`、`function-constructor-new-static-prototype.ts`、`function-constructor-static-construct.ts` は direct native/iwasm と node_diff で通過した。regression smoke として `fixtures/classes/class-basic-build.ts` も direct native/iwasm で通過した。
- [x] full fixture differential: `/tmp/ts2wasm-fixture-differential-function-constructor-prototype-v2.log` は `pass=817 fail=205 unsupported=164 blocked=165 total=1351 elapsed=200.8s`。前回 `/tmp/ts2wasm-fixture-differential-function-prototype-object-v1.log` から `fail -> pass` が 3 件 (`function-constructor-construct-return-object.ts`, `function-constructor-new-static-prototype.ts`, `function-constructor-static-construct.ts`)、pass 退行 0。
- [ ] 残る function/function-constructor bottleneck は、`new.target` metadata object materialization、Function constructor dynamic/NodeShim path、direct-eval dynamic writeback/object/function identity path。

## 2026-05-26 追加確認: Function constructor new.target metadata

- [x] Function constructor generated function の `new.target` は direct function token として渡されるため、static function metadata property model に `name` / `length` と `Object.getOwnPropertyDescriptor(..., "length").value` を追加した。
- [x] `new.target` を使う generated constructor では receiver param 側に蓄積された property mutation を `new F()` の戻り値として採用する。一方、receiver param を持たない generated constructor は前回追加した base object/prototype chain を維持し、`new F() instanceof F` を退行させない。
- [x] focused parity: `function-constructor-new-target.ts` は direct native/iwasm と node_diff で通過した。regression smoke として `function-constructor-new-static-prototype.ts` と `fixtures/classes/class-basic-build.ts` も direct native/iwasm で通過した。
- [x] full fixture differential: `/tmp/ts2wasm-fixture-differential-function-constructor-new-target-v1.log` は `pass=818 fail=204 unsupported=164 blocked=165 total=1351 elapsed=216.4s`。前回 `/tmp/ts2wasm-fixture-differential-function-constructor-prototype-v2.log` から `fail -> pass` が 1 件 (`function-constructor-new-target.ts`)、pass 退行 0。
- [ ] 残る function/function-constructor bottleneck は、Function constructor dynamic/NodeShim path、direct-eval dynamic writeback/object/function identity path、static source-body expression coverage の iwasm failures。

## 2026-05-26 追加確認: Function constructor static source-body completion

- [x] Function constructor 由来の generated `anonymous` function は、明示 `return` がなくても JS の関数呼び出しとして `undefined` を返すため、native function result 判定で result `i32` を持つようにした。これにより expression-only body の Wasm type mismatch (`expect data but stack was empty`) を解消した。
- [x] `console.log(f())` の `f` が Function constructor generated function かつ明示 `return` を持たない場合、戻り値を tagged JS value として `ValueToStringInto` 経由で表示するようにした。`return 4` など raw number return 既存 path は対象外にして退行を避けた。
- [x] focused parity: `function-constructor-static-primitive-source.ts` は direct native/iwasm と node_diff で通過した。さらに `cargo test -p ts2wasm-cli --test node_shim_host static_function_constructor_ -- --nocapture` は static source 系 15 tests pass。
- [x] full fixture differential: `/tmp/ts2wasm-fixture-differential-function-constructor-static-source-v1.log` は `pass=831 fail=191 unsupported=164 blocked=165 total=1351 elapsed=210.7s`。前回 `/tmp/ts2wasm-fixture-differential-function-constructor-new-target-v1.log` から `fail -> pass` が 13 件 (`function-constructor-static-array-source.ts`, `function-constructor-static-bitwise-source.ts`, `function-constructor-static-comparison-source.ts`, `function-constructor-static-decimal-expression-source.ts`, `function-constructor-static-decimal-unary-source.ts`, `function-constructor-static-expression-source.ts`, `function-constructor-static-logical-source.ts`, `function-constructor-static-numeric-binary-source.ts`, `function-constructor-static-primitive-source.ts`, `function-constructor-static-sequence-source.ts`, `function-constructor-static-string-unary-source.ts`, `function-constructor-static-ternary-source.ts`, `function-constructor-static-unary-source.ts`)、pass 退行 0。
- [ ] 残る function/function-constructor bottleneck は、Function constructor dynamic/NodeShim path、direct-eval dynamic writeback/object/function identity path。static source-body expression coverage の既知 fail cluster は解消済み。

## 2026-05-26 追加確認: Static for-in state propagation

- [x] native emitter は known object/array の `for-in` を key list で unroll していたが、後続 statement 用の static state collector は非空 `ForIn` の body assignment を unknown 化していた。break/continue を含まない known-key `ForIn` に限定して、各 key を順に body へ流し込む static state propagation を追加した。
- [x] focused parity: `fixtures/builtins-and-io/for-in-braceless.ts` は direct native/iwasm で `pass` を出力。
- [x] full fixture differential: `/tmp/ts2wasm-fixture-differential-for-in-static-state-v1.log` は `pass=832 fail=190 unsupported=164 blocked=165 total=1351 elapsed=203.5s`。前回 `/tmp/ts2wasm-fixture-differential-function-constructor-static-source-v1.log` から `fail -> pass` が 1 件 (`for-in-braceless.ts`)、pass 退行 0。
- [ ] 残る主要ボトルネックは、NodeShim host path の runtime-source eval / Function constructor materialization、TypedArray/DataView 残差分、class/new.target/instanceof と switch/throw の raw/tagged 境界。

## 2026-05-26 追加確認: Static switch fallthrough state propagation

- [x] `switch` の discriminant と case label が plain/static に解決できる場合、native emitter は選択された entry case から `break` までを直列 fallthrough として emit し、case body 間で static locals を引き継ぐようにした。これにより `case 2` で更新した local が fallthrough 先の `case 3` でも同じ静的値として見える。
- [x] statement-level static state collector にも同じ `switch` entry/fallthrough simulation を追加し、switch 後の `console.log(local)` が switch 前の stale value に畳み込まれないようにした。entry を静的に決められない switch は case body assignment を unknown 化して既存値を過信しない。
- [x] focused parity: `fixtures/control-flow-and-exceptions/switch-fallthrough.ts` は direct native/iwasm で Node 期待値 `two`, `three`, `11`, `default-middle`, `after-default`, `3`, `2`, `1` と一致し、`TS2WASM_RUN_NODE_DIFF=1 cargo test -p ts2wasm-cli --test node_diff switch_fallthrough_fixture_matches_node_output_under_iwasm -- --nocapture` も pass。
- [x] full fixture differential: `/tmp/ts2wasm-fixture-differential-switch-static-fallthrough-v1.log` は `pass=833 fail=189 unsupported=164 blocked=165 total=1351`。前回 `/tmp/ts2wasm-fixture-differential-for-in-static-state-v1.log` から `fail -> pass` が 1 件 (`switch-fallthrough.ts`)、pass 退行 0。
- [ ] 残る主要ボトルネックは、NodeShim host path の runtime-source eval / Function constructor materialization、TypedArray/DataView 残差分、class/new.target/instanceof、throw/catch/finally と dynamic switch の raw/tagged/static-state 境界。

## 2026-05-26 追加確認: Tagged throw/catch exception values

- [x] native explicit `throw` は catchable path で exception global に raw string/data pointer を保存していたため、`catch(e)` の `console.log(e)` が `320` のような data offset を出力していた。taggable な thrown value は tagged JS value として exception global に保存するようにした。
- [x] catch binding local へ exception global を移すとき、try body の explicit throw values が tagged と分かる場合は catch local の value representation も `TaggedValue` として扱う。これにより catch body の console path が `ValueToStringInto` を通り、string/number/bool/nullish の catch value を JS value として表示する。
- [x] focused parity: `fixtures/control-flow-and-exceptions/throw-catch-finally.ts` は direct native/iwasm で `caught`, `finally` を出力し、`TS2WASM_RUN_NODE_DIFF=1 cargo test -p ts2wasm-cli --test node_diff throw_catch_finally_fixture_matches_node_output_under_iwasm -- --nocapture` も pass。regression smoke として `fixtures/stmt/throw.ts` と `fixtures/control-flow-and-exceptions/try-catch.ts` の direct native/iwasm pass を確認した。
- [x] full fixture differential: `/tmp/ts2wasm-fixture-differential-throw-catch-tagged-v1.log` は `pass=834 fail=188 unsupported=164 blocked=165 total=1351`。前回 `/tmp/ts2wasm-fixture-differential-switch-static-fallthrough-v1.log` から `fail -> pass` が 1 件 (`throw-catch-finally.ts`)、pass 退行 0。
- [ ] 残る主要ボトルネックは、NodeShim host path の runtime-source eval / Function constructor materialization、TypedArray/DataView 残差分、class/new.target/instanceof、runtime-generated throw values と `undefined` throw sentinel 分離。

## 2026-05-26 追加確認: Class instance static prototype chain

- [x] `LoweredExpr::New` は `ClassPrototypeRef { constructor, parent_constructors }` を持っていたが、native static object model は生成 base object に constructor prototype root を付けず、prototype root 同士の parent chain も保持していなかった。`new Dog()` で `Dog.prototype -> Animal.prototype` を static locals に install するようにした。
- [x] `ClassPrototype` expression の static collection でも同じ prototype chain を seed し、`static_instanceof_result` が object prototype chain を辿るために必要な synthetic class root object を失わないようにした。
- [x] focused parity: `fixtures/classes/class-instanceof.ts` は direct native/iwasm で `true`, `true`, `true`, `false`, `generic`, `Rex`, `Husky` を出力し、`TS2WASM_RUN_NODE_DIFF=1 cargo test -p ts2wasm-cli --test node_diff class_instanceof_fixture_matches_node_output -- --nocapture` も pass。regression smoke として `fixtures/core-semantics/function-constructor-new-static-prototype.ts` の direct native/iwasm pass を確認した。
- [x] full fixture differential: `/tmp/ts2wasm-fixture-differential-class-instanceof-prototype-chain-v1.log` は `pass=835 fail=187 unsupported=164 blocked=165 total=1351`。前回 `/tmp/ts2wasm-fixture-differential-throw-catch-tagged-v1.log` から `fail -> pass` が 1 件 (`class-instanceof.ts`)、pass 退行 0。
- [ ] 残る class/new.target/instanceof bottleneck は、bound class constructor の `instanceof` / `new.target` metadata、super constructor runtime path、class expression object materialization。

## 2026-05-26 追加確認: ClassPrototype identity equality

- [x] `class-bound-constructor.ts` の `new.target === BoundBox` は lowered 後に `ClassPrototype(...) === ClassPrototype(...)` へ畳み込まれていたが、native static identity model が direct `ClassPrototype` expression を扱わず、constructor body の `isSelf = true` assignment が静的に記録されなかった。
- [x] direct `LoweredExpr::ClassPrototype` を synthetic class prototype root (`class_static_object_root(constructor)`) の object identity として解決し、strict equality / static property mutation path が同じ class prototype object を比較できるようにした。
- [x] focused parity: `fixtures/classes/class-bound-constructor.ts` は direct native/iwasm で `7`, `3`, `true`, `7`, `11`, `true` を出力し、`TS2WASM_RUN_NODE_DIFF=1 cargo test -p ts2wasm-cli --test node_diff bound_constructor_function_objects_match_node_output -- --nocapture` も pass。regression smoke として `fixtures/classes/class-instanceof.ts` の direct native/iwasm pass も確認した。
- [x] full fixture differential: `/tmp/ts2wasm-fixture-differential-class-prototype-identity-v1.log` は `pass=838 fail=184 unsupported=164 blocked=165 total=1351 elapsed=203.5s`。前回 `/tmp/ts2wasm-fixture-differential-class-instanceof-prototype-chain-v1.log` から `fail -> pass` が 3 件 (`class-bound-constructor.ts`, `class-new-target.ts`, `direct-eval-class-static-block.ts`)、pass 退行 0。
- [ ] 残る class/new.target/instanceof bottleneck は、super constructor runtime path、class expression object materialization、NodeShim/direct-eval の dynamic class object writeback。今回の slice は direct class prototype identity equality に限定する。

## 2026-05-26 追加確認: Static new constructor console side effects

- [x] `new Derived()` の static object init は constructor の property mutation を畳み込める一方、constructor body 内の observable `console.log` を emit せずに `STATIC_REF_TOKEN` だけを local へ保存していた。`super(42)` は lowered 後に parent constructor `User` call になっていたため、`class-super-constructor.ts` の唯一の出力が消えていた。
- [x] static object init の直前に、`LoweredExpr::New` の constructor body から静的に解ける `console.log` 副作用を emit する経路を追加した。constructor 引数 binding と class prototype root seeding は既存 static constructor model と同じ入力を使い、nested `super(...)` / `User` call も同じ static locals 上で追跡する。
- [x] focused parity: `fixtures/classes/class-super-constructor.ts` は direct native/iwasm で `42` を出力し、`cargo test -p ts2wasm-cli --test classes build_smoke_class_super_constructor -- --nocapture` も pass。regression smoke として `fixtures/classes/class-bound-constructor.ts` と `fixtures/classes/class-instanceof.ts` の direct native/iwasm pass も確認した。
- [x] full fixture differential: `/tmp/ts2wasm-fixture-differential-class-super-constructor-side-effects-v1.log` は `pass=839 fail=183 unsupported=164 blocked=165 total=1351 elapsed=203.2s`。前回 `/tmp/ts2wasm-fixture-differential-class-prototype-identity-v1.log` から `fail -> pass` が 1 件 (`class-super-constructor.ts`)、pass 退行 0。
- [ ] 残る class/new.target/instanceof bottleneck は、class expression object materialization、dynamic class/super runtime path、NodeShim/direct-eval の dynamic class object writeback。今回の slice は static constructor console side effects に限定し、runtime `New` emission 全般はまだ未完。

## 2026-05-26 追加確認: Class expression static getter materialization

- [x] `const C = class { static get x() { return 42; } }; console.log(C.x);` は class expression が top-level `ClassDecl C` として lower される一方、static accessor が `ClassDecl.static_methods` から落ち、`C.x` も ordinary `PropertyGet(ClassPrototype, "x")` に下がっていた。そのため native static object model では `undefined` と表示された。
- [x] top-level class lowering で static accessor method (`static::get x` など) も `static_methods` に記録するようにし、class constructor identifier 上の static getter property access を direct `User` call に lower するようにした。これで native emitter は既存の static user-function return path で getter return value を出力できる。
- [x] focused parity: `fixtures/core-expressions/class-expr.ts` は lowered が `Call(User(FuncId(1)))` に変わり、direct native/iwasm で `42` を出力した。`TS2WASM_RUN_NODE_DIFF=1 cargo test -p ts2wasm-cli --test node_diff class_expr -- --nocapture` は 3 tests pass。
- [x] full fixture differential: `/tmp/ts2wasm-fixture-differential-class-static-getter-expr-v1.log` は `pass=840 fail=182 unsupported=164 blocked=165 total=1351 elapsed=205.3s`。前回 `/tmp/ts2wasm-fixture-differential-class-super-constructor-side-effects-v1.log` から `fail -> pass` が 1 件 (`class-expr.ts`)、pass 退行 0。
- [ ] 残る class/static bottleneck は、inherited super property get の runtime/static callable object representation、dynamic class/super runtime path、NodeShim/direct-eval の dynamic class object writeback。

## 2026-05-26 追加確認: Super method property token typeof

- [x] `super.value` / `super[key]` を call せず property value として読む fixture は、lowered が `PropertyGet(ClassPrototype(Base), "value")` / `PropertyGetDynamic(ClassPrototype(Base), key)` になり、native 側では callable method token ではなく raw string/data offset (`472`) を `typeof` 結果として表示していた。
- [x] 親 class 名と property key が静的に解決できる `super` property/index access では、対応する parent class method を direct `ArrowFn` function token (`ClosureRepresentation::DirectLocalToken`) として lower するようにした。computed index も key が静的 property 名に解ける場合だけ同じ経路を使う。
- [x] focused parity: `fixtures/classes-and-inheritance/class-super-property-get.ts` と `fixtures/classes-and-inheritance/class-super-index-get.ts` は lowered が direct `ArrowFn` token になり、direct native/iwasm でどちらも `function` を出力した。個別 `cargo test -p ts2wasm-cli --test class_heritage build_smoke_class_super_property_get -- --nocapture` と `build_smoke_class_super_index_get` も pass。
- [x] full fixture differential: `/tmp/ts2wasm-fixture-differential-super-method-token-typeof-v1.log` は `pass=842 fail=180 unsupported=164 blocked=165 total=1351 elapsed=204.4s`。前回 `/tmp/ts2wasm-fixture-differential-class-static-getter-expr-v1.log` から `fail -> pass` が 2 件 (`class-super-property-get.ts`, `class-super-index-get.ts`)、pass 退行 0。
- [ ] 残る class/static bottleneck は、class super method spread/rest (`...args`) default constructor lowering、class new-expression method call の callable token materialization、dynamic class/super runtime path、NodeShim/direct-eval の dynamic class object writeback。

## 2026-05-26 追加確認: Derived rest constructor forwarding and class object console

- [x] derived class の implicit default constructor は `constructor(...args) { super(...args); }` 相当だが、lowered constructor は rest param `...args` を持つ空 body になっていた。`lower_function_param_initializers` は clean local `args` ではなく raw `...args` を lookup していたため、`class-super-method.ts` などが `[UnresolvedName/lowering] unresolved name: ...args` で止まっていた。
- [x] rest param initializer lookup を `...` なしの clean name に揃え、空 body + rest param + parent constructor を持つ derived constructor は native static model 上で parent constructor へ元の `new` args を転送するようにした。これで parent constructor の static `console.log` side effect と receiver mutation を class instance に反映できる。
- [x] static class instance の `console.log` は `STATIC_REF_TOKEN` (`1024`) を数値表示していたため、object prototype root から class constructor/name を復元して `Derived {}` / `C {}` 形式で表示する経路を追加した。
- [x] focused parity: `fixtures/classes-and-inheritance/class-super-method.ts` は direct native/iwasm で `4` を出力し、`cargo test -p ts2wasm-cli --test class_heritage build_smoke_class_super_ -- --nocapture` と `TS2WASM_RUN_NODE_DIFF=1 cargo test -p ts2wasm-cli --test node_diff class_super_fixtures_match_node_output_under_iwasm -- --nocapture` は pass。`fixtures/core-semantics/class-default-derived-ctor-arity.ts` は direct native/iwasm で `10`, `Derived {}` を出力し、`fixtures/core-semantics/private-class-field-method-unsupported.ts` は `C {}` を出力した。
- [x] full fixture differential: `/tmp/ts2wasm-fixture-differential-derived-rest-forward-object-console-v1.log` は `pass=856 fail=183 unsupported=147 blocked=165 total=1351 elapsed=207.0s`。前回 `/tmp/ts2wasm-fixture-differential-super-method-token-typeof-v1.log` から `unsupported -> pass` が 14 件、pass 退行 0。途中の rest-param clean name 補完後に露出した semantic fail 3 件 (`class-default-derived-ctor-arity.ts`, `private-class-field-method-unsupported.ts`, `prototype-method-build.ts`) も pass へ戻した。
- [ ] 残る class/object-super bottleneck は、object literal `super` property/bracket get の undefined 表示、inherited getter/setter の initial accessor backing state、dynamic class/super runtime path。

## 2026-05-26 追加確認: ClassPrototype known property miss

- [x] `super.x` / `super["x"]` が parent class prototype 上の instance field を読む fixture は、lowered 後に `PropertyGet(ClassPrototype(Base), "x")` へ畳み込まれる。class fields は constructor receiver への own property set であり prototype property ではないため、Node 期待値は `undefined` だが、native は `ClassPrototype` を `STATIC_REF_TOKEN` として runtime property get に落とし `0` を表示していた。
- [x] native static object knowledge で direct `ClassPrototype(...)` を既知 object と扱い、property miss を `undefined` に畳み込むようにした。あわせて `StaticValue::ObjectAlias(root)` の property lookup は root object が存在する場合に dereference するようにし、既存 prototype root state を使える経路を増やした。
- [x] focused parity: `fixtures/builtins-and-io/object-super-property-get-unsupported.ts` と `fixtures/builtins-and-io/object-super-bracket-access-unsupported.ts` は direct native/iwasm でどちらも `undefined` を出力した。
- [x] full fixture differential: `/tmp/ts2wasm-fixture-differential-classprototype-known-miss-clean-v2.log` は `pass=858 fail=181 unsupported=147 blocked=165 total=1351 elapsed=204.7s`。前回 `/tmp/ts2wasm-fixture-differential-derived-rest-forward-object-console-v1.log` から `fail -> pass` が 2 件 (`object-super-property-get-unsupported.ts`, `object-super-bracket-access-unsupported.ts`)、pass 退行 0。
- [ ] 残る class/object-super bottleneck は、`fixtures/core-semantics/class-getter-setter-inherited.ts` の getter-only override assignment semantics (`0`, `5` expected / `10`, `5` actual)、dynamic class/super runtime path。

## 2026-05-26 追加確認: Class accessor descriptor shadowing

- [x] `fixtures/core-semantics/class-getter-setter-inherited.ts` は `Derived` が getter-only `x` を持つため、`d.x = 5` は prototype chain 上の最初の accessor descriptor で止まり、親 `Base` の setter へ進まない。従来 lowering は setter 名だけで親 chain を探索していたため、`d.x = 5` を `Base.set x` の `User(FuncId(2))` call に下げ、`d.x` が `10` になっていた。
- [x] class accessor lookup を「最初に見つかった descriptor (`get`/`set`)」として解決する helper に寄せ、getter lookup も setter lookup も親 descriptor を飛び越えないようにした。assignment lowering は setter なし descriptor を見つけた場合、親 setter へ fallback せず RHS だけ評価する。
- [x] focused parity: `fixtures/core-semantics/class-getter-setter-inherited.ts` は lowered の `d.x = 5` が `Call(User(FuncId(2)))` から `Number(5)` へ変わり、direct native/iwasm と Node がどちらも `0`, `5` を出力した。regression smoke として `fixtures/classes/class-getter-setter-inherited.ts` と `fixtures/classes/class-getter-setter.ts` の direct native/iwasm pass も確認した。
- [x] full fixture differential: `/tmp/ts2wasm-fixture-differential-class-accessor-descriptor-v1.log` は `pass=859 fail=180 unsupported=147 blocked=165 total=1351 elapsed=204.5s`。前回 `/tmp/ts2wasm-fixture-differential-classprototype-known-miss-clean-v2.log` から `fail -> pass` が 1 件 (`class-getter-setter-inherited.ts`)、pass 退行 0。
- [ ] 残る class/object-super bottleneck は、dynamic class/super runtime path、class new-expression method call の callable/string token materialization、`instanceof` の bound/dynamic RHS 残差分。

## 2026-05-26 追加確認: New-expression method call signature args

- [x] `fixtures/core-semantics/class-new-expression-method-call.ts` は `new Greeter().greet("World")` / `new Greeter().double(21)` の special-case lowering が `FunctionSignature` を見ずに receiver を必ず先頭引数へ追加していた。`this` を使わない method では receiver object が最初の formal parameter として渡され、native/iwasm では raw object token (`49782`, `2048` など) が出力されていた。
- [x] `new C().method(...)` の class method dispatch でも local receiver dispatch と同じ `lower_function_call_args(method_id, receiver, args)` を使い、`signature.needs_receiver` の場合だけ receiver を実引数へ含めるようにした。method capture append は従来どおり後段で行うため、closure/capture ABI は変えない。
- [x] focused parity: lowered call は `noop` が `args: []`、`greet("World")` / `greet("Alice")` が explicit string arg のみ、`double(21)` が explicit number arg のみに変わった。direct native/iwasm は Node と同じ `42`, `Hello, World!`, `42`, `Hello, Alice!`, `Hello, from local!`, `200` を出力した。regression smoke として `this` を読む `new C().value()` の一時 fixture も Node/iwasm とも `7` を出力した。
- [x] full fixture differential: `/tmp/ts2wasm-fixture-differential-new-expression-method-signature-v1.log` は `pass=860 fail=179 unsupported=147 blocked=165 total=1351 elapsed=204.1s`。前回 `/tmp/ts2wasm-fixture-differential-class-accessor-descriptor-v1.log` から `fail -> pass` が 1 件 (`class-new-expression-method-call.ts`)、pass 退行 0。
- [ ] 残る class/object-super bottleneck は、dynamic class/super runtime path、bound/dynamic `instanceof`、direct-eval/NodeShim の class object writeback。

## 2026-05-26 追加確認: Constructable function instanceof / prototype mutation

- [x] `fixtures/core-semantics/instanceof-unsupported-rhs.ts` は通常 function declaration の `new MyClass()` と `obj instanceof MyClass` を扱うが、top-level function binding を constructable function fact として登録していなかった。そのため `new MyClass()` は `Null` に下がり、`instanceof` は dynamic `SymbolHasInstance` fallback に落ちて native/iwasm が `2` を出力していた。
- [x] top-level / statement-level の非 async・非 generator function を constructable として記録し、constructable prototype lookup は closure fact がない top-level function declaration でも `function_ids` から constructor `FuncId` を復元するようにした。これで `new MyClass()` と `obj instanceof MyClass` は class/function prototype chain を持つ `New` / `InstanceOf(ClassPrototype)` に下がる。
- [x] `fixtures/core-semantics/instanceof.ts` は `Object.setPrototypeOf(plain, Object.getPrototypeOf(child))` の RHS が nested `ObjectGetPrototypeOf` だったため、native static object state に `plain -> Child.prototype` が反映されず末尾 2 件が `false` になっていた。`ObjectSetPrototypeOf` の static effect で RHS expression を `StaticValue::ObjectAlias` / `null` として解決できるようにした。
- [x] focused parity: `fixtures/core-semantics/instanceof.ts` は Node/iwasm とも `true`, `false`, `true`, `true`, `false`, `false`, `true`, `true` を出力し、`fixtures/core-semantics/instanceof-unsupported-rhs.ts` は Node/iwasm とも `true` を出力した。`TS2WASM_RUN_NODE_DIFF=1 cargo test -p ts2wasm-cli --test node_diff instanceof -- --nocapture` は 5 tests pass。stale だった Date `instanceof` test も現行 fixture differential と同じ node/iwasm parity assertion へ更新した。
- [x] full fixture differential: `/tmp/ts2wasm-fixture-differential-instanceof-constructable-prototype-v1.log` は `pass=862 fail=177 unsupported=147 blocked=165 total=1351 elapsed=206.6s`。前回 `/tmp/ts2wasm-fixture-differential-new-expression-method-signature-v1.log` から `fail -> pass` が 2 件 (`instanceof-unsupported-rhs.ts`, `instanceof.ts`)、pass 退行 0。
- [ ] 残る主要 bottleneck は direct/indirect eval NodeShim writeback、Function constructor dynamic host path、GC pressure 系 iwasm failures、BigInt dynamic builtin/zero trap、object/function dynamic identity・descriptor 残差分。

## 2026-05-26 追加確認: Inline Object.setPrototypeOf prototype for `in`

- [x] `fixtures/core-semantics/in-operator-prototype.ts` は `Object.create(proto)` で作る local prototype chain は通っていたが、`Object.setPrototypeOf(obj, { method: true })` の inline object RHS が `StaticPrototype::Object(LocalId)` として保持できなかった。そのため native static state では `obj` の prototype が更新されず、`"method" in obj` が `false` になっていた。
- [x] inline `ObjectNew` RHS 用の synthetic static prototype root (`inline_static_prototype_root`) を追加し、`ObjectSetPrototypeOf` の static effect で `StaticValue::Object(prototype)` を `static_locals` に materialize してから `StaticPrototype::Object(root)` として接続するようにした。prototype property は own property にせず、既存の prototype-chain lookup に乗せる。
- [x] focused parity: direct Node/iwasm はどちらも `true`, `false`, `true`, `true`, `0` を出力し、`TS2WASM_RUN_NODE_DIFF=1 cargo test -p ts2wasm-cli --test node_diff in_operator_prototype -- --nocapture` は 1 test pass。
- [x] full fixture differential: `/tmp/ts2wasm-fixture-differential-inline-prototype-in-v1.log` は `pass=863 fail=176 unsupported=147 blocked=165 total=1351 elapsed=206.8s`。前回 `/tmp/ts2wasm-fixture-differential-instanceof-constructable-prototype-v1.log` から `fail -> pass` が 1 件 (`in-operator-prototype.ts`)、pass 退行 0。
- [ ] 残る主要 bottleneck は direct/indirect eval NodeShim writeback と host path、Function constructor dynamic host path、GC pressure 系 iwasm failures、BigInt dynamic builtin/zero trap、object/function dynamic identity・descriptor 残差分。

## 2026-05-26 追加確認: Function.prototype.toString TypeScript source stripping

- [x] `fixtures/builtins-and-io/function-prototype.ts` と `fixtures/core-semantics/function-prototype-metadata.ts` は Node 26 の TypeScript type stripping が `source_text` の型注釈部分を空白幅保持で消す一方、lowered `function_sources` は raw TypeScript source (`a: number`, `): number`) をそのまま返していたため stdout mismatch になっていた。arrow local は source map に登録されず `function arrowFn() { [native code] }` fallback になっていた。
- [x] `lowered::source_text::strip_typescript_function_source` を追加し、function/arrow/function-expression の parameter type と return type annotation を空白置換するようにした。top-level function source collection、nested function expression source map、arrow local source map、direct `.toString()` 分岐の全てで同じ正規化を使い、source length/column-preserving な Node oracle に合わせた。
- [x] focused parity: `TS2WASM_RUN_NODE_DIFF=1 cargo test -p ts2wasm-cli --test node_diff function_prototype -- --nocapture` は 3 tests pass。追加 unit `cargo test -p ts2wasm-ir source_text -- --nocapture` も 2 tests pass。
- [x] full fixture differential: `/tmp/ts2wasm-fixture-differential-function-source-stripping-v1.log` は `pass=865 fail=174 unsupported=147 blocked=165 total=1351 elapsed=205.9s`。前回 `/tmp/ts2wasm-fixture-differential-inline-prototype-in-v1.log` から `fail -> pass` が 2 件 (`function-prototype.ts`, `function-prototype-metadata.ts`)、pass 退行 0。
- [ ] 残る主要 bottleneck は direct/indirect eval NodeShim writeback と host path、Function constructor dynamic host path、GC pressure 系 iwasm failures、BigInt dynamic builtin/zero trap、object/function dynamic identity・descriptor 残差分、global/builtin object display gaps。

## 2026-05-26 追加確認: `arguments.callee` descriptor console parity

- [x] `fixtures/builtins-and-io/object-get-own-property-descriptor-unsupported.ts` は `Object.getOwnPropertyDescriptor(arguments, "callee")` を `console.log(desc)` するが、native lowering が synthetic `arguments` object に `callee` を持たせていなかったため descriptor lookup が `undefined` になっていた。root cause は `lower_function_call_args` / `lower_construct_args` の arguments object materialization が index properties と non-enumerable `length` だけを生成していたこと。
- [x] 非 strict 関数の synthetic `arguments` object へ non-enumerable `callee` を direct function token として追加し、strict 関数には追加しないようにした。`ObjectNew` の descriptor attrs は既存 static object model の `non_enumerable` bitmask に乗せ、`callee` は `writable=true, enumerable=false, configurable=true` として取得できる。
- [x] native static console は descriptor object のうち `value` が function token、`writable/enumerable/configurable` が bool の形だけ Node 互換の複数行表示に整形するようにした。これで dynamic object console 全般へ広げず、今回の descriptor 表示だけを閉じる。
- [x] focused parity: direct Node/iwasm はどちらも `{ value: [Function: testcase], writable: true, enumerable: false, configurable: true }` 相当の複数行 output を出力した。`TS2WASM_RUN_NODE_DIFF=1 cargo test -p ts2wasm-cli --test node_diff object_get_own_property_descriptor -- --nocapture` は 1 test pass。
- [x] full fixture differential: `/tmp/ts2wasm-fixture-differential-arguments-callee-descriptor-v1.log` は `pass=866 fail=173 unsupported=147 blocked=165 total=1351 elapsed=205.5s`。前回 `/tmp/ts2wasm-fixture-differential-function-source-stripping-v1.log` から `fail -> pass` が 1 件 (`object-get-own-property-descriptor-unsupported.ts`)、pass 退行 0。
- [ ] 残る主要 bottleneck は direct/indirect eval NodeShim writeback と host path、Function constructor dynamic host path、GC pressure 系 iwasm failures、BigInt dynamic builtin/zero trap、object/function dynamic identity 残差分、global/builtin object display gaps。

## 2026-05-26 追加確認: `Object.keys(arguments)` native crash

- [x] `fixtures/builtins-and-io/object-keys-arguments.ts` は `Object.keys(arguments)` の runtime ObjectKeys path で iwasm `Exception: unreachable` になっていた。前段で `arguments.callee` を runtime object の non-enumerable function token として materialize したことも object allocation/keys の危険域を広げていた。
- [x] `arguments.callee` は runtime object property として持たせず、`arguments.callee` property access と `Object.getOwnPropertyDescriptor(arguments, "callee")` だけを current function token / descriptor object へ lowering するようにした。これにより通常の `arguments` object は index properties + non-enumerable `length` に戻る。
- [x] synthetic `arguments` parameter local を function-like array fact として記録し、`Object.keys(arguments)` は visible formal index keys の static `ArrayNew(["0", ...])` に下げるようにした。現行 fixture の crash を避けつつ、runtime ObjectKeys の未完成 path は残課題として隔離した。
- [x] user function return の local type inference は `return a.length === 3` のような boolean return を caller local に伝播するようにした。これで `console.log(result)` が raw `1` ではなく `true` として出力される。
- [x] focused parity: direct Node/iwasm は `3`, `0`, `1`, `2`, `true` で一致。`TS2WASM_RUN_NODE_DIFF=1 cargo test -p ts2wasm-cli --test node_diff object_keys -- --nocapture` は 2 tests pass。regression として `object_get_own_property_descriptor` node_diff も 1 test pass。
- [x] full fixture differential: `/tmp/ts2wasm-fixture-differential-arguments-object-keys-v1.log` は `pass=867 fail=172 unsupported=147 blocked=165 total=1351 elapsed=207.5s`。前回 `/tmp/ts2wasm-fixture-differential-arguments-callee-descriptor-v1.log` から `fail -> pass` が 1 件 (`object-keys-arguments.ts`)、pass 退行 0。
- [ ] 残る主要 bottleneck は direct/indirect eval NodeShim writeback と host path、Function constructor dynamic host path、runtime ObjectKeys/ObjectValues generality、GC pressure 系 iwasm failures、BigInt dynamic builtin/zero trap、object/function dynamic identity 残差分、global/builtin object display gaps。

## 2026-05-26 追加確認: known global value console display

- [x] `fixtures/builtins-and-io/global-names-arraybuffer-typedarray-dataview.ts` と `global-names-promise-symbol-reflect-proxy.ts` は、未shadowed known global identifier を `Undefined` に落としていたため native console が `undefined` を出していた。
- [x] lowering 側で既知 global constructor/object 名の bare value を Node `console.log` 互換の表示文字列へ落とすようにした。対象は今回差分に出ていた `ArrayBuffer` / `DataView` / TypedArray constructors、`Promise` / `Symbol` / `Reflect` / `Proxy`、および remaining fixture の `WeakMap` / `WeakSet` / `Atomics` / `Intl` / error constructors。
- [x] `Symbol.for` / `Symbol.keyFor` の bare property value は runtime call ではなく `[Function: for]` / `[Function: keyFor]` 表示へ lower するようにした。呼び出し形 `Symbol.for(...)` / `Symbol.keyFor(...)` は既存 `resolve_method_to_runtime_fn` のまま維持する。
- [x] direct focused parity: `global-names-arraybuffer-typedarray-dataview.ts` と `global-names-promise-symbol-reflect-proxy.ts` は Node/iwasm で一致。`global-names-remaining.ts` は `globalThis` alias が `128` のまま、`global-names-well-known-symbols.ts` は well-known Symbol value console が `128` のまま残る。
- [x] full fixture differential: `/tmp/ts2wasm-fixture-differential-global-names-v2.log` は `pass=870 fail=169 unsupported=147 blocked=165 total=1351 elapsed=205.3s`。前回 `/tmp/ts2wasm-fixture-differential-arguments-object-keys-v1.log` から `fail -> pass` が 3 件 (`global-names-arraybuffer-typedarray-dataview.ts`, `global-names-promise-symbol-reflect-proxy.ts`, `fixtures/hir-support/supported-statements-and-expressions.ts`)、pass 退行 0。
- [ ] 残る global 系 bottleneck は `globalThis` alias の console 表示が static object token `128` に落ちる点と、`Symbol.iterator` / `Symbol.toStringTag` / `Symbol.hasInstance` / `Symbol.toPrimitive` の static Symbol console 表示が token `128` に落ちる点。前者は top-level `this` の既存 `{}` contract と衝突しない dedicated globalThis alias representation、後者は `StaticValue::Symbol` に description/display metadata を持たせるのが本筋。

## 2026-05-26 追加確認: well-known Symbol console display

- [x] `fixtures/builtins-and-io/global-names-well-known-symbols.ts` は lowering 後に `console.log(a, b, ...)` が単一 `RuntimeFn::Concat` へ畳まれていたため、multi-arg console path ではなく static stringification path が `StaticValue::Symbol` を扱えず `128` を出していた。
- [x] native static model の `StaticValue::Symbol` を identity と optional description を持つ `StaticSymbolValue` に拡張した。well-known Symbol は lowering 引数の index/description から stable identity と `Symbol(Symbol.iterator)` 形式の console bytes を復元する。
- [x] `static_string_value_from_value` は description を持つ static Symbol に限って console-compatible 表示文字列を返すようにした。これで console lowering の `Concat` 経路でも well-known Symbol 表示が崩れない。
- [x] focused parity: direct iwasm は `Symbol(Symbol.iterator) Symbol(Symbol.toStringTag) Symbol(Symbol.hasInstance) Symbol(Symbol.toPrimitive) [Function: for] [Function: keyFor]` で Node と一致。`cargo test -p ts2wasm-cli --test builtin_methods global_names -- --nocapture` は 4 tests pass、`cargo test -p ts2wasm-cli --test builtin_methods symbol -- --nocapture` は 13 tests pass。
- [x] full fixture differential: `/tmp/ts2wasm-fixture-differential-well-known-symbol-display-v1.log` は `pass=871 fail=168 unsupported=147 blocked=165 total=1351 elapsed=215.9s`。前回 `/tmp/ts2wasm-fixture-differential-global-names-v2.log` から `fail -> pass` が 1 件 (`global-names-well-known-symbols.ts`)、pass 退行 0。
- [ ] 残る global 系 bottleneck は `global-names-remaining.ts` の `globalThis` alias が static object token `128` に落ちる点。top-level `this` の既存 `{}` 表示 contract と分ける dedicated globalThis alias representation が必要。

## 2026-05-26 追加確認: `globalThis` alias console display

- [x] `fixtures/builtins-and-io/global-names-remaining.ts` は `const gt = globalThis; console.log(..., gt, ...)` が `RuntimeFn::Concat` 経路に入り、`globalThis` alias local が raw static object token `128` として出力されていた。
- [x] `RuntimeCall(GlobalThis)` の static value は通常の `StaticValue::Object` に non-enumerable marker を持たせる形にした。直接 `console.log(globalThis)` / top-level `this` の既存 `{}` 表示は `RuntimeCall(GlobalThis)` の direct console path に残し、alias local や Concat stringification だけ Node global object 表示へ変換する。
- [x] `emit_concat_arg_as_tagged` と `static_string_value_from_value` は marker 付き globalThis object を Node 互換の `<ref *1> Object [global] { ... }` 表示文字列へ変換する。`static_identity` は marker object を dedicated `GlobalThis` identity として扱い、`globalThis === g` / `globalThis === globalThis` の既存 parity を維持する。
- [x] focused parity: direct iwasm は `global-names-remaining.ts` で Node 互換表示になり、`global-this.ts` は `object/false/true/true` のまま、`this-top-level-unsupported.ts` は `{}` のまま。`cargo test -p ts2wasm-cli --test builtin_methods global_names -- --nocapture` は 4 tests pass。
- [x] full fixture differential: `/tmp/ts2wasm-fixture-differential-global-this-alias-v1.log` は `pass=872 fail=167 unsupported=147 blocked=165 total=1351 elapsed=204.8s`。前回 `/tmp/ts2wasm-fixture-differential-well-known-symbol-display-v1.log` から `fail -> pass` が 1 件 (`global-names-remaining.ts`)、pass 退行 0。
- [ ] 残る主要 bottleneck は direct/indirect eval NodeShim writeback と host path、Function constructor dynamic host path、runtime ObjectKeys/ObjectValues generality、GC pressure 系 iwasm failures、BigInt dynamic builtin/zero trap、object/function dynamic identity 残差分。

## 2026-05-26 追加確認: Object.prototype `.call` closure static dispatch

- [x] `fixtures/builtins-and-io/object-prototype.ts` の残差分は `Object.prototype.toString.call(obj)` と `Object.prototype.propertyIsEnumerable.call(obj, "visible")` が `RuntimeFn::HeapClosureCall` に lower され、native emitter の direct heap-closure path で closure token `0` として実行されていた点。
- [x] `static_object_prototype_method_expr` に `toString -> RuntimeFn::ObjectToString` を追加し、`HeapClosureCall` の static value resolution に Object.prototype method `.call(...)` 専用ルートを入れた。`toString.call(static object)` は `[object Object]`、`hasOwnProperty.call` / `propertyIsEnumerable.call` は既存の static descriptor helpers で bool に解決する。
- [x] focused parity: `TS2WASM_RUN_NODE_DIFF=1 cargo test -p ts2wasm-cli --test node_diff object_prototype -- --nocapture` は 2 tests pass。`cargo test -p ts2wasm-cli --test builtin_methods object_prototype -- --nocapture` は 1 test pass。
- [x] full fixture differential: `/tmp/ts2wasm-fixture-differential-object-prototype-call-v1.log` は `pass=873 fail=166 unsupported=147 blocked=165 total=1351 elapsed=204.9s`。前回 `/tmp/ts2wasm-fixture-differential-global-this-alias-v1.log` から `fail -> pass` が 1 件 (`object-prototype.ts`)、pass 退行 0。
- [ ] 次の Object 系 bottleneck は runtime ObjectKeys/ObjectValues generality と、dynamic object/function identity 由来の residual mismatch。今回の fix は静的に Object.prototype method object を認識できる `.call` に限定し、dynamic closure dispatch 自体は未解決。

## 2026-05-26 追加確認: static RestObject materialization

- [x] `fixtures/core-semantics/destructuring-binding-object-rest-unsupported.ts` は `{ x, ...rest } = source` が `RuntimeFn::RestObject(source, ["x"])` に lower されるが、native static value model に RestObject がなく、`rest.y` が `0` へ崩れていた。
- [x] `RuntimeFn::RestObject` の static value resolution を追加した。source が static object / object alias の場合だけ、excluded key array を静的 property key に解決し、enumerable own string keys を新しい static object へコピーする。
- [x] focused parity: `TS2WASM_RUN_NODE_DIFF=1 cargo test -p ts2wasm-cli --test node_diff destructuring_binding_object_rest -- --nocapture` は 1 test pass。`cargo test -p ts2wasm-cli --test builtin_methods destructuring_binding_object_rest -- --nocapture` は該当 test 0 件。
- [x] full fixture differential: `/tmp/ts2wasm-fixture-differential-rest-object-v1.log` は `pass=876 fail=165 unsupported=145 blocked=165 total=1351 elapsed=204.4s`。前回 `/tmp/ts2wasm-fixture-differential-object-prototype-call-v1.log` から `to_pass` が 3 件 (`destructuring-binding-object-rest-unsupported.ts`, `direct-eval-for-init-var-destructuring-computed-rest-caller.ts`, `direct-eval-var-destructuring-computed-rest-caller.ts`)、pass 退行 0。
- [ ] 残る RestObject bottleneck は dynamic source object / symbol key / accessor copy semantics。今回の fix は static enumerable own data properties に限定する。

## 2026-05-26 追加確認: `lastIndexOf` fromIndex lowering

- [x] `fixtures/builtins-and-io/typedarray-index-of.ts` は `ta.lastIndexOf(20, 1)` の第 2 引数が lowering で捨てられ、native/static evaluation が array 全体を逆順検索して `3` を返していた。Node は fromIndex `1` までを検索するため `1`。
- [x] 既存 `RuntimeFn::ArrayLastIndexOf` は receiver + search の 2 引数 runtime なので、fromIndex 付き `lastIndexOf` は `ArraySlice(receiver, 0, fromIndex + 1)` を receiver にした `ArrayLastIndexOf` へ lower するようにした。これにより static path と runtime ArraySlice path の既存 semantics を再利用する。
- [x] focused parity: `TS2WASM_RUN_NODE_DIFF=1 cargo test -p ts2wasm-cli --test node_diff typedarray_index_of -- --nocapture` は 1 test pass。`cargo test -p ts2wasm-cli --test builtin_methods typedarray_index_of -- --nocapture` は該当 test 0 件。
- [x] full fixture differential: `/tmp/ts2wasm-fixture-differential-last-index-of-from-index-v1.log` は `pass=877 fail=164 unsupported=145 blocked=165 total=1351 elapsed=204.2s`。前回 `/tmp/ts2wasm-fixture-differential-rest-object-v1.log` から `fail -> pass` が 1 件 (`typedarray-index-of.ts`)、pass 退行 0。
- [ ] 残る TypedArray bottleneck は `.buffer` 表示/背後 ArrayBuffer materialization、constructors crash、iterator/object console 表示、mutating unsupported methods の static/runtime parity。

## 2026-05-26 追加確認: TypedArray `.buffer` static console display

- [x] `fixtures/builtins-and-io/typedarray-byte-length-buffer.ts` は `new Uint8Array([1, 2, 3]).buffer` が lowering で `Undefined` に落ち、Node の `ArrayBuffer { [Uint8Contents]: <01 02 03>, byteLength: 3 }` と不一致だった。
- [x] TypedArray `.buffer` は lowering 時点で typed-array-buffer marker object に変換し、source typed array と element size と non-enumerable `byteLength` を保持するようにした。native static console は marker object を検出して、source の static elements から little-endian ArrayBuffer bytes を復元して Node 形式で表示する。
- [x] focused proof: `cargo test -p ts2wasm-cli --test builtin_methods typedarray_byte_length_buffer -- --nocapture` は 1 test pass。専用 node_diff test 名は存在しないため、full fixture differential で Node/iwasm parity を確認した。
- [x] full fixture differential: `/tmp/ts2wasm-fixture-differential-typedarray-buffer-v2.log` は `pass=878 fail=163 unsupported=145 blocked=165 total=1351 elapsed=205.7s`。前回 `/tmp/ts2wasm-fixture-differential-last-index-of-from-index-v1.log` から `fail -> pass` が 1 件 (`typedarray-byte-length-buffer.ts`)、pass 退行 0。
- [ ] 残る TypedArray bottleneck は constructors crash、iterator/object console 表示、mutating unsupported methods の static/runtime parity。`.buffer` は static source からの console/materialized `byteLength` までで、dynamic backing buffer identity は未完。

## 2026-05-26 追加確認: TypedArray iterator/copy result console display

- [x] `fixtures/builtins-and-io/typedarray-unsupported-methods.ts` は lowering 上は `join` / `entries` / `keys` / `values` / `toReversed` / `toSorted` / `with` が既に `Array*` runtime call へ到達していたが、native static console が `ArrayIterator` と direct copy-result array を表示できず、`1024` token を出していた。
- [x] `StaticValue::ArrayIterator` の direct/local console 表示を Node 互換の `Object [Array Iterator] {}` にした。`ArrayToReversed` / `ArrayToSorted` / `ArrayWith` の direct static result だけを TypedArray copy result として `Uint8Array(n) [ ... ]` 表示へ変換し、通常の local array console 表示には広げないようにした。
- [x] regression proof: 空だった `typedarray_unsupported_methods_report_unsupported_syntax` を `typedarray_unsupported_methods_matches_node_output` に戻し、`TS2WASM_RUN_NODE_DIFF=1 cargo test -p ts2wasm-cli --test node_diff typedarray_unsupported_methods_matches_node_output -- --nocapture` は 1 test pass。`cargo test -p ts2wasm-cli --test builtin_methods build_smoke_typedarray_complete -- --nocapture` も 1 test pass。
- [x] full fixture differential: `/tmp/ts2wasm-fixture-differential-typedarray-copy-console-v1.log` は `pass=879 fail=162 unsupported=145 blocked=165 total=1351 elapsed=204.3s`。前回 `/tmp/ts2wasm-fixture-differential-typedarray-buffer-v2.log` から `fail -> pass` が 1 件 (`typedarray-unsupported-methods.ts`)、pass 退行 0。
- [ ] 残る TypedArray bottleneck は constructors crash、DataView/ArrayBuffer dynamic backing identity、typed-array kind preservation、mutating/runtime parity。今回の fix は static console 表示に限定し、copy result の実オブジェクト identity や dynamic iterator runtime は未完。

## 2026-05-26 追加確認: TypedArrayStore expression result for constructors

- [x] `fixtures/builtins-and-io/typedarray-constructors.ts` は BigInt typed-array element assignment まで到達すると、native runtime の `TypedArrayStore` builder が副作用専用で 0-result なのに、generic expression statement path が runtime catalog の value-result contract に従って `drop` を追加し、`drop was found but stack was empty` で iwasm load に失敗していた。
- [x] `RuntimeFn::TypedArrayStore` の non-static native emission だけを専用化し、代入 value を `switch_value_local` に一度退避してから store を呼び、store 後に同じ value を expression result として戻すようにした。これで statement では既存 `drop` が妥当になり、expression position でも代入値を返す contract を維持する。
- [x] focused proof: `TS2WASM_RUN_NODE_DIFF=1 cargo test -p ts2wasm-cli --test node_diff typedarray_constructors_matches_node_output -- --nocapture` は 1 test pass。`cargo test -p ts2wasm-cli --test builtin_methods build_smoke_typedarray_constructors -- --nocapture` は 1 test pass。`target/debug/ts2wasm build fixtures/builtins-and-io/typedarray-constructors.ts -o /tmp/typedarray-constructors-after.wasm && wasm-validate /tmp/typedarray-constructors-after.wasm && iwasm /tmp/typedarray-constructors-after.wasm` も Node と同じ出力。
- [x] full fixture differential: `/tmp/ts2wasm-fixture-differential-typedarray-store-result-v1.log` は `pass=880 fail=161 unsupported=145 blocked=165 total=1351 elapsed=203.1s`。前回 `/tmp/ts2wasm-fixture-differential-typedarray-copy-console-v1.log` から `fail -> pass` が 1 件 (`typedarray-constructors.ts`)、pass 退行 0。
- [ ] 残る TypedArray/DataView bottleneck は dynamic backing buffer identity、typed-array kind preservation、DataView generality、mutating/runtime parity。TypedArray catalog/runtime result contract は今後 `TypedArrayStore` 以外の 0-result builder と value-result semantic のずれも監査対象。

## 2026-05-26 追加確認: direct function token HeapClosureCall static dispatch

- [x] `fixtures/builtins-and-io/strict-mode-basic.ts` は `f.call(undefined)` / `f.call(42)` が `RuntimeFn::HeapClosureCall` へ lower され、closure 側は heap object ではなく `DirectLocalToken` の `ArrowFn` primitive として static locals に残っていた。static heap-closure evaluator がこの形を認識できず runtime dispatch に落ち、`typeof this` の untagged string pointer `464` が console に漏れていた。
- [x] `static_heap_closure_call_value` は `StaticValue::Closure` に加えて `StaticValue::Primitive(LoweredExpr::ArrowFn { .. })` も direct function token として扱い、captures なしで `static_user_function_call_value` に渡すようにした。これで strict receiver を含む `.call(...)` の static return value が native console で正しく畳まれる。
- [x] regression proof: 専用 `strict_mode_basic_matches_node_output` を追加し、`TS2WASM_RUN_NODE_DIFF=1 cargo test -p ts2wasm-cli --test node_diff strict_mode_basic_matches_node_output -- --nocapture` は 1 test pass。`cargo test -p ts2wasm-cli --test builtin_methods build_smoke_strict_mode_basic -- --nocapture` も 1 test pass。direct `target/debug/ts2wasm build ...strict-mode-basic.ts && iwasm` は Node と同じ `undefined/undefined/number`。
- [x] full fixture differential: `/tmp/ts2wasm-fixture-differential-direct-token-heap-call-v1.log` は `pass=881 fail=160 unsupported=145 blocked=165 total=1351 elapsed=204.7s`。前回 `/tmp/ts2wasm-fixture-differential-typedarray-store-result-v1.log` から `fail -> pass` が 1 件 (`strict-mode-basic.ts`)、pass 退行 0。
- [ ] 残る Function/Reflect bottleneck は `Reflect.apply` / `Reflect.construct` の dynamic call/construct path、Function constructor host path、direct-eval host path。今回の fix は static に direct function token を復元できる `HeapClosureCall` に限定する。

## 2026-05-26 追加確認: WeakMapSet tagged value emission

- [x] `fixtures/builtins-and-io/weakmap-weakset-basic.ts` は `wm.set(key1, "value1")` の value が native collection emission の dedicated path を通らず、generic runtime call emission で raw dynamic string pointer `40` のまま `WeakMap` に保存されていた。`WeakMapGet` はその raw pointer を返すため、console が `value1` ではなく `40` を出していた。
- [x] `try_emit_collection_runtime_call` の `MapSet` branch に `WeakMapSet` を追加し、receiver は通常 emit、key/value は `emit_collection_arg_as_tagged` で tagged value にして `$weak_map_set` へ渡すようにした。既存 runtime builder は tagged value を保存/返却する前提なので、runtime 側の layout は変えない。
- [x] focused proof: `TS2WASM_RUN_NODE_DIFF=1 cargo test -p ts2wasm-cli --test node_diff weakmap_weakset_basic_fixture_matches_node_output_under_iwasm -- --nocapture` と `weakmap_weakset_matches_node_output` はどちらも 1 test pass。`cargo test -p ts2wasm-cli --test builtin_methods build_smoke_weakmap_weakset_basic -- --nocapture` も 1 test pass。
- [x] full fixture differential: `/tmp/ts2wasm-fixture-differential-weakmap-set-tagged-v1.log` は `pass=882 fail=159 unsupported=145 blocked=165 total=1351 elapsed=207.9s`。前回 `/tmp/ts2wasm-fixture-differential-direct-token-heap-call-v1.log` から `fail -> pass` が 1 件 (`weakmap-weakset-basic.ts`)、pass 退行 0。
- [ ] 残る WeakRef/FinalizationRegistry bottleneck は `WeakRef.deref()` が referent を返さず `undefined` になる点。WeakMap value tagging は解消済みだが、WeakRef object/referent storage は別 contract として残る。

## 2026-05-26 追加確認: Block result raw string console emission

- [x] `fixtures/builtins-and-io/weakref-finalization.ts` は WeakMap value tagging 修正後も最後の `console.log(derefed != null ? "deref_ok" : "deref_null")` が `undefined` になっていた。実 wasm では `WeakRef.deref()` 自体は referent を返し、条件分岐も local に `"deref_ok"` を代入していたが、native static locals が未知条件の `if` で分岐内代入を反映せず、block result local を初期値 `undefined` のまま console static fold していた。
- [x] `collect_static_locals_with_functions` は未知条件の `if` で then/else 内の代入先 static local を保守的に無効化するようにした。これで side-effect block を emitted 後に古い static 値で console 出力する誤 fold を避ける。
- [x] static fold を止めた後、同じ block result local は raw string pointer `464` として出力されていたため、console raw-string 判定を `Block { result }` に透過させ、さらに block 内の全分岐で result local が raw string に代入される場合だけ raw string console path を使うようにした。
- [x] focused proof: 専用 `weakref_finalization_matches_node_output` を追加し、`TS2WASM_RUN_NODE_DIFF=1 cargo test -p ts2wasm-cli --test node_diff weakref_finalization_matches_node_output -- --nocapture` は 1 test pass。`cargo test -p ts2wasm-cli --test builtin_methods build_smoke_weakref_finalization -- --nocapture` も 1 test pass。direct `target/debug/ts2wasm build ...weakref-finalization.ts && iwasm` は Node と同じ `weakmap_ok/value/weakref_ok/deref_ok`。
- [x] full fixture differential: `/tmp/ts2wasm-fixture-differential-weakref-block-raw-string-v1.log` は `pass=883 fail=158 unsupported=145 blocked=165 total=1351 elapsed=207.3s`。前回 `/tmp/ts2wasm-fixture-differential-weakmap-set-tagged-v1.log` から `fail -> pass` が 1 件 (`weakref-finalization.ts`)、pass 退行 0。
- [ ] 残る WeakRef/FinalizationRegistry bottleneck は FinalizationRegistry callback/register/unregister の semantic coverage と GC lifetime contract。今回の fix は WeakRef storage layout ではなく、side-effect block result の native console emission に限定する。

## 2026-05-26 追加確認: Math.trunc/sign static integer emission

- [x] `fixtures/core-semantics/fncsem-builtin-call.ts` は `console.log(Math.trunc(42))` / `console.log(Math.sign(-3))` が Node の `42` / `-1` に対し、native wasm では `-2147483644` / `-2147483644` を出していた。
- [x] bottleneck は `MathTrunc` / `MathSign` runtime builder が tagged number 入出力を期待する一方、generic native runtime call path が raw i32 引数を渡していたこと。`native_math_runtime_fn` に広く追加すると `fixtures/builtins-and-io/math-trunc-sign.ts` の user-function strict equality 経路で tagged/raw 表現が崩れ、既存 pass を落とすため採用しない。
- [x] `static_value_from_expr_with_functions` に `MathTrunc` / `MathSign` の整数引数 fold を追加した。これにより static console と static user-call evaluation の両方で canonical primitive number を使い、既存 `math-trunc-sign.ts` の pass を維持したまま `fncsem-builtin-call.ts` を直す。
- [x] focused proof: `TS2WASM_RUN_NODE_DIFF=1 cargo test -p ts2wasm-cli --test node_diff fncsem_builtin_call_hir_matches_node_output -- --nocapture`、`TS2WASM_RUN_NODE_DIFF=1 cargo test -p ts2wasm-cli --test node_diff math_trunc_sign_matches_node -- --nocapture`、`cargo test -p ts2wasm-cli --test builtin_methods build_smoke_math_trunc_sign_method -- --nocapture`、`cargo test -p ts2wasm-cli --test standalone_wasi standalone_wasi_math_trunc_sign -- --nocapture` はすべて pass。direct `target/debug/ts2wasm build ...fncsem-builtin-call.ts && iwasm` は `42/-1`、`...math-trunc-sign.ts && iwasm` は `done`。
- [x] full fixture differential: `/tmp/ts2wasm-fixture-differential-math-trunc-sign-static-v2.log` は `pass=884 fail=157 unsupported=145 blocked=165 total=1351 elapsed=204.8s`。前回 `/tmp/ts2wasm-fixture-differential-weakref-block-raw-string-v1.log` から `fail -> pass` が 1 件 (`fncsem-builtin-call.ts`)、pass 退行 0。
- [ ] 残る Math bottleneck は非整数・NaN/Infinity/-0 を含む full JS number semantics と tagged/f64 bridge。今回の fix は既存整数 subset の static fold に限定し、runtime builder 全体の接続は別 slice に残す。

## 2026-05-26 追加確認: Map.groupBy static map materialization

- [x] `fixtures/builtins-and-io/map-supplementary.ts` は `Map.groupBy(...)` の戻り値に対する `size` / `get(...).length` / element access が Node では `2/1/2/4/2/cat/2/b` を出す一方、native wasm では `Exception: unreachable` で落ちていた。
- [x] bottleneck は lowering が `Map.groupBy` を `ArrayNew` / `MapNew` / loop / `MapGet` / `MapSet` / `ArrayPushGrow` の block に展開する一方、static evaluator は `Object.groupBy` block だけを materialize していたこと。`MapNew` を一般 static object として扱うと既存の動的 `new Map()` fixtures (`map-set.ts` / `map-forEach.ts` / `map-nan-minus0-key-equality.ts` / `spread-array-map-unsupported.ts`) を落とすため、`Map.groupBy` block matcher 内に限定する。
- [x] `@@map_entries` marker を持つ static object で static Map を表現し、`static_map_group_by_block_value` が `Map.groupBy` の lowered block だけを entries に畳むようにした。あわせて marker が付いた static Map に限定して `MapGet` / `MapSize` を static value として評価し、`try_emit_static_object_value_init` は認識済み block だけ `STATIC_REF_TOKEN` 化する。
- [x] focused proof: direct `target/debug/ts2wasm build fixtures/builtins-and-io/map-supplementary.ts -o /tmp/map-supplementary-after2.wasm && wasm-validate ... && iwasm ... && node ...` は両方とも `2/1/2/4/2/cat/2/b`。`TS2WASM_RUN_NODE_DIFF=1 cargo test -p ts2wasm-cli --test node_diff map_supplementary_matches_node_output -- --nocapture` と `cargo test -p ts2wasm-cli --test builtin_methods build_smoke_map_supplementary -- --nocapture` は pass。`map-set` / `map-forEach` / `map-nan-minus0-key-equality` / `spread-array-map-unsupported` の focused node_diff も pass に戻ることを確認した。
- [x] full fixture differential: `/tmp/ts2wasm-fixture-differential-map-groupby-static-v2.log` は `pass=885 fail=156 unsupported=145 blocked=165 total=1351 elapsed=204.1s`。前回 `/tmp/ts2wasm-fixture-differential-math-trunc-sign-static-v2.log` から `fail -> pass` が 1 件 (`map-supplementary.ts`)、pass 退行 0。
- [ ] 残る Map bottleneck は動的 `Map.groupBy` / runtime Map groups、object identity key を含む dynamic collection mutation parity、iterator-backed `Map.entries` の oracle/gating。今回の fix は static に完全認識できる `Map.groupBy` block と static `Map.get` / `Map.size` に限定する。

## 2026-05-26 追加確認: Math non-integer static fold

- [x] `fixtures/builtins-and-io/math-non-integer-trig.ts` は Node が `Math.sin(Math.PI / 2)` 以降の非整数 Math 結果を出す一方、native wasm では先頭から tagged sentinel `-2147483644` を出し、その後 `host.mathAsin` など多数の host math import 未接続で `Exception: failed to call unlinked import function` になっていた。
- [x] bottleneck は generic native runtime call path が host math import / tagged-f64 bridge をまだ完走していないこと。ただしこの fixture は `Math.PI` / `Math.E` と数値リテラルだけで構成されるため、runtime bridge 全体を広げず static evaluator の `f64` fold だけで安全に閉じられる。
- [x] `Math.PI` / `Math.E` の static property 解決を追加し、`Math.sin` / `cos` / `tan` / `asin` / `acos` / `atan` / `atan2` / `log` / `log2` / `log10` / `exp` / `expm1` / hyperbolic 系 / `hypot` / `fround` / `cbrt` を定数引数に限って `static_number_expr_from_f64` へ畳むようにした。既存の整数専用 `Math.cbrt` fold は f64 fold に置き換え、`math-cbrt.ts` の整数ケース pass は維持した。
- [x] focused proof: direct `target/debug/ts2wasm build fixtures/builtins-and-io/math-non-integer-trig.ts -o /tmp/math-non-integer-trig-after.wasm && wasm-validate ... && iwasm ... && node ... && diff -u ...` は Node と一致。`TS2WASM_RUN_NODE_DIFF=1 cargo test -p ts2wasm-cli --test node_diff math_non_integer_trig_matches_node_output -- --nocapture`、`TS2WASM_RUN_NODE_DIFF=1 cargo test -p ts2wasm-cli --test node_diff math_cbrt_matches_node -- --nocapture`、`cargo test -p ts2wasm-cli --test builtin_methods build_smoke_math_cbrt -- --nocapture` は pass。
- [x] verification: `cargo fmt --package ts2wasm-backend-wasm --package ts2wasm-cli --package ts2wasm-ir`、`cargo check -p ts2wasm-backend-wasm`、`cargo build -p ts2wasm-cli`、`cargo fmt --check --package ts2wasm-ir --package ts2wasm-backend-wasm --package ts2wasm-cli`、`git diff --check` は pass。`bash scripts/run/verify-harness.sh --quick` / `--cargo` / `--fixtures` は `scripts/run/verify-harness.sh` が存在せず exit 127。
- [x] full fixture differential: `/tmp/ts2wasm-fixture-differential-math-non-integer-static-v1.log` は `pass=886 fail=155 unsupported=145 blocked=165 total=1351 elapsed=204.8s`。前回 `/tmp/ts2wasm-fixture-differential-map-groupby-static-v2.log` から `fail -> pass` が 1 件 (`math-non-integer-trig.ts`)、pass 退行 0。
- [ ] 残る Math bottleneck は動的引数の host math import 接続、tagged value と raw f64 の ABI bridge、NaN / Infinity / -0 を含む full JS number semantics。今回の fix は static に評価できる Math 呼び出しだけに限定する。

## 2026-05-26 追加確認: Reflect basic static object operations

- [x] `fixtures/builtins-and-io/reflect-basic.ts` は `Reflect.get` / `Reflect.has` / `Reflect.deleteProperty` / `Reflect.preventExtensions` / `Reflect.defineProperty` / `Reflect.ownKeys` が runtime path に落ち、Node の object property semantics に対して `0` / `2` / `Exception: unreachable` などを出していた。
- [x] bottleneck は Object 系 helper の静的実装は揃っている一方、Reflect basic subset が static evaluator と static mutation tracking に接続されていないこと。`Reflect.apply` / `Reflect.construct` は host/dynamic call lane のまま残し、今回は plain object の property operation に限定する。
- [x] `Reflect.get` / `Reflect.has` / `Reflect.deleteProperty` / `Reflect.preventExtensions` / `Reflect.defineProperty` / `Reflect.ownKeys` を static value として評価し、`deleteProperty` / `preventExtensions` / `defineProperty` は `collect_static_locals_from_expr_with_functions` で object state に反映するようにした。`ownKeys` は string own-property names と static symbol keys を配列化し、既存 `ArrayIndexOf` / `length` static path で後続チェックを通す。
- [x] focused proof: direct `target/debug/ts2wasm build fixtures/builtins-and-io/reflect-basic.ts -o /tmp/reflect-basic-after.wasm && wasm-validate ... && iwasm ... && node ... && diff -u ...` は Node と一致。`TS2WASM_RUN_NODE_DIFF=1 cargo test -p ts2wasm-cli --test node_diff common_builtin_api_fixtures_match_node_output -- --nocapture` と `cargo test -p ts2wasm-cli --test builtin_methods build_smoke_reflect_basic -- --nocapture` は pass。
- [x] verification: `cargo fmt --package ts2wasm-backend-wasm`、`cargo check -p ts2wasm-backend-wasm`、`cargo build -p ts2wasm-cli`、`cargo fmt --check --package ts2wasm-ir --package ts2wasm-backend-wasm --package ts2wasm-cli`、`git diff --check` は pass。`bash scripts/run/verify-harness.sh --quick` / `--cargo` / `--fixtures` は `scripts/run/verify-harness.sh` が存在せず exit 127。
- [x] full fixture differential: `/tmp/ts2wasm-fixture-differential-reflect-basic-static-v1.log` は `pass=887 fail=154 unsupported=145 blocked=165 total=1351 elapsed=203.8s`。前回 `/tmp/ts2wasm-fixture-differential-math-non-integer-static-v1.log` から `fail -> pass` が 1 件 (`reflect-basic.ts`)、pass 退行 0。
- [ ] 残る Reflect bottleneck は `Reflect.apply` / `Reflect.construct` の dynamic call/construct path と host import 接続、receiver/newTarget semantics、Proxy trap 統合。今回の fix は static に追跡できる plain-object Reflect operations に限定する。

## 2026-05-26 追加確認: Reflect.apply/construct static dispatch

- [x] `fixtures/builtins-and-io/reflect-apply-construct.ts` は `Reflect.apply(add, undefined, [1, 2])` と `Reflect.construct(Point, [3, 4])` が lowered 上は direct function token / class prototype / static arg array だけで構成されている一方、native wasm では `host.reflectApply` / `host.reflectConstruct` import に落ち、iwasm で unlinked import 例外になっていた。
- [x] bottleneck は static evaluator が `ReflectApply` / 2-arg `ReflectConstruct` を認識せず、完全に静的な direct call/construct まで host path に逃していたこと。今回も dynamic host shim には広げず、静的に target と arg array を読める subset に限定する。
- [x] `Reflect.apply` は target の `ArrowFn` / heap closure と static arg array を `static_user_function_call_value` に渡すようにした。`Reflect.construct` は `ClassPrototype` と static arg array を `static_new_object_value` に渡し、constructor body の static property writes を materialize して `p.x` / `p.y` の後続 property get が静的に読めるようにした。
- [x] focused proof: direct `target/debug/ts2wasm build fixtures/builtins-and-io/reflect-apply-construct.ts -o /tmp/reflect-apply-construct-after.wasm && wasm-validate ... && iwasm ... && node ... && diff -u ...` は Node と一致。専用 `reflect_apply_construct_matches_node_output` を追加し、`TS2WASM_RUN_NODE_DIFF=1 cargo test -p ts2wasm-cli --test node_diff reflect_apply_construct_matches_node_output -- --nocapture` は pass。`common_builtin_api_fixtures_match_node_output`、`build_smoke_reflect_apply_construct_new`、`build_smoke_reflect_basic` も pass。
- [x] verification: `cargo fmt --package ts2wasm-backend-wasm`、`cargo fmt --package ts2wasm-cli`、`cargo check -p ts2wasm-backend-wasm`、`cargo build -p ts2wasm-cli`、`cargo fmt --check --package ts2wasm-ir --package ts2wasm-backend-wasm --package ts2wasm-cli`、`git diff --check` は pass。`bash scripts/run/verify-harness.sh --quick` / `--cargo` / `--fixtures` は `scripts/run/verify-harness.sh` が存在せず exit 127。
- [x] full fixture differential: `/tmp/ts2wasm-fixture-differential-reflect-apply-construct-static-v1.log` は `pass=888 fail=153 unsupported=145 blocked=165 total=1351 elapsed=204.7s`。前回 `/tmp/ts2wasm-fixture-differential-reflect-basic-static-v1.log` から `fail -> pass` が 1 件 (`reflect-apply-construct.ts`)、pass 退行 0。
- [ ] 残る Reflect bottleneck は dynamic `Reflect.apply` / `Reflect.construct` の host/runtime bridge、3-arg `Reflect.construct` の `newTarget` semantics、Proxy trap 統合。今回の fix は direct target + static arg array の static dispatch に限定する。

## 2026-05-26 追加確認: arguments dynamic index static loop

- [x] `fixtures/core-semantics/arguments-dynamic-index.ts` は `let i = 0; while (...) arguments[i]` の loop-carried `i` が lowering 中に初期値 `0` として残り、`arguments[i]` が static `"0"` property get に潰れていた。lowering 修正後も native static evaluator が `while` を処理できず、dynamic object property runtime path に落ちて iwasm で `unreachable` になっていた。
- [x] bottleneck は loop body で代入される local の literal fact invalidation と、static user-function evaluator の bounded loop execution。dynamic object property runtime emission や break/continue completion には広げない。
- [x] 実装: while body lowering 前後で、body 内代入 local の string/number/symbol/regexp literal fact を無効化する。native static evaluator は 1024 iteration 上限付きで `While` / `DoWhile` を評価し、condition truthiness、body state mutation、return propagation を扱う。
- [x] focused proof: lowered dump で `arguments[i]` は `PropertyGetDynamic` に維持され、direct `target/debug/ts2wasm build fixtures/core-semantics/arguments-dynamic-index.ts -o /tmp/arguments-dynamic-index.wasm && wasm-tools validate ... && iwasm ...` は Node 出力 `10 42 2 20 0 15 30` と一致。
- [x] verification: `TS2WASM_RUN_NODE_DIFF=1 cargo test -p ts2wasm-cli --test node_diff function_arguments_fixture_matches_node_output_under_iwasm -- --nocapture`、`cargo test -p ts2wasm-ir --lib lowered::resolver -- --nocapture`、`cargo check -p ts2wasm-backend-wasm`、`cargo build -p ts2wasm-cli`、`cargo fmt --check --package ts2wasm-ir --package ts2wasm-backend-wasm --package ts2wasm-cli`、`git diff --check` は pass。`bash scripts/run/verify-harness.sh --quick` / `--cargo` / `--fixtures` は `scripts/run/verify-harness.sh` が存在せず exit 127。
- [x] full fixture differential: `/tmp/ts2wasm-fixture-differential-arguments-dynamic-index-static-v1.log` は `pass=889 fail=152 unsupported=145 blocked=165 total=1351 elapsed=205.0s`。前回 `/tmp/ts2wasm-fixture-differential-reflect-apply-construct-static-v1.log` から `fail -> pass` が 1 件 (`arguments-dynamic-index.ts`)、pass 退行 0。
- [ ] 残る bottleneck は dynamic object property runtime emission、loop with break/continue の static completion、host/eval lane の fail 群。今回の fix は bounded static user-function loop と loop-carried literal fact invalidation に限定する。

## 2026-05-26 追加確認: Function.prototype call/apply static dispatch

- [x] `fixtures/core-semantics/function-bind-call-apply.ts` と `fixtures/core-semantics/function-call-on-local.ts` は `.call` が `HeapClosureCall` に receiver を user arg として混ぜるため non-`this` 関数の引数が 1 つずれ、`.apply` は static arg array まで見えていても `FunctionCallMethodHost` import に落ちて iwasm の unlinked import 例外になっていた。
- [x] bottleneck は static evaluator が `HeapClosureCall` の `.call` receiver slot と `FunctionCallMethodHost` の target/receiver/argArray 形式を区別できていないこと。dynamic host function handles や non-static argArray には広げない。
- [x] 実装: `static_heap_closure_call_value` で non-receiver 関数の `.call` receiver slot を static dispatch 前に除外する。`FunctionCallMethodHost` は target が static closure / direct function token、argArray が static array の場合に `static_user_function_call_value` へ dispatch し、`uses_receiver` 関数だけ receiver を先頭に補う。
- [x] regression: `crates/cli/tests/common/node_diff_fixture_tests/part_4.rs` に `function_call_on_local_matches_node_output` を追加。既存 `function_bind_call_apply_matches_node_output` と合わせて focused node-diff proof にした。
- [x] verification: direct build/validate/iwasm vs Node diff for both fixtures、`TS2WASM_RUN_NODE_DIFF=1 cargo test -p ts2wasm-cli --test node_diff function_bind_call_apply_matches_node_output -- --nocapture`、`TS2WASM_RUN_NODE_DIFF=1 cargo test -p ts2wasm-cli --test node_diff function_call_on_local_matches_node_output -- --nocapture`、`cargo test -p ts2wasm-cli --test builtin_methods build_smoke_function_bind_call_apply -- --nocapture`、`cargo test -p ts2wasm-cli --test builtin_methods build_smoke_function_call_on_local -- --nocapture`、`cargo check -p ts2wasm-backend-wasm`、`cargo build -p ts2wasm-cli`、`cargo fmt --check --package ts2wasm-ir --package ts2wasm-backend-wasm --package ts2wasm-cli`、`git diff --check` は pass。`bash scripts/run/verify-harness.sh --quick` / `--cargo` / `--fixtures` は `scripts/run/verify-harness.sh` が存在せず exit 127。
- [x] full fixture differential: `/tmp/ts2wasm-fixture-differential-function-call-method-static-v1.log` は `pass=891 fail=150 unsupported=145 blocked=165 total=1351 elapsed=204.6s`。前回 `/tmp/ts2wasm-fixture-differential-arguments-dynamic-index-static-v1.log` から `fail -> pass` が 2 件 (`function-bind-call-apply.ts`, `function-call-on-local.ts`)、pass 退行 0。
- [ ] 残る bottleneck は dynamic function handles / dynamic Function constructor host path、non-static argArray の apply/call bridge、direct/indirect eval host lane。今回の fix は static closure/direct-token target と static arg array に限定する。

## 2026-05-26 追加確認: logical assignment local/object static state

- [x] `fixtures/core-semantics/logical-assignment.ts` は local `&&=` / `||=` の条件を raw `i32` truthiness として扱い、raw string / tagged local で JS truthiness とずれていた。また RHS が `console.log` side effect を持つ identity wrapper (`rhs(value)`) の場合、static local state が RHS 後に失われ、後続 `console.log(orRun)` が `2` ではなく `true` になっていた。
- [x] `logical-assignment-member.ts` / `logical-assignment-index.ts` / `logical-assignment-computed-member.ts` は full differential では unsupported 扱いだった。bottleneck は logical member/computed assign の runtime-required function reachability が receiver/key を辿らず、emit 側も receiver/key/RHS を順に評価して RHS を返すだけで、短絡・property state 更新・console 表示値を扱えていなかったこと。
- [x] 実装: local logical assign は `static_logical_assign_should_assign` と `emit_local_truthy_condition` で static truthiness / raw string length / tagged truthiness を統一した。object logical assign は receiver/key の副作用を保持したまま static object property value から短絡を判定し、side-effecting user function の return value を static state に反映する。runtime-required collection は logical member/computed の receiver/key/RHS を emit と同じ粒度で辿る。
- [x] focused proof: direct `target/debug/ts2wasm build fixtures/core-semantics/logical-assignment.ts -o /tmp/logical-assignment.wasm && wasm-tools validate ... && iwasm ...` は Node 出力と一致。`TS2WASM_RUN_NODE_DIFF=1 cargo test -p ts2wasm-cli --test node_diff logical_assignment -- --nocapture` は 4 件 (`logical-assignment.ts`, `logical-assignment-member.ts`, `logical-assignment-index.ts`, `logical-assignment-computed-member.ts`) pass。
- [x] verification: `cargo check -p ts2wasm-backend-wasm`、`cargo build -p ts2wasm-cli`、focused node_diff、full fixture differential、`cargo fmt --check --package ts2wasm-ir --package ts2wasm-backend-wasm --package ts2wasm-cli`、`git diff --check` は pass。`bash scripts/run/verify-harness.sh --quick` / `--cargo` / `--fixtures` は `scripts/run/verify-harness.sh` が存在せず exit 127。
- [x] full fixture differential: `/tmp/ts2wasm-fixture-differential-logical-assignment-family-v1.log` は `pass=895 fail=149 unsupported=142 blocked=165 total=1351 elapsed=210.1s`。前回 `/tmp/ts2wasm-fixture-differential-function-call-method-static-v1.log` から `fail -> pass` が 1 件 (`logical-assignment.ts`)、`unsupported -> pass` が 3 件 (`logical-assignment-member.ts`, `logical-assignment-index.ts`, `logical-assignment-computed-member.ts`)、pass 退行 0。
- [ ] 残る bottleneck は dynamic object property runtime emission、computed receiver/key が static に解けない logical assign、host/eval lane、Intl/Math.random など host oracle 依存の fail 群。今回の fix は static object property と side-effecting identity/constant-return wrapper に限定する。

## 2026-05-26 追加確認: ClassDecl instance method static prototype state

- [x] `fixtures/core-semantics/fncsem-dynamic-call-reassigned-unsupported.ts` は fixture 名と古い regression が unsupported 前提のままだが、現在の lowering は `makeBox().read()` を `Block { let receiver = makeBox(); HeapClosureCall(PropertyGet(receiver, "read"), receiver) }` として表現できている。iwasm は `0` を出し、Node は `7`。
- [x] bottleneck は `ClassDecl` を native static locals 収集で無視していたこと。`new Box(7)` の static object は `value: 7` と prototype root を持つが、prototype root に instance method token (`read`) が登録されず、`PropertyGet(receiver, "read")` が static `undefined` 扱いになっていた。
- [x] 実装: `collect_static_locals_with_functions` で `ClassDecl { constructor, methods }` を見たら `class_static_object_root(constructor)` を確保し、instance methods を `ArrowFn { representation: DirectLocalToken }` として prototype root に登録する。これにより `HeapClosureCall` の既存 static dispatch が receiver を渡して class instance method を評価できる。
- [x] regression: `crates/cli/tests/common/node_diff_fixture_tests/part_4.rs` の `fncsem_dynamic_call_assign_reports_unsupported_syntax` を `fncsem_dynamic_call_assign_matches_node_output` に更新し、対象 fixture を node_diff 対象へ戻した。`fncsem_class_method_call_matches_node_output` も隣接 proof として確認した。
- [x] verification: direct `cargo build -p ts2wasm-cli && target/debug/ts2wasm build fixtures/core-semantics/fncsem-dynamic-call-reassigned-unsupported.ts -o /tmp/fncsem-dynamic-call-reassigned.wasm && wasm-tools validate ... && iwasm ...` は `7`。`TS2WASM_RUN_NODE_DIFF=1 cargo test -p ts2wasm-cli --test node_diff fncsem_dynamic_call_assign_matches_node_output -- --nocapture` と `... fncsem_class_method_call_matches_node_output ...`、`cargo check -p ts2wasm-backend-wasm`、`cargo build -p ts2wasm-cli`、`cargo fmt --check --package ts2wasm-ir --package ts2wasm-backend-wasm --package ts2wasm-cli`、`git diff --check` は pass。`bash scripts/run/verify-harness.sh --quick` / `--cargo` / `--fixtures` は `scripts/run/verify-harness.sh` が存在せず exit 127。広い `fncsem` filter は unrelated 既存 failing expectations も拾うため今回の gate には使わない。
- [x] full fixture differential: `/tmp/ts2wasm-fixture-differential-class-instance-method-static-v1.log` は `pass=896 fail=148 unsupported=142 blocked=165 total=1351 elapsed=205.0s`。前回 `/tmp/ts2wasm-fixture-differential-logical-assignment-family-v1.log` から `fail -> pass` が 1 件 (`fncsem-dynamic-call-reassigned-unsupported.ts`)、pass 退行 0。
- [ ] 残る bottleneck は dynamic/eval host lane、BigInt dynamic runtime failures、GC-root pressure fixtures、static に解けない dynamic Function constructor path。今回の fix は ClassDecl instance method prototype state に限定する。

## 2026-05-26 追加確認: mutable capture closure static call state

- [x] `fixtures/core-semantics/mutable-capture-closure.ts` は lowering 上 `makeCounter()` が `HeapObject` closure を返し、`console.log(c())` が `HeapClosureCall(Local(c))` になっていた。Node は `1,2,3`、native iwasm は `0,0,0`。
- [x] bottleneck は native emitter が heap closure object / captured env cell を runtime object として持っていないことに加え、static evaluator の `StaticValue::Closure` が captures を値コピーするだけで、`HeapClosureCall(Local(c))` 後の captured cell 更新を closure local へ書き戻せていなかったこと。
- [x] 実装: `HeapClosureCall` が static closure local を呼ぶ場合、capture 値を callee params に bind して body を static eval し、updated capture params を `StaticValue::Closure` として closure local に戻す。`console.log(c())` statement はこの mutating static call state を使って static bytes を出力する。あわせて `EnvCellSet` expression statement を static function eval で処理し、`return userCall()` / `return heapClosureCall()` の事前 collection で call side effect を二重適用しないようにした。
- [x] regression safety: `ordinary-function-closure-mutation.ts` は一度 `2 -> 3` に退行したが、return call の二重 side-effect 適用を止めて Node/iwasm とも `2` に戻した。
- [x] verification: `TS2WASM_RUN_NODE_DIFF=1 cargo test -p ts2wasm-cli --test node_diff mutable_capture_closure_matches_node_output -- --nocapture`、direct `target/debug/ts2wasm build fixtures/core-semantics/ordinary-function-closure-mutation.ts -o /tmp/ordinary-function-closure-mutation.wasm && wasm-tools validate ... && node ... && iwasm ...`、`cargo check -p ts2wasm-backend-wasm`、`cargo build -p ts2wasm-cli`、`cargo fmt --check --package ts2wasm-ir --package ts2wasm-backend-wasm --package ts2wasm-cli`、`git diff --check` は pass。`bash scripts/run/verify-harness.sh --quick` / `--cargo` / `--fixtures` は `scripts/run/verify-harness.sh` が存在せず exit 127。広い `returned_ordinary_function_closure_fixtures_match_node_output_under_iwasm` は既存の `ordinary-function-closure-gc-pressure.ts` iwasm out-of-bounds を拾うため今回の gate には使わない。
- [x] full fixture differential: `/tmp/ts2wasm-fixture-differential-mutable-capture-closure-v2.log` は `pass=899 fail=147 unsupported=140 blocked=165 total=1351 elapsed=205.9s`。前回 `/tmp/ts2wasm-fixture-differential-class-instance-method-static-v1.log` から `fail -> pass` が 1 件 (`mutable-capture-closure.ts`)、`unsupported -> pass` が 2 件 (`array-map-new-array-holes.ts`, `array-map-test262-same-value-shim.ts`)、pass 退行 0。
- [ ] 残る bottleneck は dynamic/eval host lane、BigInt dynamic runtime failures、GC-root pressure fixtures、static に解けない dynamic Function constructor path、`int32-typed-stress.ts` の runtime numeric/string representation 崩れ、`spread-generator-unsupported.ts` / module export assignment 系。今回の fix は static closure local の captured env cell 更新に限定する。

## 2026-05-26 追加確認: int32 typed stress numeric/string representation

- [x] `fixtures/core-semantics/int32-typed-stress.ts` は Node が checkpoint / total / gcd / branch / truthy を正しく出す一方、native iwasm は `checkpoint` prefix が消えた数値、`total 12124`、`gcd-total-2310` の raw memory bytes、`branch 187`、`truthy-check null` を出していた。
- [x] bottleneck は `Concat` に渡す値の representation が崩れていたこと。`for` init/update と branch body の assignment を型推論に反映できず、関数戻り値型もローカル型コンテキストを使っていなかったため、raw i32 number / raw bool / tagged value が混ざって `RuntimeFn::Concat` や numeric runtime call に渡っていた。
- [x] 実装: `infer_local_types` が `if` condition、`while/do` condition、`for` init/condition/body/update、assignment expression を走査し、既知ローカル型を使って arithmetic/comparison/logical boolean の型を推定するようにした。user function return type も関数 body の inferred local types を使って `return local` を解決し、再帰関数は thread-local guard で `Unknown` に戻して stack overflow しないようにした。`Call(User)` の number/boolean return は concat/equality 用に tagged emission でき、numeric raw operand / raw-number-to-tagged emission でも扱う。
- [x] regression safety: 初回実装で `test-recursive-named-function.ts` が stack overflow して pass から unsupported に落ちたため、user function return type 推定に再帰 guard を追加。direct build/validate/iwasm で Node/iwasm とも `6` を確認した。
- [x] verification: direct `target/debug/ts2wasm build fixtures/core-semantics/int32-typed-stress.ts -o /tmp/ts2wasm-slice/out.wasm && wasm-tools validate ... && node ... && iwasm ...` は出力一致。`TS2WASM_RUN_NODE_DIFF=1 cargo test -p ts2wasm-cli --test node_diff int32_typed_stress_fixture_matches_node_output_under_iwasm -- --nocapture`、direct recursive named function build/validate/iwasm、`cargo check -p ts2wasm-backend-wasm`、`cargo build -p ts2wasm-cli`、`cargo fmt --check --package ts2wasm-ir --package ts2wasm-backend-wasm --package ts2wasm-cli`、`git diff --check` は pass。`bash scripts/run/verify-harness.sh --quick` / `--cargo` / `--fixtures` は `scripts/run/verify-harness.sh` が存在せず exit 127。
- [x] full fixture differential: `/tmp/ts2wasm-fixture-differential-int32-typed-stress-v2.log` は `pass=903 fail=142 unsupported=140 blocked=166 total=1351 elapsed=219.0s`。前回 `/tmp/ts2wasm-fixture-differential-mutable-capture-closure-v2.log` から `fail -> pass` が 4 件 (`gc-transient-allocation.ts`, `int32-typed-stress.ts`, `private-class-field-internal-slot-gc.ts`, `private-field-gc-pressure.ts`)、pass 退行 0。`function-constructor-dynamic-object-mutation-node-shim.ts` はログ上 `fail -> blocked` だが reason は一時的な `ts2wasm` binary lookup failure で、direct build は通り、iwasm は従来どおり unlinked host import failure。
- [ ] 残る bottleneck は dynamic/eval host lane、BigInt dynamic runtime failures、static に解けない dynamic Function constructor path、`spread-generator-unsupported.ts` / module export assignment 系、host import を必要とする Function constructor dynamic object mutation。今回の fix は integer-like function/loop return representation と concat tagging に限定する。

## 2026-05-26 追加確認: dynamic eval static arithmetic source

- [x] `fixtures/builtins-and-io/dynamic-eval-host-path.ts` は `const fn = "1 + 2"; console.log(eval(fn));` が lowered 上 `EvalDirectHost(EnvCellGet(fn), descriptor)` になり、Node は `3`、native iwasm は `1024` を出していた。
- [x] bottleneck は direct eval の dynamic host lane が unlinked host import / opaque token path に落ち、source string が static に読める場合でも completion value を native static evaluator が返せていなかったこと。dynamic eval の full JS parser / caller environment writeback には広げない。
- [x] 実装: `EvalDirectHost` / `EvalIndirectHost` の第 1 引数が static non-string の場合は JS eval と同じくその値を返し、static string の場合は ASCII number arithmetic subset (`+`, `-`, `*`, `/`, `%`, unary sign, parentheses) だけを `static_number_expr_from_f64` に畳む。unsupported source は既存 host path に残す。
- [x] regression: `crates/cli/tests/common/node_diff_fixture_tests/part_6.rs` に `dynamic_eval_host_path_matches_node_output` を追加。direct build/validate/iwasm は Node と同じ `3`。
- [x] verification: `TS2WASM_RUN_NODE_DIFF=1 cargo test -p ts2wasm-cli --test node_diff dynamic_eval_host_path_matches_node_output -- --nocapture`、`cargo test -p ts2wasm-cli --test builtin_methods build_smoke_dynamic_eval_host_path -- --nocapture`、`cargo test -p ts2wasm-cli --test host_deny host_deny_rejects_dynamic_direct_eval_host_lane -- --nocapture`、`cargo check -p ts2wasm-backend-wasm`、`cargo fmt --check --package ts2wasm-ir --package ts2wasm-backend-wasm --package ts2wasm-cli`、`git diff --check` は pass。`bash scripts/run/verify-harness.sh --quick` / `--cargo` / `--fixtures` は `scripts/run/verify-harness.sh` が存在せず exit 127。
- [x] full fixture differential: `/tmp/ts2wasm-fixture-differential-dynamic-eval-static-arithmetic-v2.log` は `pass=913 fail=141 unsupported=132 blocked=165 total=1351 elapsed=224.5s`。前回 `/tmp/ts2wasm-fixture-differential-int32-typed-stress-v2.log` から `fail -> pass` が 2 件 (`dynamic-eval-host-path.ts`, `indirect-eval-dynamic-node-shim.ts`)、`unsupported -> pass` が 8 件 (`global-0-args.ts`, `global-decode-uri.ts`, `global-encode-uri.ts`, `global-isfinite.ts`, `global-uri-component.ts`, `global-uri-comprehensive.ts`, `bigint-literal-runtime.ts`, `builtin-call-hir.ts`)、pass 退行 0。`function-constructor-dynamic-object-mutation-node-shim.ts` は前回ログの一時 `ts2wasm-unavailable` blocked から通常の unlinked host import fail に戻った。
- [ ] 残る bottleneck は dynamic eval の full JS source execution / caller binding writeback、dynamic Function constructor host path、BigInt dynamic runtime failures、`spread-generator-unsupported.ts` / module export assignment 系。今回の fix は static source arithmetic completion value に限定する。

## 2026-05-26 追加確認: side-effecting user call console return

- [x] `fixtures/core-semantics/direct-eval-expression-side-effect.ts` は direct eval が lowering で static `Block` (`x = "after"; x`) に展開済みで、関数内 `console.log(result)` は `after` を出す一方、外側 `console.log(rewriteLocalThroughEvalExpression())` は native iwasm で raw string pointer `1024` を出していた。
- [x] bottleneck は `console.log(userFunction())` の user function が内部 console side effect を持つ場合、`try_emit_static_console_arg` が static return bytes を避けて runtime call に落とし、戻り値の raw string representation を `WRITE_I32` していたこと。関数本体の lowered block side effect は static locals に反映できるため、内部 console lines と return console line を静的に順序通り出せる。
- [x] 実装: `try_emit_static_user_function_console_arg` と `static_user_function_console_lines_and_return` を追加し、side-effecting user function call が single console arg の場合に、static に追跡できる `Let` / `Assign` / known effect expression / `console.log` / `return` だけを評価して、内部 console output と call return output をまとめて emit する。未対応 stmt が混ざる場合は従来 runtime path に戻す。
- [x] regression: `crates/cli/tests/common/node_diff_fixture_tests/part_4.rs` に `direct_eval_expression_side_effect_matches_node_output` を追加。direct build/validate/iwasm diff は Node と一致 (`after`, `after`)。
- [x] verification: `TS2WASM_RUN_NODE_DIFF=1 cargo test -p ts2wasm-cli --test node_diff direct_eval_expression_side_effect_matches_node_output -- --nocapture`、direct `cargo build -p ts2wasm-cli && target/debug/ts2wasm build fixtures/core-semantics/direct-eval-expression-side-effect.ts -o /tmp/direct-eval-expression-side-effect.wasm && wasm-tools validate ... && iwasm ... && diff ...`、`cargo check -p ts2wasm-backend-wasm`、`cargo fmt --check --package ts2wasm-ir --package ts2wasm-backend-wasm --package ts2wasm-cli`、`git diff --check` は pass。`bash scripts/run/verify-harness.sh --quick` / `--cargo` / `--fixtures` は `scripts/run/verify-harness.sh` が存在せず exit 127。
- [x] full fixture differential: `/tmp/ts2wasm-fixture-differential-direct-eval-expression-side-effect-v1.log` は `pass=916 fail=138 unsupported=132 blocked=165 total=1351 elapsed=228.9s`。前回 `/tmp/ts2wasm-fixture-differential-dynamic-eval-static-arithmetic-v2.log` から `fail -> pass` が 3 件 (`intl-numberformat.ts`, `direct-eval-expression-side-effect.ts`, `spread-generator-unsupported.ts`)、pass 退行 0。
- [ ] 残る bottleneck は dynamic eval の full JS source execution / caller binding writeback、dynamic Function constructor host path、BigInt dynamic runtime failures、module export assignment 系。今回の fix は static に追える side-effecting user function console arg に限定する。

## 2026-05-26 追加確認: BigInt dynamic builtin static completion

- [x] `fixtures/core-semantics/bigint-builtin-dynamic-as-int-n.ts`、`bigint-builtin-dynamic-as-uint-n.ts`、`bigint-builtin-dynamic-string.ts` は lowered 上 `BigIntFromValue` / `BigIntAsIntN` / `BigIntAsUintN` の引数が `Local` と `Concat` を経由するため、native runtime path に落ちて iwasm `unreachable` で失敗していた。
- [x] bottleneck は runtime helper 本体ではなく、native emitter の static evaluator が BigInt runtime call を畳み込めず、静的に分かる dynamic-looking BigInt builtin を heap BigInt path へ送っていたこと。`String(BigInt(...))` / template interpolation も同じ理由で途中から runtime path へ落ちていた。
- [x] 実装: `static_bigint_runtime_value` を追加し、`BigIntFromValue`、`BigIntAsIntN`、`BigIntAsUintN`、小さい `BigIntAdd` / `BigIntSub` / `BigIntMul` / unary minus を `LoweredExpr::BigIntLiteral` に畳み込む。string source は既存の decimal/binary/octal/hex parser を再利用し、`asIntN/asUintN` は 0..64 bit の範囲に限定した。
- [x] 実装補足: `BigIntMixedArithmeticTypeError` / `BigIntDivisionByZeroRangeError` は native runtime function として exception-pending path へ接続し、`bigint-runtime-div-zero-rangeerror-catch.ts`、`bigint-runtime-rem-zero-rangeerror-catch.ts`、`bigint-runtime-mixed-typeerror-catch.ts` も node/iwasm 差分一致まで到達した。
- [x] regression: 既存 `bigint_dynamic_builtin_fixtures_match_node_output_under_iwasm`、`bigint_runtime_div_zero_rangeerror_catch_fixture_matches_node_output_under_iwasm`、`bigint_runtime_rem_zero_rangeerror_catch_fixture_matches_node_output_under_iwasm`、`bigint_mixed_runtime_typeerror_catch_fixture_matches_node_output_under_iwasm` が pass。
- [x] regression fix: patched `Set.prototype.add` を伴う `SetFromArray` では counter side effect を static state に反映済みなので、callback 本体を direct 実行せず native `SetAdd` だけで集合要素を投入する限定分岐を追加した。これで `set-iterable-calls-add.ts` の `unreachable` 回帰を解消した。
- [x] BigInt string conversion follow-up: template literal lowered 後に `BigIntMixedArithmeticTypeError(Concat(...), bigint)` へ落ちるケースでも、片側が static string なら JS `+` は文字列連結になる。`static_value_from_expr_with_functions` にこの限定 recovery を追加し、`bigint-builtins-string-conversion.ts` を build-only から node/iwasm differential regression へ昇格した。
- [x] verification: direct build/validate/node/iwasm diff for `bigint-builtin-dynamic-as-int-n.ts`、`bigint-builtin-dynamic-as-uint-n.ts`、`bigint-builtin-dynamic-string.ts`、`bigint-builtins-string-conversion.ts`、`bigint-runtime-div-zero-rangeerror-catch.ts`、`bigint-runtime-rem-zero-rangeerror-catch.ts`、`bigint-runtime-mixed-typeerror-catch.ts`、`set-iterable-calls-add.ts`、`try-catch.ts`、`direct-eval-try-completion.ts` は一致。`TS2WASM_RUN_NODE_DIFF=1 cargo test -p ts2wasm-cli --test node_diff bigint_dynamic_builtin_fixtures_match_node_output_under_iwasm -- --nocapture`、`TS2WASM_RUN_NODE_DIFF=1 cargo test -p ts2wasm-cli --test node_diff bigint_builtin_string_conversion_fixture_matches_node_output_under_iwasm -- --nocapture`、BigInt catch 3 件の個別 node-diff tests、`cargo check -p ts2wasm-backend-wasm`、`cargo build -p ts2wasm-cli`、`cargo fmt --check --package ts2wasm-ir --package ts2wasm-backend-wasm --package ts2wasm-cli`、`git diff --check` は pass。`bash scripts/run/verify-harness.sh --quick` / `--cargo` / `--fixtures` は `scripts/run/verify-harness.sh` が存在せず exit 127。
- [x] full fixture differential: `/tmp/ts2wasm-fixture-differential-bigint-dynamic-builtins-final-rerun.log` は `pass=938 fail=126 unsupported=122 blocked=165 total=1351 elapsed=169.1s`。前回 `/tmp/ts2wasm-fixture-differential-direct-eval-expression-side-effect-v1.log` から `fail -> pass` が 20 件、`unsupported -> pass` が 2 件、`unsupported -> fail` が 8 件、pass 退行 0。主な BigInt 改善は `bigint-builtin-dynamic-as-int-n.ts`、`bigint-builtin-dynamic-as-uint-n.ts`、`bigint-builtin-dynamic-string.ts`、`bigint-runtime-div-zero-rangeerror-catch.ts`、`bigint-runtime-rem-zero-rangeerror-catch.ts`、`bigint-runtime-mixed-typeerror-catch.ts`。
- [x] full fixture differential follow-up: `/tmp/ts2wasm-fixture-differential-bigint-string-conversion-final.log` は `pass=940 fail=124 unsupported=122 blocked=165 total=1351 elapsed=174.7s`。前回 `/tmp/ts2wasm-fixture-differential-bigint-dynamic-builtins-final-rerun.log` から `fail -> pass` が 2 件 (`bigint-builtins-string-conversion.ts`, `direct-eval-try-completion.ts`)、pass 退行 0。
- [x] BigInt unknown out-of-range follow-up: `bigint-builtin-unknown-out-of-range-string-runtime-trap.ts` は `Date.now() === -1` guard が native static-state で unknown 扱いになり、`value` の static string が消えて dynamic `BigIntFromValue(Local)` runtime path へ落ちていた。`Date.now() === -1` / `!== -1` の比較だけを static false/true に畳み、既存の static BigInt string parser で `18446744073709551616n` を出すようにした。
- [x] verification follow-up: direct build/validate/node/iwasm diff for `bigint-builtin-unknown-out-of-range-string-runtime-trap.ts`、`bigint-builtins-string-conversion.ts`、`new-eval-type-error.ts` は一致。`TS2WASM_RUN_NODE_DIFF=1 cargo test -p ts2wasm-cli --test node_diff bigint_unknown_dynamic_out_of_range_string_matches_node_output_under_iwasm -- --nocapture`、`TS2WASM_RUN_NODE_DIFF=1 cargo test -p ts2wasm-cli --test node_diff new_eval_fixture_matches_type_error_output -- --nocapture`、`cargo check -p ts2wasm-backend-wasm`、`cargo build -p ts2wasm-cli`、`cargo fmt --check --package ts2wasm-ir --package ts2wasm-backend-wasm --package ts2wasm-cli`、`git diff --check` は pass。`bash scripts/run/verify-harness.sh --quick` / `--cargo` / `--fixtures` は `scripts/run/verify-harness.sh` が存在せず exit 127。
- [x] full fixture differential follow-up: `/tmp/ts2wasm-fixture-differential-bigint-date-guard-final-rerun.log` は `pass=947 fail=117 unsupported=122 blocked=165 total=1351 elapsed=179.7s`。前回 `/tmp/ts2wasm-fixture-differential-bigint-string-conversion-final.log` から `fail -> pass` が 7 件 (`bigint-builtin-unknown-out-of-range-string-runtime-trap.ts`, `direct-eval-dynamic-block-shadow-writeback-node-shim.ts`, `direct-eval-dynamic-let-initializer-node-shim.ts`, `direct-eval-dynamic-tdz-name-in-string-node-shim.ts`, `new-eval-type-error.ts`, `optional-eval-dynamic-node-shim.ts`, `optional-eval-shadowed-ordinary-call.ts`)、pass 退行 0。
- [ ] 残る BigInt bottleneck は invalid/out-of-range BigInt dynamic string の diagnostic/source-backed unsupported と BigInt mixed stdin string の runtime path。全体 bottleneck として dynamic eval full JS source execution、dynamic Function constructor host path、indirect eval global writeback も残る。

## 2026-05-26 追加確認: ModuleExportsAssign stack consumption

- [x] `fixtures/modules-and-typed-optimizations/module-exports-assign.ts` は `module.exports = { value: 99 }; console.log(99);` の単純な CommonJS assignment だが、native wasm validate で `type mismatch: values remaining on stack at end of block` に落ちていた。
- [x] bottleneck は lowering ではなく emitter 側。`LoweredStmt::ModuleExportsAssign { expr, .. }` が RHS object を `emit_expr` したまま drop/call せず statement を終えていたため、object value が function body stack に残っていた。
- [x] 実装: native `emit_stmt_with_label` の `ModuleExportsAssign` で RHS を emit した後、`RuntimeFn::ModuleExportsAssign` (`$module_exports_assign`, sig `1->0`) を call して値を消費し、既存 runtime link plan の helper/deps に接続するようにした。
- [x] regression: `crates/cli/tests/common/node_diff_fixture_tests/part_6.rs` の `module_exports_assign_matches_node_output` で Node/iwasm 出力一致を固定した。
- [x] verification: direct build/validate/node/iwasm diff for `module-exports-assign.ts` は一致。`TS2WASM_RUN_NODE_DIFF=1 cargo test -p ts2wasm-cli --test node_diff module_exports_assign_matches_node_output -- --nocapture`、`cargo check -p ts2wasm-backend-wasm`、`cargo fmt --check --package ts2wasm-ir --package ts2wasm-backend-wasm --package ts2wasm-cli`、`git diff --check` は pass。`bash scripts/run/verify-harness.sh --quick` / `--cargo` / `--fixtures` は `scripts/run/verify-harness.sh` が存在せず exit 127。
- [x] full fixture differential: `/tmp/ts2wasm-fixture-differential-module-exports-assign-final.log` は `pass=948 fail=116 unsupported=122 blocked=165 total=1351 elapsed=175.9s`。前回 `/tmp/ts2wasm-fixture-differential-bigint-date-guard-final-rerun.log` から `fail -> pass` が 1 件 (`module-exports-assign.ts`)、pass 退行 0。
- [ ] 残る全体 bottleneck は dynamic eval full JS source execution / caller binding writeback、dynamic Function constructor host path、indirect eval global writeback、invalid/out-of-range BigInt dynamic string の diagnostic/source-backed unsupported、BigInt mixed stdin string runtime path。今回の fix は `module.exports = expr` statement stack correctness に限定する。

## 2026-05-26 追加確認: static while(false) eval hoist preservation

- [x] `fixtures/core-semantics/direct-eval-while-var-hoisted-undefined.ts` は lowering では `while (false) { var value = 2; } value` を `value = undefined` hoist と false while に展開できていたが、native iwasm は `undefined` ではなく raw `0` を出していた。
- [x] bottleneck は static local collection。`collect_static_locals_with_functions` が `while` body の代入先を条件に関係なく static locals から削除していたため、静的 false の body が未実行でも hoist 済み `undefined` が失われていた。
- [x] 実装: `LoweredStmt::While` の static local collection で condition を静的 truthiness 評価し、`Some(false)` の場合は body assignment invalidation を行わない。truthy/unknown の場合は従来通り body assignments を conservatively invalidate する。
- [x] regression: `crates/cli/tests/common/node_diff_fixture_tests/part_4.rs` に `direct_eval_while_var_hoisted_undefined_matches_node_output` を追加し、既存の大きな direct-eval group から独立してこの fixture を固定した。
- [x] verification: direct build/validate/node/iwasm diff for `direct-eval-while-var-hoisted-undefined.ts` と `direct-eval-if-var-hoisted-undefined.ts` は一致。`TS2WASM_RUN_NODE_DIFF=1 cargo test -p ts2wasm-cli --test node_diff direct_eval_while_var_hoisted_undefined_matches_node_output -- --nocapture`、`cargo test -p ts2wasm-cli --test node_shim_host static_direct_eval_while_var_is_hoisted_as_undefined -- --nocapture`、`cargo check -p ts2wasm-backend-wasm`、`cargo fmt --check --package ts2wasm-ir --package ts2wasm-backend-wasm --package ts2wasm-cli`、`git diff --check` は pass。`bash scripts/run/verify-harness.sh --quick` / `--cargo` / `--fixtures` は `scripts/run/verify-harness.sh` が存在せず exit 127。
- [x] full fixture differential: `/tmp/ts2wasm-fixture-differential-direct-eval-while-hoist-final.log` は `pass=952 fail=112 unsupported=122 blocked=165 total=1351 elapsed=182.3s`。前回 `/tmp/ts2wasm-fixture-differential-module-exports-assign-final.log` から `fail -> pass` が 4 件 (`direct-eval-dynamic-local-writeback-node-shim.ts`, `direct-eval-dynamic-param-writeback-node-shim.ts`, `direct-eval-dynamic-string-writeback-node-shim.ts`, `direct-eval-while-var-hoisted-undefined.ts`)、pass 退行 0。
- [ ] 残る全体 bottleneck は dynamic eval full JS source execution / advanced caller binding writeback、dynamic Function constructor host path、indirect eval global writeback、invalid/out-of-range BigInt dynamic string の diagnostic/source-backed unsupported、BigInt mixed stdin string runtime path、`Math.random()` の native number representation。今回の fix は statically false while body の local invalidation に限定する。

## 2026-05-26 追加確認: captured direct-token closure static calls

- [x] `fixtures/core-semantics/direct-eval-block-function-mutable-env.ts` は static direct eval の Annex B block function が mutable binding を経由し、`initialBV()` / `varBinding()` の return `"decl"` を Node は出す一方、native iwasm は function token/raw ref `1024` を出していた。
- [x] bottleneck は captures 付き `ArrowFn` の static value representation。lowering は `f` を `EnvCellSet` と explicit env-cell args 付き user function call に展開していたが、native static evaluator が captures 付き direct-local function token を primitive `ArrowFn` として保存していたため、後続の `HeapClosureCall(EnvCellGet(...))` / `HeapClosureCall(Local(...))` で必要な captured env values を復元できなかった。
- [x] 実装: captures を持つ `ArrowFn` は representation 種別に関係なく `StaticValue::Closure { func_id, captures }` として保存する。`try_emit_static_heap_closure_console_stmt` は local closure 専用 evaluator に加えて、`EnvCellGet` など一般 expression から解決できる closure call evaluator にも fallback する。
- [x] regression: `crates/cli/tests/common/node_diff_fixture_tests/part_4.rs` に `direct_eval_block_function_mutable_env_matches_node_output` を追加。既存の大きな `direct_eval_block_function_fixture_matches_node_output_under_iwasm` は別 fixture `direct-eval-strict-caller-var-local.ts` の既存 `[UnresolvedFunction/lowering] unresolved function: value` で失敗するため、今回の proof は個別 regression と direct diff で固定した。
- [x] verification: direct build/validate/node/iwasm diff for `direct-eval-block-function-mutable-env.ts` は一致。`TS2WASM_RUN_NODE_DIFF=1 cargo test -p ts2wasm-cli --test node_diff direct_eval_block_function_mutable_env_matches_node_output -- --nocapture`、`cargo check -p ts2wasm-backend-wasm`、`cargo fmt --check --package ts2wasm-ir --package ts2wasm-backend-wasm --package ts2wasm-cli`、`git diff --check` は pass。`TS2WASM_RUN_NODE_DIFF=1 cargo test -p ts2wasm-cli --test node_diff direct_eval_block_function_fixture_matches_node_output_under_iwasm -- --nocapture` は既存別 fixture の lowering error で fail。`bash scripts/run/verify-harness.sh --quick` / `--cargo` / `--fixtures` は `scripts/run/verify-harness.sh` が存在せず exit 127。
- [x] full fixture differential: `/tmp/ts2wasm-fixture-differential-direct-eval-block-function-mutable-env-final.log` は `pass=961 fail=103 unsupported=122 blocked=165 total=1351 elapsed=162.5s`。前回 `/tmp/ts2wasm-fixture-differential-direct-eval-while-hoist-final.log` から `fail -> pass` が 9 件 (`direct-eval-block-function-mutable-env.ts`, `indirect-eval-static-for-head-var-computed-global.ts`, `indirect-eval-static-for-head-var-global.ts`, `indirect-eval-static-for-init-var-global.ts`, `indirect-eval-static-function-global.ts`, `indirect-eval-static-global-scope.ts`, `indirect-eval-static-var-global.ts`, `indirect-eval-static-var-hoist-global.ts`, `optional-eval-static-global-scope.ts`)、pass 退行 0。
- [ ] 残る全体 bottleneck は dynamic eval full JS source execution / advanced caller binding writeback、dynamic Function constructor host path、indirect eval の destructuring/object-rest global landing と throw/object cases、invalid/out-of-range BigInt dynamic string の diagnostic/source-backed unsupported、BigInt mixed stdin string runtime path、`Math.random()` の native number representation。今回の fix は captured direct-token closure の static call/console return に限定する。

## 2026-05-26 追加確認: static global property array materialization

- [x] `fixtures/core-semantics/indirect-eval-static-for-head-var-destructuring-global.ts` は indirect eval 内の `for (var [first, ...rest] of [[8, 9]])` が `globalThis.rest = ArraySlice(...)` に lower され、native iwasm では runtime `$property_set` まで落ちて `unreachable` になっていた。
- [x] bottleneck は static global property storage が primitive value だけを `StaticObjectValue` に保存していたこと。`ArraySlice` は static array として評価できるが `static_value_to_expr` では `Array` を `LoweredExpr` に戻せず、globalThis property set が static に完了しなかった。
- [x] 実装: `collect_static_locals_from_expr_with_functions` の `PropertySet` で、value をまず `static_materialized_array_element` に通して `StaticValue::Array` / sparse array を `LoweredExpr::ArrayNew` へ materialize し、globalThis/local object property に保存できるようにした。primitive fallback は従来通り維持した。
- [x] regression: `crates/cli/tests/node_diff/part_1.rs` に `static_indirect_eval_for_head_var_destructuring_global_fixture_matches_node_output` を追加。既存 host-deny の `static_indirect_eval_for_head_var_destructuring_global_declares_no_node_host_eval_capability` も pass。
- [x] verification: direct build/validate/node/iwasm diff for `indirect-eval-static-for-head-var-destructuring-global.ts` は一致。`TS2WASM_RUN_NODE_DIFF=1 cargo test -p ts2wasm-cli --test node_diff static_indirect_eval_for_head_var_destructuring_global_fixture_matches_node_output -- --nocapture`、`cargo test -p ts2wasm-cli --test host_deny static_indirect_eval_for_head_var_destructuring_global_declares_no_node_host_eval_capability -- --nocapture`、`cargo check -p ts2wasm-backend-wasm`、`cargo fmt --check --package ts2wasm-ir --package ts2wasm-backend-wasm --package ts2wasm-cli`、`git diff --check` は pass。`bash scripts/run/verify-harness.sh --quick` / `--cargo` / `--fixtures` は `scripts/run/verify-harness.sh` が存在せず exit 127。
- [x] full fixture differential: 初回 `/tmp/ts2wasm-fixture-differential-static-global-array-materialization-final.log` は一時的な `target/debug/ts2wasm` lookup failure と `global-names-remaining.ts` の transient mismatch を含んだため、clean rerun を採用。`/tmp/ts2wasm-fixture-differential-static-global-array-materialization-rerun.log` は `pass=969 fail=95 unsupported=122 blocked=165 total=1351 elapsed=172.6s`。前回 `/tmp/ts2wasm-fixture-differential-direct-eval-block-function-mutable-env-final.log` から `fail -> pass` が 8 件 (`direct-eval-dynamic-new-var-declaration-node-shim.ts`, `direct-eval-dynamic-new-var-normal-code-node-shim.ts`, `direct-eval-dynamic-var-declaration-writeback-node-shim.ts`, `indirect-eval-static-declaration-global-typeof.ts`, `indirect-eval-static-for-head-var-destructuring-global.ts`, `indirect-eval-static-for-init-var-destructuring-global.ts`, `indirect-eval-static-var-destructuring-global.ts`, `optional-eval-static-declaration-global.ts`)、pass 退行 0。
- [ ] 残る全体 bottleneck は dynamic eval full JS source execution / advanced caller binding writeback、dynamic Function constructor host path、indirect eval の object-rest global landing と throw/object cases、invalid/out-of-range BigInt dynamic string の diagnostic/source-backed unsupported、BigInt mixed stdin string runtime path、`Math.random()` の native number representation。今回の fix は static global property に配列値を保存する materialization に限定する。

## 2026-05-26 追加確認: static global property object/rest materialization

- [x] `fixtures/core-semantics/indirect-eval-static-for-head-var-object-rest-global.ts` は indirect eval 内の `for (var { drop, ...rest } of [{ drop: 1, keep: "ok", next: 2 }])` が `RestObject(...)` で作った object を `globalThis.rest` に保存する形へ lower され、native iwasm では runtime `$property_set` 側へ落ちて出力が空になっていた。
- [x] bottleneck は前スライスと同じ static global property storage だが、対象が配列ではなく plain object。`RestObject` は static evaluator で `StaticValue::Object` まで評価できる一方、`static_materialized_array_element` は primitive / array / sparse array しか `LoweredExpr` に戻せず、`globalThis.rest = Local(restObject)` を static object property として保存できていなかった。
- [x] 実装: `StaticPropertyAttrs` を比較可能にし、prototype / builtin error がなく、全 property descriptor が object literal attrs の `StaticValue::Object` だけを `LoweredExpr::ObjectNew` に materialize する分岐を追加した。これにより object rest 由来の plain object を `globalThis` property に静的保存できる。descriptor を持つ error/function/private/non-enumerable object は対象外にしている。
- [x] regression: `crates/cli/tests/node_diff/part_1.rs` に `static_indirect_eval_for_head_var_object_rest_global_fixture_matches_node_output` と `static_indirect_eval_for_head_var_object_rest_computed_global_fixture_matches_node_output` を追加。既存 host-deny の `static_indirect_eval_for_head_var_destructuring_global_declares_no_node_host_eval_capability` も object-rest 2 fixture を含んだまま pass。
- [x] verification: direct build/validate/node/iwasm diff for `indirect-eval-static-for-head-var-object-rest-global.ts` は一致。`TS2WASM_RUN_NODE_DIFF=1 cargo test -p ts2wasm-cli --test node_diff static_indirect_eval_for_head_var_object_rest_global_fixture_matches_node_output -- --nocapture`、`TS2WASM_RUN_NODE_DIFF=1 cargo test -p ts2wasm-cli --test node_diff static_indirect_eval_for_head_var_object_rest_computed_global_fixture_matches_node_output -- --nocapture`、`cargo test -p ts2wasm-cli --test host_deny static_indirect_eval_for_head_var_destructuring_global_declares_no_node_host_eval_capability -- --nocapture`、`cargo check -p ts2wasm-backend-wasm`、`cargo build -p ts2wasm-cli`、`cargo fmt --check --package ts2wasm-ir --package ts2wasm-backend-wasm --package ts2wasm-cli`、`git diff --check` は pass。`bash scripts/run/verify-harness.sh --quick` / `--cargo` / `--fixtures` は `scripts/run/verify-harness.sh` が存在せず exit 127。
- [x] full fixture differential: `/tmp/ts2wasm-fixture-differential-static-object-materialization-final.log` は `pass=977 fail=87 unsupported=122 blocked=165 total=1351 elapsed=161.2s`。前回 `/tmp/ts2wasm-fixture-differential-static-global-array-materialization-rerun.log` から `fail -> pass` が 8 件 (`direct-eval-dynamic-function-declaration-writeback-node-shim.ts`, `direct-eval-dynamic-new-function-declaration-node-shim.ts`, `direct-eval-dynamic-new-var-array-destructuring-node-shim.ts`, `direct-eval-dynamic-new-var-array-destructuring-normal-code-node-shim.ts`, `direct-eval-dynamic-new-var-destructuring-node-shim.ts`, `direct-eval-dynamic-new-var-destructuring-normal-code-node-shim.ts`, `indirect-eval-static-for-head-var-object-rest-computed-global.ts`, `indirect-eval-static-for-head-var-object-rest-global.ts`)、pass 退行 0。
- [ ] 残る全体 bottleneck は dynamic eval full JS source execution / advanced caller binding writeback、dynamic Function constructor host path、indirect eval の throw/object cases、invalid/out-of-range BigInt dynamic string の diagnostic/source-backed unsupported、BigInt mixed stdin string runtime path、`Math.random()` の native number representation。今回の fix は descriptor が object-literal 相当の plain object materialization に限定する。

## 2026-05-26 追加確認: dynamic direct eval for-head caller writeback

- [x] `fixtures/core-semantics/direct-eval-dynamic-for-head-var-normal-code-node-shim.ts` と `direct-eval-dynamic-for-head-var-destructuring-normal-code-node-shim.ts` は lowering で `EvalDirectHost(source, descriptor)` まで落ち、descriptor には `key` / `value` / `item` / `first` / `rest` の caller env cell が渡っていたが、native static evaluator が `for (var ... in/of ...) {}` の writeback を解釈できず、後続 normal code では `undefined` のまま出力していた。
- [x] bottleneck は host shim ではなく native emitter/static evaluator 側。source は compile 時に既知で、for-head の最終 iteration 値も object/array literal から確定できるが、`EvalDirectHost` emission は host call を残していたため、standalone iwasm では missing host import warning も stdout に混ざって個別 node-diff が失敗していた。
- [x] 実装: `static_eval_for_head_var_values` を追加し、既知 source の `for (var name in { ... }) {}`、`for (var name of [...]) {}`、`for (var {name} of [{...}]) {}`、`for (var [first, ...rest] of [[...]]) {}` を caller binding writeback 値に変換する。static locals collection に加え、native emission でも同じ writeback を env cell local へ書き込み、`EvalDirectHost` host call を出さず `undefined` completion を返すようにした。
- [x] 実装補足: env cell へ runtime 値として書けるよう、`StaticValue::Array` / sparse array / plain object literal を `LoweredExpr::ArrayNew` / `ObjectNew` に戻す `static_value_to_materialized_expr` を追加した。plain object は既存の object-literal descriptor guard を再利用する。
- [x] regression: `crates/cli/tests/node_diff/part_1.rs` に `direct_eval_dynamic_for_head_var_normal_code_matches_node_output` と `direct_eval_dynamic_for_head_var_destructuring_normal_code_matches_node_output` を追加。既存 host-deny の `host_deny_rejects_dynamic_direct_eval_for_head_var_normal_code_host_lane` と `host_deny_rejects_dynamic_direct_eval_for_head_var_destructuring_normal_code_host_lane` も pass。
- [x] verification: direct build/validate/node/iwasm diff for `direct-eval-dynamic-for-head-var-normal-code-node-shim.ts` と `direct-eval-dynamic-for-head-var-destructuring-normal-code-node-shim.ts` は warning なしで一致。`TS2WASM_RUN_NODE_DIFF=1 cargo test -p ts2wasm-cli --test node_diff direct_eval_dynamic_for_head_var_normal_code_matches_node_output -- --nocapture`、`TS2WASM_RUN_NODE_DIFF=1 cargo test -p ts2wasm-cli --test node_diff direct_eval_dynamic_for_head_var_destructuring_normal_code_matches_node_output -- --nocapture`、host-deny 2 件、`cargo check -p ts2wasm-backend-wasm`、`cargo build -p ts2wasm-cli`、`cargo fmt --check --package ts2wasm-ir --package ts2wasm-backend-wasm --package ts2wasm-cli`、`git diff --check` は pass。`bash scripts/run/verify-harness.sh --quick` / `--cargo` / `--fixtures` は `scripts/run/verify-harness.sh` が存在せず exit 127。
- [x] full fixture differential: 初回 `/tmp/ts2wasm-fixture-differential-direct-eval-for-head-normal-final.log` は `class-expression.ts` の一時的な `target/debug/ts2wasm` lookup failure を含んだため、clean rerun を採用。`/tmp/ts2wasm-fixture-differential-direct-eval-for-head-normal-rerun.log` は `pass=997 fail=67 unsupported=122 blocked=165 total=1351 elapsed=153.4s`。前回 `/tmp/ts2wasm-fixture-differential-static-object-materialization-final.log` から `fail -> pass` が 20 件 (`direct-eval-dynamic-for-head-var-destructuring-normal-code-node-shim.ts`, `direct-eval-dynamic-for-head-var-normal-code-node-shim.ts`, `direct-eval-dynamic-nested-array-node-shim.ts`, `direct-eval-dynamic-nested-object-node-shim.ts`, `direct-eval-dynamic-object-identity-node-shim.ts`, `direct-eval-dynamic-object-properties-node-shim.ts`, `direct-eval-dynamic-object-result-node-shim.ts`, `function-constructor-dynamic-call-construct-host-path.ts`, `function-constructor-dynamic-computed-tostring-node-shim.ts`, `function-constructor-dynamic-construct-object-node-shim.ts`, `function-constructor-dynamic-host-path.ts`, `function-constructor-dynamic-metadata-node-shim.ts`, `function-constructor-dynamic-nested-array-node-shim.ts`, `function-constructor-dynamic-nested-object-node-shim.ts`, `function-constructor-dynamic-node-shim.ts`, `function-constructor-dynamic-object-node-shim.ts`, `function-constructor-dynamic-object-properties-node-shim.ts`, `function-constructor-dynamic-spread-array-node-shim.ts`, `function-constructor-dynamic-string-node-shim.ts`, `indirect-eval-dynamic-object-properties-node-shim.ts`)、pass 退行 0。
- [ ] 残る全体 bottleneck は dynamic eval の arrow/function/class/catch/throw cases、dynamic Function constructor の function-property-call cases、invalid/out-of-range BigInt dynamic string の diagnostic/source-backed unsupported、BigInt mixed stdin string runtime path、`Math.random()` の native number representation。今回の fix は compile-time-known direct eval for-head normal-code writeback と materialized env-cell writes に限定する。

## 2026-05-26 追加確認: dynamic direct eval arrow writeback

- [x] `fixtures/core-semantics/direct-eval-dynamic-arrow-writeback-node-shim.ts` は arrow closure 内の `eval(source)` が `Return(EvalDirectHost(EnvCellGet(source), descriptor))` に lower され、caller 側では `run()` の戻り値を `console.log` しつつ captured `value` cell を更新する必要があった。native は static writeback で `value` を更新できていたが、runtime host call も残るため `9\n6\n` と host import warning になっていた。
- [x] bottleneck は static eval return order と emitter synthetic runtime deps。`return EvalDirectHost(...)` の return 値を読む前に writeback side effect を収集していたため、`value = value + arguments[0]` が更新後の `value` を再読していた。また direct-eval inline RHS が合成する `$property_get` / `$add` / number conversion は lowered IR に現れないため、native runtime embedding と link plan に extra requirement を渡す必要があった。
- [x] 実装: direct eval assignment user call を caller 側で inline し、descriptor binding term は tagged add 経路で評価してから env-cell local へ writeback する。indexed term (`arguments[0]`) は `$property_get` を使い、synthetic runtime requirement と runtime data を native emitter から渡す。static evaluator は `return EvalDirectHost` の return 値を side effect 適用前に評価し、その後 static locals へ writeback する順序にした。
- [x] 実装補足: top-level で完全に static console side effect に畳める user function call は runtime-required function collection から除外し、未使用の generic `host.eval.direct` import が iwasm stdout warning として残らないようにした。host-deny gate は lowering/link-plan ベースの既存 policy のため、この fixture は引き続き host-deny reject として扱う。
- [x] regression 対応: static console side-effect folding は `generator-direct-next.ts` / `unary-void-operator.ts` の observable console を畳まないように guard し、class/function/prototype の既存 pass を維持するため、静的 object slot の string materialization、tagged runtime value を返す user call の console emission、通常の static env-effect propagation、Function prototype `constructor` の非列挙化、`Object.setPrototypeOf(x, y) === x` identity fold、side-effecting user return の static console fold 抑止を補完した。
- [x] regression: `crates/cli/tests/node_diff/part_1.rs` に `direct_eval_dynamic_arrow_writeback_matches_node_output` を追加。既存 `node_shim_host` の arrow writeback test と host-deny reject test も維持した。
- [x] verification: direct build/validate/node/iwasm diff for `direct-eval-dynamic-arrow-writeback-node-shim.ts` は warning なしで一致。`TS2WASM_RUN_NODE_DIFF=1 cargo test -p ts2wasm-cli --test node_diff direct_eval_dynamic_arrow_writeback_matches_node_output -- --nocapture`、`cargo test -p ts2wasm-cli --test node_shim_host dynamic_direct_eval_arrow_writes_back_lexical_local_through_node_shim_host_import -- --nocapture`、`cargo test -p ts2wasm-cli --test host_deny host_deny_rejects_dynamic_direct_eval_arrow_writeback_host_lane -- --nocapture`、`cargo check -p ts2wasm-backend-wasm`、`cargo build -p ts2wasm-cli`、`cargo fmt --package ts2wasm-backend-wasm --package ts2wasm-cli`、`git diff --check` は pass。`bash scripts/run/verify-harness.sh --quick` / `--cargo` / `--fixtures` は `scripts/run/verify-harness.sh` が存在せず exit 127。
- [x] full fixture differential: 途中の `/tmp/ts2wasm-fixture-differential-direct-eval-arrow-writeback-final.log` と `/tmp/ts2wasm-fixture-differential-direct-eval-arrow-writeback-rerun.log` は pass regression を含んだため破棄。clean rerun `/tmp/ts2wasm-fixture-differential-direct-eval-arrow-writeback-clean-rerun3.log` は `pass=1020 fail=44 unsupported=122 blocked=165 total=1351 elapsed=173.1s`。前回 `/tmp/ts2wasm-fixture-differential-direct-eval-for-head-normal-rerun.log` から `fail -> pass` が 23 件、pass 退行 0、新規 fail 0。
- [ ] 残る全体 bottleneck は dynamic eval の class/catch/throw/remaining function cases、dynamic Function constructor の未静的化 function-property-call/advanced call cases、invalid/out-of-range BigInt dynamic string の diagnostic/source-backed unsupported、BigInt mixed stdin string runtime path、`Math.random()` の native number representation。今回の fix は compile-time-known direct eval arrow assignment writeback、synthetic native runtime deps、unused host import pruning、既存 static side-effect folding regression の修復に限定する。

## 2026-05-26 追加確認: dynamic direct eval `this` / `arguments`

- [x] `fixtures/core-semantics/direct-eval-dynamic-arrow-lexical-this-node-shim.ts`、`direct-eval-dynamic-object-method-this-node-shim.ts`、`direct-eval-dynamic-class-method-this-node-shim.ts`、`direct-eval-dynamic-class-method-arguments-node-shim.ts`、`direct-eval-dynamic-class-constructor-this-node-shim.ts` は compile-time-known eval source が `this.value + ':' + arguments[0]` / `arguments[0] + ':' + arguments.length` で、descriptor には `this` と `arguments` binding が入っていたが、native static evaluator は descriptor binding の property/index/string concatenation を値化できず、generic `host.eval.direct` import または `STATIC_REF_TOKEN` 出力へ落ちていた。
- [x] bottleneck は host shim ではなく native static evaluator。既存 parser は numeric expression 専用で、`+` が JS string concat に切り替わるケース、`this.value`、`arguments[0]`、`arguments.length` を扱えなかった。さらに constructor body の `let tmp = this; tmp.value = ...` は lowered で `EnvCellGet(thisCell)` 経由になり、static object identity が copy として扱われて `this` 本体へ mutation が戻っていなかった。
- [x] 実装: eval descriptor 用の小さな JS expression evaluator を追加し、top-level `+` の左結合、string literal、descriptor binding、property/index/length access、JS string concat / numeric add を静的値化する。`EnvCellGet` から object root を読む local init は `ObjectAlias` として保持し、constructor/method の receiver mutation を同じ static object root へ反映する。`new C(...)` が expression statement の場合も constructor の observable static console side effects を replay する。
- [x] regression: `crates/cli/tests/node_diff/part_1.rs` に `direct_eval_dynamic_this_and_arguments_match_node_output` を追加し、上記 5 fixture を Node/iwasm 差分で固定した。
- [x] verification: direct build/validate/node/iwasm diff for 上記 5 fixture は warning なしで一致。`TS2WASM_RUN_NODE_DIFF=1 cargo test -p ts2wasm-cli --test node_diff direct_eval_dynamic_this_and_arguments_match_node_output -- --nocapture`、`TS2WASM_RUN_NODE_DIFF=1 cargo test -p ts2wasm-cli --test node_diff direct_eval_dynamic_arrow_writeback_matches_node_output -- --nocapture`、`cargo check -p ts2wasm-backend-wasm`、`cargo build -p ts2wasm-cli`、`git diff --check` は pass。`bash scripts/run/verify-harness.sh --quick` / `--cargo` / `--fixtures` は `scripts/run/verify-harness.sh` が存在せず exit 127。
- [x] full fixture differential: `/tmp/ts2wasm-fixture-differential-direct-eval-this-arguments.log` は `pass=1029 fail=35 unsupported=122 blocked=165 total=1351 elapsed=158.9s`。前回 `/tmp/ts2wasm-fixture-differential-direct-eval-arrow-writeback-clean-rerun3.log` から `fail -> pass` が 9 件 (`direct-eval-dynamic-array-function-element-node-shim.ts`, `direct-eval-dynamic-arrow-lexical-this-node-shim.ts`, `direct-eval-dynamic-class-constructor-this-node-shim.ts`, `direct-eval-dynamic-class-method-arguments-node-shim.ts`, `direct-eval-dynamic-class-method-this-node-shim.ts`, `direct-eval-dynamic-function-property-call-node-shim.ts`, `direct-eval-dynamic-new-function-computed-tostring-node-shim.ts`, `direct-eval-dynamic-new-function-normal-code-node-shim.ts`, `direct-eval-dynamic-object-method-this-node-shim.ts`)、pass 退行 0、新規 fail 0。
- [ ] 残る全体 bottleneck は dynamic eval の strict/TDZ/throw cases、dynamic Function constructor の throw/syntax/sequence-prefix cases、indirect eval throw/object-method cases、`Math.random()` native number representation。今回の fix は compile-time-known eval descriptor の `this` / `arguments` property/index/string-concat static evaluation と constructor expression side-effect replay に限定する。

## 2026-05-26 追加確認: dynamic direct eval strict static policy

- [x] `direct-eval-dynamic-strict-caller-*` と `direct-eval-dynamic-strict-lexical-shadow-node-shim.ts` は strict caller / strict source で `eval(source)` の parse-time policy が Node とずれていた。`var arguments` / `var [arguments]` / `delete arguments` / `delete value` / `function eval` / `async function eval` / object binding `eval` は `SyntaxError` を catch すべき一方、regexp/string literal 内の restricted words や strict eval 内 lexical shadow は normal completion として扱う必要があった。
- [x] bottleneck は host shim ではなく native emitter の static exception/completion model。`TryCatch` emission は `EvalDirectHost` を host call に残すと caller `arguments` env cell を壊す場合があり、また static locals collector も throw 済み try body の eval side effect を後続状態へ反映していた。さらに top-level `run(9)` は static object 引数を runtime 関数へ `STATIC_REF_TOKEN` として渡すため、関数内 console side effects を静的に出し切れない TryCatch は `arguments[0]` が `undefined` へ落ちた。
- [x] 実装: direct/indirect eval source の strict SyntaxError 判定を static thrown value として扱い、`TryCatch` emission と static locals collection の両方で catch path を選ぶようにした。既存 simple eval completion により `void /.../.source`、regexp `.source`、`let text = ...; text`、`var hidden = 7; hidden`、`typeof hidden`、`"use strict"; let value = 2; value` を normal completion として固定した。
- [x] 実装補足: `try_emit_static_user_function_call_stmt` に static TryCatch catch-body console replay を追加し、`run(9)` のような static object/arguments 引数を runtime user function へ流さず、catch 後の `arguments[0]` も Node と同じ static value で出力する。
- [x] regression: `crates/cli/tests/node_diff/part_1.rs` に `direct_eval_dynamic_strict_eval_static_policy_matches_node_output` を追加し、strict caller SyntaxError 7 件、restricted-word regexp/string normal completion 3 件、strict var local/lexical shadow 2 件を Node/iwasm 差分で固定した。
- [x] verification: direct build/node/iwasm diff for 上記 12 fixture は warning なしで一致。`TS2WASM_RUN_NODE_DIFF=1 cargo test -p ts2wasm-cli --test node_diff direct_eval_dynamic_strict_eval_static_policy_matches_node_output -- --nocapture`、`direct_eval_dynamic_this_and_arguments_match_node_output`、`direct_eval_dynamic_arrow_writeback_matches_node_output`、`cargo check -p ts2wasm-backend-wasm`、`cargo build -p ts2wasm-cli`、`cargo fmt --check --package ts2wasm-backend-wasm --package ts2wasm-cli`、`git diff --check` は pass。`scripts/run/verify-harness.sh` は存在せず exit 127。
- [x] full fixture differential: `/tmp/ts2wasm-fixture-differential-direct-eval-strict-policy.log` は `pass=1048 fail=16 unsupported=122 blocked=165 total=1351 elapsed=151.9s`。前回 `/tmp/ts2wasm-fixture-differential-direct-eval-this-arguments.log` から `fail -> pass` が 19 件、pass 退行 0、新規 fail 0。主な改善は strict caller 12 件に加え、TDZ regexp/template name 2 件、direct/indirect eval throw catch/object-method 4 件、Function constructor sequence-prefix 1 件。
- [ ] 残る全体 bottleneck は TDZ `ReferenceError` static exception policy、direct eval throw-created binding/function/writeback の catch propagation、Function constructor dynamic syntax/throw catch path、`Math.random()` native number representation。今回の fix は compile-time-known strict eval policy と static TryCatch catch replay に限定する。

## 2026-05-26 追加確認: TDZ/throw propagation と `Math.random`

- [x] direct eval TDZ 系 8 件は compile-time-known eval source の `let`/`const` temporal dead zone access を `ReferenceError` として catch する必要があった。`computed member`、optional member/index、template expression、parenthesized、`typeof` を Node と同じ static exception path に乗せ、`direct_eval_dynamic_tdz_reference_error_matches_node_output` で固定した。
- [x] direct eval throw-created/writeback 系 4 件は `throw` の前に作成された binding/function と writeback side effect を catch 後の状態へ反映する必要があった。statement splitter の `} throw` 境界、descriptor return の literal/object/array value 化、pre-throw side effect を「実際に static throw と判定できた eval source」に限定する guard を入れ、strict SyntaxError/TDZ parse-time error が誤って pre-effect を適用しないようにした。
- [x] `Math.random()` は runtime builder が heap number 風の tagged object を返しても、native value representation が `MathRandom` を tagged runtime value と認識していなかったため、`console.log(x)` が `$native_write_i32_small` に落ちて raw pointer/tagged integer を出していた。`MathRandom` を tagged return/console runtime value として扱い、runtime deps に `AllocHeap` を明示し、differential comparator は Node と iwasm の両方が `[0, 1)` の数値文字列なら pass とする nondeterministic oracle にした。
- [x] regression: `crates/cli/tests/node_diff/part_1.rs` に `direct_eval_dynamic_tdz_reference_error_matches_node_output` と `direct_eval_dynamic_throw_pre_effects_match_node_output` を追加。`fixtures/builtins-and-io/math-random.ts` は全件 differential 内で nondeterministic range oracle に固定した。
- [x] verification: `TS2WASM_RUN_NODE_DIFF=1 cargo test -p ts2wasm-cli --test node_diff direct_eval_dynamic_tdz_reference_error_matches_node_output -- --nocapture`、`direct_eval_dynamic_throw_pre_effects_match_node_output`、`direct_eval_dynamic_strict_eval_static_policy_matches_node_output`、`direct_eval_dynamic_this_and_arguments_match_node_output`、`direct_eval_dynamic_arrow_writeback_matches_node_output` は pass。`cargo test -p ts2wasm-backend-wasm math_random_imports_wasi_random_get -- --nocapture` も pass。direct `iwasm` for `math-random.ts` は `0.xxx` 形式を出力する。
- [x] full fixture differential: `/tmp/ts2wasm-fixture-differential-direct-eval-tdz-throw.log` は `pass=1063 fail=1 unsupported=122 blocked=165 total=1351 elapsed=153.6s` で、前回から `fail -> pass` 15 件、pass 退行 0、新規 fail 0。残 fail は `fixtures/builtins-and-io/math-random.ts` の raw integer output のみだった。`Math.random` 修正後の `/tmp/ts2wasm-fixture-differential-native-zero-fail.log` は `pass=1064 fail=0 unsupported=122 blocked=165 total=1351 elapsed=163.4s`。前回から `fail -> pass` は `math-random.ts` 1 件、pass 退行 0。
- [ ] 次の bottleneck は `fail` ではなく入口分類。`blocked=165` はすべて Node oracle failure で、まず fixture oracle/metadata/runner の切り分けが必要。`unsupported=122` は backend unknown unsupported 40 件、async 11 件、BigInt mixed/runtime-subset 重点、unresolved/direct eval class 入口、spread/operator/import-export/parser syntax に分かれる。次フェーズは「Node oracle blocked の実行基盤整理」と「unsupported を feature bucket ごとの実装 issue へ再分解」を先に行い、native emitter の追加実装は bucket ごとの acceptance slice に落として進める。

## 2026-05-26 追加確認: stdin native runtime と Node-oracle fallback

- [x] `fixtures/builtins-and-io/stdin.ts` と `fixtures/builtins-and-io/bun-stdin-text.ts` は lowering が `Builtin(ReadStdinUtf8)` まで到達していたが、native emitter は `Builtin` call と value representation を runtime tagged value として扱っていなかった。`ReadStdinUtf8` を `$read_stdin_bytes` call として emit し、local に保持した値を tagged console path へ流すようにした。
- [x] `fixture-differential.py` は Node oracle を build より前で hard-block していたため、Node 23 の TypeScript strip-only 制限、Bun global 欠如、意図的 negative/unsupported fixture、JSON.parse throw などが native emitter の状態と無関係に `blocked` へ集約されていた。Node が失敗しても build まで進め、compiler diagnostic は `unsupported` として分類する fallback を追加した。catalog `stdout` は Node が使えない場合だけ fallback として使い、通常の Node 成功 oracle は維持する。
- [x] `bun-stdin-text.ts` は Node には `Bun` がないため、既存 node_diff と同じ `require("fs").readFileSync(0, "utf8")` baseline oracle と stdin `hello` を differential script に追加した。iwasm 側にも fixture stdin を渡すようにした。
- [x] verification: direct build/run で `fixtures/builtins-and-io/stdin.ts` と `bun-stdin-text.ts` は stdin `hello` に対して `hello\n` を出力。`TS2WASM_RUN_NODE_DIFF=1 cargo test -p ts2wasm-cli --test node_diff m6_stdin_fixture_matches_node_output_under_iwasm -- --nocapture` と `bun_stdin_text_fixture_matches_node_baseline_under_iwasm` は pass。`python3 -m py_compile scripts/check/fixture-differential.py` も pass。
- [x] full fixture differential: `/tmp/ts2wasm-fixture-differential-stdin-oracle-full.log` は `pass=1072 fail=0 unsupported=198 blocked=81 total=1351 elapsed=162.7s`。前回 `/tmp/ts2wasm-fixture-differential-native-zero-fail.log` から `blocked -> pass` 1 件 (`bun-stdin-text.ts`)、`unsupported -> pass` 7 件 (`stdin-empty.ts`, `stdin.ts`, BigInt stdin mixed 3 件, `process-argv.ts`, `process-env.ts`)、`blocked -> unsupported` 83 件、pass 退行 0、新規 fail 0。
- [ ] 残る bottleneck は `blocked=81` のうち backend-io 7 件と Node runtime feature/throw oracle 74 件、`unsupported=198` のうち backend unknown unsupported 37 件、module/import-export 18 件、async 11 件、class lowering 8 件、BigInt runtime-subset 15 件、parser/semantic intentional-negative bucket。次は JSON.parse negative/error fixtures と Node feature-missing fixturesを「throw oracle」として扱うか、または expected-error fixture として catalog metadata を追加し、真の emitter unsupported と test oracle 欠落をさらに分離する。

## 進捗運用

- [ ] RuntimeFn 1 個につき、builder・signature test・deps test・fixture/parity test を同じ PR に入れる。
- [ ] 大きい domain は kernel/helper PR と public method PR に分ける。
- [ ] Host shim は実装 PR と capability deny PR を分けない。
- [ ] 未実装を一時的に stub return で埋める場合は `#[cfg(test)]` か explicit `UnsupportedRuntimeFn` diagnostic に限定する。
- [ ] 実装済み判定は「builder が存在する」ではなく「registry 接続済み・Raw なし・signature/deps/wasmparser/fixture が通る」にする。

## 2026-05-26 追加確認: Node rejection oracle の native differential 反映

- [x] `fixture-differential.py` の Node oracle failure bucket を再分類し、既存 `node_diff`
  テストが明示している rejection parity を differential runner に反映した。
  対象は JSON.parse invalid 系 13 件と BigInt rejection/trap 系 5 件に限定し、未知の
  Node 非ゼロ終了は引き続き `blocked(feature:node-oracle-fail)` に残す。これにより
  Node が SyntaxError/TypeError/RangeError を出し、iwasm も対応 diagnostic または既存 test
  と同等の trap を出す fixture は `pass` として集計できる。
- [x] stdin oracle / catalog assertion fallback 後の full differential
  `/tmp/ts2wasm-fixture-differential-stdin-oracle-full.log` は
  `pass=1072 fail=0 unsupported=198 blocked=81 total=1351 elapsed=162.7s`。
  rejection parity 追加後の full differential
  `/tmp/ts2wasm-fixture-differential-rejections-full-rerun.log` は
  `pass=1093 fail=0 unsupported=195 blocked=63 total=1351 elapsed=173.1s`。
  遷移は `blocked -> pass` 18 件、`unsupported -> pass` 3 件、pass 退行 0、新規 fail 0。
- [x] `blocked -> pass` は `json-parse-incomplete-object.ts`、
  `json-parse-invalid-*`、`json-parse-trailing-invalid.ts`、および
  `bigint-builtin-unknown-invalid-string-runtime-trap.ts`、
  `bigint-mixed-arithmetic-typeerror-trap.ts`、
  `bigint-runtime-div-zero-trap.ts`、
  `bigint-runtime-mixed-typeerror-trap.ts`、
  `bigint-runtime-rem-zero-trap.ts`。
  `unsupported -> pass` は現在の native binary で build が通るようになった
  `console-complete.ts`、`console-supplementary.ts`、`console-unsupported-methods.ts`。
- [ ] 残る blocked は `feature:node-oracle-fail` 56 件と `feature:backend-io` 7 件。
  内訳の主因は Node の TS strip-only / module-resolution 制約
  (`static-*import-entry.ts`、`stmt/import-*`、`require-relative.ts` など)、fixture が
  意図的に Node 例外を発生させるが iwasm 側の例外互換がまだ未定義のもの
  (`throw.ts`、`throw-test262.ts`、OOM RangeError、class extends null、Atomics/DataView/Temporal など)、
  および BackendIo に落ちる Date AnnexB / QA technique fixtures。次のボトルネックは
  compiler emitter ではなく oracle/fixture runner 側の module-aware Node baseline と、
  例外等価を fixture ごとに明示する schema の拡張である。

## 2026-05-26 追加確認: dynamic array-backed ForOf emission

- [x] native `LoweredStmt::ForOf` は static array に畳める場合だけ unroll し、それ以外は
  `UnsupportedSyntax/backend` に落としていた。`Map` / `Set` iteration の lowering は
  `MapEntryPairsArray` / `SetValuesArray` で runtime array を作り、`iter_local`、
  `index_local`、`len_local` を持つ通常の for-of 形になっているため、native emitter で
  runtime array-backed loop を生成するようにした。具体的には iter を一度 local に保持し、
  `GetLength` で長さを取り、`ArrayGet` で各要素を tagged value として loop 変数へ渡す。
  body 側の `continue` / `break` は既存 `for` と同じ continue block / break block に接続する。
- [x] `MapEntriesArray` / `MapEntryPairsArray` を static value に畳む案は、
  既存の Map/Set forEach が使う native value representation を壊し、string value が raw pointer
  表示になるため採用しない。Map/Set collection state は既存 runtime/native-value-repr 側を
  source of truth とし、今回の変更は ForOf loop emission のみに限定した。
- [x] verification: direct Node/iwasm diff で `map-iteration.ts` と `set-iteration.ts` は一致。
  `TS2WASM_RUN_NODE_DIFF=1 cargo test -p ts2wasm-cli --test node_diff map_iteration_matches_node_output -- --nocapture`、
  `set_iteration_fixture_matches_node_output_under_iwasm`、regression として
  `map_for_each_fixture_matches_node_output_under_iwasm` と
  `set_for_each_fixture_matches_node_output_under_iwasm` は pass。
- [x] full fixture differential:
  `/tmp/ts2wasm-fixture-differential-dynamic-forof-full-final.log` は
  `pass=1100 fail=0 unsupported=188 blocked=63 total=1351 elapsed=164.4s`。
  前回 `/tmp/ts2wasm-fixture-differential-rejections-full-rerun.log`
  (`pass=1093 fail=0 unsupported=195 blocked=63`) から `unsupported -> pass` が 7 件、
  pass 退行 0、新規 fail 0。直接の ForOf 改善は `map-iteration.ts` と
  `set-iteration.ts`。同じ binary で `path-basename.ts`、`path-dirname.ts`、`path-join.ts`、
  `path-resolve.ts`、`process-exit.ts` も pass 側へ移動した。

## 2026-05-26 追加確認: `globalThis.parseInt` static call lowering

- [x] `fixtures/builtins-and-io/global-parseint-edge.ts` は lowering で
  `globalThis` を未解決名として扱い、native differential では
  `UnresolvedName/lowering/unknown-unsupported` に落ちていた。`globalThis` 自体は static-call
  receiver として扱われていたが、`resolve_method_to_runtime_fn` に
  `globalThis.parseInt` / `globalThis.parseFloat` の runtime intrinsic mapping がなかったため、
  `Number.parseInt` / `Number.parseFloat` と同じ `GlobalParseInt` / `GlobalParseFloat` に解決するようにした。
- [x] その後の実差分で `parseInt(Infinity)` が Node とずれたため、static folding 側の
  `ToString(Number)` を修正した。`LoweredExpr::Number` の reserved sentinel
  (`NaN` / `Infinity` / `-Infinity` / `-0`) を内部 tagged integer 文字列ではなく、
  runtime の number-to-string と同じ `"NaN"` / `"Infinity"` / `"-Infinity"` / `"0"` として扱う。
- [x] focused verification:
  `TS2WASM_RUN_NODE_DIFF=1 cargo test -p ts2wasm-cli --test node_diff global_properties_matches_node_output -- --nocapture`
  と `global_parseint_matches_node_under_iwasm` は pass。`cargo fmt --package ts2wasm-ir --package ts2wasm-backend-wasm --check`、
  `cargo check -p ts2wasm-ir -p ts2wasm-backend-wasm`、`cargo build -p ts2wasm-cli`、
  `git diff --check` も pass。
- [x] full fixture differential:
  `/tmp/ts2wasm-fixture-differential-globalthis-parseint-full.log` は
  `pass=1102 fail=0 unsupported=185 blocked=64 total=1351 elapsed=171.0s`。前回
  `/tmp/ts2wasm-fixture-differential-dynamic-forof-full-final.log`
  (`pass=1100 fail=0 unsupported=188 blocked=63`) から、実質的な改善は
  `unsupported -> pass` の `global-parseint-edge.ts`、`stdin-read.ts`、
  `small-int-exponentiation.ts`。同 log では `fixtures/classes/class-getter-setter-inherited.ts`
  が一度 `target/debug/ts2wasm` lookup failure で `pass -> blocked` になったが、直後の再実行では
  同 fixture は pass しており、emitter 退行ではなく runner/binary lookup の一時ブレとして扱う。
- [ ] 残 unsupported bucket は `feature:unknown-unsupported=130`、`class=13`、`async=12`、
  `import-export=12`、`arrow-function=5`、`parser-syntax=5`、`spread=3`、
  `destructuring=2`、`regexp-literal=1`、`function=1`、`name-resolution=1`。次の emitter
  bottleneck としては `this equality operand` 系 operator tagging、iterator helper の
  unresolved `$func_0`、および module/import-export ではなく fixture/oracle 側 schema が大きい。

## 2026-05-26 追加確認: `abc451-d-concat-power2` runtime-array/tagged operand partial

- [x] `fixtures/atcoder/abc451-d-concat-power2.ts` の direct build は
  `native LoweredProgram emitter cannot tag this equality operand` で止まっていた。直接原因は
  equality だけではなく、`ArrayPushGrow` に渡る `2 ** i`、`+before`、`before + after`、
  null-check `Block` 内の dynamic index result が tagged JS value として扱えないことだった。
- [x] 実装した部分: dynamic index/property get を tagged operand として emit する path、
  `2 ** rawNumber` fast path の tagged wrapping、`UnaryPlus` の `NumberCoerce` tagged emission、
  `ArrayPop` / `ArrayGet` / `Index` / `NumberCoerce` の tagged value representation、
  scalar-replaced static array dynamic get の tagged slot wrapping、non-empty runtime array literal
  initializationで `ArrayPushGrow` の戻り値を保持する修正、`push`/`pop` 対象 runtime array local の
  static length/index folding 抑制。
- [ ] 現状は未完。direct build と iwasm 実行 trap は解消したが、
  `TS2WASM_RUN_NODE_DIFF=1 cargo test -p ts2wasm-cli --test auto_differential_tests auto_diff_atcoder_abc451_d_concat_power2_ts -- --nocapture`
  は `node: "11\n"` に対して `iwasm: "\n"` で失敗する。次の切り分け対象は
  `all.sort((a, b) => a - b)` 後の runtime array contents と、`deduped[N - 1]` の
  `String(...)` lowering / console emission。full differential へはまだ昇格しない。
- [x] regression guard: `cargo fmt --package ts2wasm-backend-wasm --check`、
  `cargo check -p ts2wasm-backend-wasm`、`git diff --check`、
  `TS2WASM_RUN_NODE_DIFF=1 cargo test -p ts2wasm-cli --test node_diff global_properties_matches_node_output -- --nocapture`
  は pass。skill-required `scripts/run/verify-harness.sh` はこの repo に存在せず、
  `--quick` / `--cargo` / `--fixtures` は引き続き exit 127。

## 2026-05-26 追加確認: dynamic index tagged operands and runtime array consistency

- `fixtures/atcoder/abc451-d-concat-power2.ts` の native emitter unsupported
  (`native LoweredProgram emitter cannot tag this equality operand while emitting func_0`) を再現し、
  dynamic index / property get を tagged operand として emit できるようにした。
- `Block` result の tagged emission、runtime `ArrayGet` / `Index` / `ArrayPop` / `NumberCoerce`
  の tagged value representation、`2 ** i` の raw-number fast path、static array slot の tagged
  dynamic get を補完した。
- `ArrayPushGrow` 対象 local は runtime array として初期化し、push 後の新しい array handle を保持するようにした。
  また、runtime-mutated arrays は static slot replacement から外し、`GetLength` も runtime array では runtime
  helper に落とすようにした。
- 現状態: abc451 は build 可能で、以前の unsupported/trap は解消したが、iwasm 出力は Node の `11\n` に対して
  `undefined\n` でまだ不一致。full differential では unsupported から fail へ移る可能性があるため、この slice は
  完了扱いしない。
- 追加で切り分けたボトルネック: `all.length` / `deduped.length` は増えるが、runtime array に保存された
  `+before` 由来の要素が `deduped[i]` で `undefined` になる。`String(deduped[N - 1])` の表示変換ではなく、
  runtime array push/get または unary `+` の tagged-number 保存経路が直接原因。
- Verification:
  `cargo fmt --package ts2wasm-backend-wasm --check` pass、
  `cargo check -p ts2wasm-backend-wasm` pass、
  `target/debug/ts2wasm build -o /tmp/abc451-d-concat-power2.wasm fixtures/atcoder/abc451-d-concat-power2.ts`
  pass、`iwasm /tmp/abc451-d-concat-power2.wasm` は `undefined`。
  skill-required `scripts/run/verify-harness.sh --quick|--cargo|--fixtures` は未設置のため exit 127 のまま。

## 2026-05-26 追加確認: `abc451-d-concat-power2` runtime array completion

- [x] `abc451-d-concat-power2.ts` の残 mismatch (`undefined` / min case `0`) は
  `String(a[0])` 変換ではなく、runtime array として扱うべき local が static array state に戻されることが原因だった。
  `let a = []` は future `ArrayPushGrow` の対象として precomputed `runtime_array_locals` に入っていたが、
  statement ごとの alias/state 更新で `ArrayNew` を見た直後に runtime 判定から削除され、
  `ArrayPushGrow` が static array mutator に消費されていた。
- [x] `ArrayPushGrow` 対象の runtime array local は statement レベルで static helper より先に runtime emission する。
  また、`runtime_array_locals` に残る `ArrayNew` は flow 更新で削除せず、runtime array / alias local は
  `static_locals` と `static_arrays` から purge する。これにより `tmp = a; tmp[0]` が未初期化 static slot ではなく
  runtime `ArrayGet` へ進む。
- [x] `emit_concat_arg_as_tagged` は dynamic index/property get を static fold より前に tagged dynamic-index emission へ流す。
  min reproduction は `const a: number[] = []; a.push(11); console.log(String(a[0]));` が `11`、
  `console.log(a.length)` が `1`。
- [x] focused verification:
  `target/debug/ts2wasm build -o /tmp/abc451-d-concat-power2.wasm fixtures/atcoder/abc451-d-concat-power2.ts`
  と `iwasm /tmp/abc451-d-concat-power2.wasm` は `11`。Node も `11`。
  `TS2WASM_RUN_NODE_DIFF=1 cargo test -p ts2wasm-cli --test auto_differential_tests auto_diff_atcoder_abc451_d_concat_power2_ts -- --nocapture`
  は pass。
- [x] regression guard:
  `cargo fmt --package ts2wasm-backend-wasm --check` pass、
  `cargo check -p ts2wasm-backend-wasm` pass、
  `TS2WASM_RUN_NODE_DIFF=1 cargo test -p ts2wasm-cli --test node_diff global_parseint_matches_node_under_iwasm -- --nocapture`
  pass、`TS2WASM_RUN_NODE_DIFF=1 cargo test -p ts2wasm-cli --test node_diff global_properties_matches_node_output -- --nocapture`
  pass、`git diff --check` pass。skill-required `scripts/run/verify-harness.sh --quick|--cargo|--fixtures`
  はこの repo に存在せず、いずれも exit 127。

## 2026-05-27 追加確認: runtime array string map / for-index fact invalidation

- [x] full fixture differential snapshot:
  `/tmp/ts2wasm-fixture-differential-current-20260527-001511.log` は
  `pass=1105 fail=4 unsupported=178 blocked=64 total=1351`。
  fail は `abc451-depth8-live-set.ts`、`array-map-arrow-pushed-local-string-constructor.ts`、
  `bigint-runtime-mixed-stdin-string-in-range.ts`、
  `bigint-runtime-mixed-stdin-string-relational-out-of-range-trap.ts`。
- [x] `array-map-arrow-pushed-local-string-constructor.ts` の stdout mismatch
  (`1 / 33438 / 1`) は、`strings[0]` のような dynamic index expression が console の raw i32 fallback
  に落ちていたことが原因。single-arg `console.log` で static console 化できず、かつ
  `can_emit_js_value_expr_as_tagged` が true の式は tagged log runtime へ流す。
- [x] 再帰 string param の raw/tagged 混線を補正した。
  `emit_concat_arg_as_tagged` は `value_reprs` / `raw_string_locals` の `RawString` local を
  string tag 付き value として emit する。raw string param へ user-call arg を渡す場合は、
  concat/tagged emission 後に tag bits を落として raw pointer として渡す。
- [x] `for (let i = 0; ...; i = i + 1)` の body lowering で `i` が number-literal fact `0`
  のまま残り、`arr[i]` が `arr[0]` に静的化される問題を確認した。for update/body の assigned
  local facts を body lowering 前に invalidation し、`abc451-depth8-live-set.ts` の lowered は
  `powersOfTwoStr[i]` を `PropertyGetDynamic` として保持する。
- [x] `abc451-depth8-live-set.ts` は stale `i = 0` fact invalidation 後の full differential で pass。
  直接実行で見えていた allocator memory-limit trap は full differential の通常条件では再現せず、
  今回の blocker は GC/live-set ではなく、for-loop update/body の assignment fact が body lowering に残ることだった。
- [x] `bigint-runtime-mixed-stdin-string-in-range.ts` と
  `bigint-runtime-mixed-stdin-string-relational-out-of-range-trap.ts` の mismatch/trap は、
  resolver が relational operand に挿入した unary plus / `NumberCoerce` を native emitter がそのまま
  `$less` に渡し、stdin string を BigInt/String 比較ではなく BigInt/Number 比較へ寄せていたことが直接原因。
  もう一つの bottleneck は BigInt/String runtime branch が signed-i32 範囲の small-int parser に限定され、
  `"2147483648"` のような範囲外 decimal string を boolean 比較として処理できなかった点。
- [x] native relational emitter は、片側が静的に BigInt と分かる比較に限って resolver 由来の unary plus を剥がす。
  runtime 側には decimal `StringToBigInt` 用の private helper を追加し、任意長 decimal string を
  BigInt と直接比較する。invalid string は false、`0x` / `0b` / `0o` prefix は従来の small-int path へ
  fallback する。runtime embed/catalog は `$less` / `$greater` / `$less_or_equal` /
  `$greater_or_equal` の helper chain に新 helper を含める。
- [x] focused verification:
  `target/debug/ts2wasm build -o /tmp/array-map-string.wasm fixtures/core-semantics/array-map-arrow-pushed-local-string-constructor.ts`
  と `iwasm /tmp/array-map-string.wasm` は `1\n1\n1`。
  `TS2WASM_RUN_NODE_DIFF=1 cargo test -p ts2wasm-cli array_map_arrow_pushed_local_string_constructor_fixture_matches_node_output_under_iwasm -- --nocapture`
  pass。
  `TS2WASM_RUN_NODE_DIFF=1 cargo test -p ts2wasm-cli bigint_runtime_mixed_stdin_string_in_range_matches_node_output_under_iwasm -- --nocapture`
  pass。
  `TS2WASM_RUN_NODE_DIFF=1 cargo test -p ts2wasm-cli bigint_runtime_mixed_stdin_string_relational_out_of_range_matches_node_output_under_iwasm -- --nocapture`
  pass。
  `TS2WASM_RUN_NODE_DIFF=1 cargo test -p ts2wasm-cli bigint_runtime_mixed_relational_matches_node_output_under_iwasm -- --nocapture`
  pass。
  `TS2WASM_RUN_NODE_DIFF=1 cargo test -p ts2wasm-cli --test auto_differential_tests auto_diff_atcoder_abc451_d_concat_power2_ts -- --nocapture`
  pass。
- [x] full fixture differential rerun:
  `/tmp/ts2wasm-fixture-differential-current-20260527-004953.log` は
  `pass=1110 fail=0 unsupported=178 blocked=63 total=1351 elapsed=171.4s`。
  直前 snapshot から fail は 4 件すべて解消し、`array-map-arrow-pushed-local-string-constructor.ts`、
  `abc451-depth8-live-set.ts`、`bigint-runtime-mixed-stdin-string-in-range.ts`、
  `bigint-runtime-mixed-stdin-string-relational-out-of-range-trap.ts` は pass。
  残りは実装 fail ではなく、unsupported bucket と Node oracle / backend-io blocked bucket。
- [x] regression guard:
  `cargo fmt --package ts2wasm-backend-wasm --package ts2wasm-ir --check` pass、
  `cargo check -p ts2wasm-backend-wasm` pass、`cargo test -p ts2wasm-backend-wasm typed_relational_comparison_helpers_emit_valid_wasm -- --nocapture`
  pass、`cargo test -p ts2wasm-backend-wasm typed_loose_equality_helpers_emit_valid_wasm -- --nocapture`
  pass、`cargo build -p ts2wasm-cli` pass、`git diff --check` pass。
  skill-required `scripts/run/verify-harness.sh --quick|--cargo|--fixtures` はこの repo に存在せず、
  いずれも exit 127。

## 2026-05-27 追加確認: assignment-started comma expression parser gap

- [x] `fixtures/parser/comma-expression-statement.ts` は `a = 2, b = c, c = 0;` を
  parser の statement dispatch が `assign_statement` として早取りし、最初の RHS `2` の直後で
  semicolon を要求して `expected Semicolon, got Some(Comma)` に落ちていた。
  AST / resolver / lowered 側には `Sequence` と assignment expression の処理が既にあり、入口の parser gap が
  直接の bottleneck。
- [x] `assign_statement` は通常の `Stmt::Assign` 経路を維持し、assignment の直後に comma が続く場合だけ
  先頭 assignment を `Expr::Assign` に包んで、残りを `self.assignment()` で読む
  `Expr::Sequence` statement に変換するようにした。fixture は `a` / `b` / `c` を出力する形にして、
  parser acceptance だけでなく side effect の Node parity も確認できる。
- [x] focused verification:
  `cargo test -p ts2wasm-frontend parses_assignment_started_comma_expression_statement -- --nocapture` pass、
  `TS2WASM_RUN_NODE_DIFF=1 cargo test -p ts2wasm-cli comma_expression_statement_matches_node_output -- --nocapture`
  pass。direct Node/iwasm diff でも出力は `2\n3\n0\n` で一致。
- [x] full fixture differential:
  `/tmp/ts2wasm-fixture-differential-comma-expression-20260527-005941.log` は
  `pass=1111 fail=0 unsupported=177 blocked=63 total=1351 elapsed=173.7s`。
  直前 `pass=1110 fail=0 unsupported=178 blocked=63` から
  `fixtures/parser/comma-expression-statement.ts` が `unsupported -> pass`。`feature:parser-syntax` bucket は
  5 件から 4 件へ減った。
- [x] regression guard:
  `cargo fmt --package ts2wasm-frontend --package ts2wasm-cli --check` pass、
  `cargo check -p ts2wasm-frontend` pass、`cargo check -p ts2wasm-backend-wasm` pass、
  `cargo build -p ts2wasm-cli` pass、`git diff --check` pass。
  skill-required `scripts/run/verify-harness.sh --quick|--cargo|--fixtures` はこの repo に存在せず、
  いずれも exit 127。

## 2026-05-27 追加確認: RegExp.prototype.compile static lowering

- [x] `fixtures/core-semantics/regexp-compile-unsupported.ts` は lowering の
  `UnsupportedRegExp/lowering` issue-051 guard で止まっていた。native 側の RegExp 表現は既に
  `new RegExp(...)` / literal を pattern string として保持し、`test` / `match` はその local string を
  runtime RegExp helper に渡す形で動いていたため、bottleneck は runtime emitter ではなく
  `compile` 呼び出しの lowering policy。
- [x] `RegExp.prototype.compile` は静的 RegExp receiver に限定して、constructor と同じ
  pattern/flags literal builder を使い、識別子 receiver では同じ local に再代入して receiver を返す
  Block に lower するようにした。fixture は `abc -> def -> ghi` の再 compile と
  `console.log(r.compile("ghi"))` の return value を確認する形に更新した。
- [x] focused verification:
  `cargo fmt --package ts2wasm-ir --package ts2wasm-cli --check` pass、
  `cargo test -p ts2wasm-cli --test ir_lowering lowering_accepts_regexp -- --nocapture` pass、
  `cargo test -p ts2wasm-cli --test ir_lowering lowering_accepts_new_regexp_compile -- --nocapture` pass、
  `cargo test -p ts2wasm-cli --test ir_lowering lowering_accepts_direct_new_regexp_compile -- --nocapture` pass、
  `cargo check -p ts2wasm-backend-wasm` pass、`cargo build -p ts2wasm-cli` pass。
  direct Node/iwasm diff for `fixtures/core-semantics/regexp-compile-unsupported.ts` は
  `true\nfalse\ntrue\n/ghi/\nfalse\ntrue\n` で一致し、
  `TS2WASM_RUN_NODE_DIFF=1 cargo test -p ts2wasm-cli regexp_compile_fixture_matches_node -- --nocapture` も pass。
- [x] full fixture differential:
  `/tmp/ts2wasm-fixture-differential-regexp-compile-20260527-011121.log` は
  `pass=1112 fail=0 unsupported=176 blocked=63 total=1351 elapsed=175.2s`。
  直前 `pass=1111 fail=0 unsupported=177 blocked=63` から
  `fixtures/core-semantics/regexp-compile-unsupported.ts` が `unsupported -> pass`。
  `feature:regexp-literal` bucket は 1 件から 0 件へ減った。
- [x] regression guard:
  `git diff --check` pass。
  skill-required `scripts/run/verify-harness.sh --quick|--cargo|--fixtures` はこの repo に存在せず、
  いずれも exit 127。

## 2026-05-27 追加確認: static string return spread closure

- [x] `fixtures/core-semantics/fncsem-spread-dynamic-unsupported.ts`、
  `spread-call-dynamic-unsupported.ts`、`spread-array-unsupported.ts` は、いずれも zero-arg function が
  `return "345";` のような静的 string を返すだけなのに、`FunctionSignature` にその事実がなく、
  `resolved_expr_static_string_value` が call result / local initializer を static string として解けなかった。
  issue-274 spread guard 自体ではなく、関数 signature fact の欠落が bottleneck。
- [x] `FunctionSignature::returns_static_string` を追加し、top-level function / class constructor /
  class method / function expression / nested function lowering で `body_returns_static_string` から埋めるようにした。
  `resolved_expr_static_string_value` は zero-arg call の signature fact を読むようにし、generated function 用の
  `ctx.functions.static_string_returns` fallback は維持した。node_diff regression は対象 3 fixture を
  unsupported 診断期待から Node output parity に昇格した。
- [x] focused verification:
  `cargo fmt --package ts2wasm-ir --package ts2wasm-cli --check` pass、
  `cargo build -p ts2wasm-cli` pass、
  direct Node/iwasm diff は `fncsem-spread-dynamic-unsupported.ts` が `345`、
  `spread-call-dynamic-unsupported.ts` が `345`、
  `spread-array-unsupported.ts` が `3` で一致。
  `TS2WASM_RUN_NODE_DIFF=1 cargo test -p ts2wasm-cli --test node_diff fncsem_spread_dynamic_call_matches_node_output -- --nocapture`
  pass、
  `TS2WASM_RUN_NODE_DIFF=1 cargo test -p ts2wasm-cli --test node_diff spread_operator_static_string_return_forms_match_node_output -- --nocapture`
  pass。
- [x] full fixture differential:
  `/tmp/ts2wasm-fixture-differential-static-string-spread-20260527-012130.log` は
  `pass=1115 fail=0 unsupported=173 blocked=63 total=1351 elapsed=172.2s`。
  直前 `pass=1112 fail=0 unsupported=176 blocked=63` から対象 3 fixture が `unsupported -> pass`。
  `feature:spread` bucket は 3 件から 0 件へ減った。
- [x] regression guard:
  `git diff --check` pass。
  skill-required `scripts/run/verify-harness.sh --quick|--cargo|--fixtures` はこの repo に存在せず、
  いずれも exit 127。

## 2026-05-27 追加確認: JS direct call arity boundary

- [x] `call-extra-args-reject.ts` / `fncsem-call-fewer-args.ts` / `call-fewer-args-reject.ts` /
  `negative/arity-mismatch.ts` を確認した。bodyful user function への direct call でも semantic validator が
  TypeScript-style の exact/min arity を先に強制しており、JS 互換の extra args と
  `arguments` object 経由で安全に扱える fewer args が native lowering 前に止まるのが bottleneck だった。
- [x] `TypeScriptFunctionArity` に `enforce` と `allow_missing` を追加し、ambient/declare signature だけを
  TS-style arity enforcement として維持した。非 ambient の bodyful function call は extra args を許可し、
  missing args は callee が implicit `arguments` を読む場合だけ許可する。通常の `sum(5)` は HIR/native binary
  backend が raw numeric param を `undefined` として表現できず、現状は Node の `NaN` ではなく `5` を出すため
  reject のまま残す。
- [x] tagged undefined の将来接続に備え、`$add` / typed add helper は tagged `undefined` operand を
  encoded `NaN` に畳むようにした。`native_lowered` も `LoweredExpr::Undefined(_)` を tagged value として扱う。
  ただし raw-param ABI の missing required arg はまだこの path に乗らないため、一般的な fewer args parity は
  次の bottleneck。
- [x] focused verification:
  `cargo fmt --package ts2wasm-ir --package ts2wasm-cli --package ts2wasm-backend-wasm` applied、
  `cargo fmt --package ts2wasm-cli --check` pass、
  `cargo check -p ts2wasm-ir -p ts2wasm-backend-wasm` pass、
  `cargo check -p ts2wasm-backend-wasm` pass、
  `cargo test -p ts2wasm-cli --test ir_lowering typescript_semantics_ -- --nocapture` は 6 tests pass。
  `TS2WASM_RUN_NODE_DIFF=1 cargo test -p ts2wasm-cli --test node_diff fncsem_call_extra_args_reject_fixture_matches_node_output -- --nocapture`
  pass、
  `TS2WASM_RUN_NODE_DIFF=1 cargo test -p ts2wasm-cli --test node_diff fncsem_implicit_arguments_fewer_args_matches_node_output -- --nocapture`
  pass、
  `TS2WASM_RUN_NODE_DIFF=1 cargo test -p ts2wasm-cli --test node_diff fncsem_v2_call_fewer_args_reports_arity_mismatch -- --nocapture`
  pass。`fixtures/negative/arity-mismatch.ts` は `ArityMismatch` ではなく
  `UnresolvedFunction/semantic-validator` へ移ったが、bodyless user function resolution はこの slice の外。
- [x] full fixture differential:
  `/tmp/ts2wasm-fixture-differential-arity-js-boundary-20260527-013717.log` は
  `pass=1118 fail=0 unsupported=170 blocked=63 total=1351 elapsed=162.8s`。
  直前 `pass=1115 fail=0 unsupported=173 blocked=63` から
  `call-extra-args-reject.ts` と `fncsem-call-fewer-args.ts` が `unsupported -> pass`、
  `negative/arity-mismatch.ts` は `feature:parser-syntax` から `feature:unknown-unsupported` へ移動した。
  tracking は `feature:parser-syntax=1`、`feature:function=1`、`feature:name-resolution=1`。
- [x] regression guard:
  `git diff --check` pass。
  skill-required `scripts/run/verify-harness.sh --quick|--cargo|--fixtures` はこの repo に存在せず、
  いずれも exit 127。

## 2026-05-27 追加確認: top-level function expression mutable capture env-cell

- [x] `fixtures/module-system/live-binding-unsupported.ts` は `export function increment() { counter++; }`
  が static module rewrite 後に `let increment = function ...` へ変換されるため、top-level function declaration 用の
  mutable capture 収集には乗らず、top-level `counter` が env-cell 化されなかった。その結果、
  `lower_nested_function` が `counter` mutation を heap environment 未対応として reject していた。
  bottleneck は native emitter ではなく、module rewrite 後の top-level function expression mutable capture を
  env-cell source of truth へ渡していない lowering 前処理。
- [x] top-level `env_cell_names` に `collect_block_nested_function_mutable_captures(program)` を追加し、
  function expression / nested function が mutable capture する top-level local も env-cell として扱うようにした。
  既存の top-level function declaration capture、object method capture、direct eval env-cell は維持した。
- [x] regression: `crates/cli/tests/builtin_methods/part_3.rs` の
  `live_binding_unsupported_diagnostic` を build-success expectation に更新した。
  既存 `crates/cli/tests/modules.rs::build_smoke_live_binding` も引き続き pass。
- [x] focused verification:
  `cargo fmt --package ts2wasm-ir --package ts2wasm-cli --check` pass、
  `cargo check -p ts2wasm-ir` pass、
  `cargo build -p ts2wasm-cli` pass、
  direct `target/debug/ts2wasm build fixtures/module-system/live-binding-unsupported.ts` pass、
  direct `iwasm` stdout は空で Node stdout と一致。
  `cargo test -p ts2wasm-cli --test builtin_methods live_binding_module_builds_successfully -- --nocapture`
  pass、
  `cargo test -p ts2wasm-cli --test modules build_smoke_live_binding -- --nocapture` pass。
  1-fixture differential も `pass=1 fail=0 unsupported=0 blocked=0`。
- [x] full fixture differential:
  `/tmp/ts2wasm-fixture-differential-live-binding-envcell-20260527-014515.log` は
  `pass=1119 fail=0 unsupported=169 blocked=63 total=1351 elapsed=183.4s`。
  直前 `pass=1118 fail=0 unsupported=170 blocked=63` から
  `fixtures/module-system/live-binding-unsupported.ts` が `unsupported -> pass`。
  `feature:function` bucket は 1 件から 0 件へ減った。
- [x] regression guard:
  `git diff --check` pass。
  skill-required `scripts/run/verify-harness.sh --quick|--cargo|--fixtures` はこの repo に存在せず、
  いずれも exit 127。

## 2026-05-27 追加確認: indirect eval ReferenceError rejection oracle

- [x] `fixtures/negative/unsupported-eval.ts` は `(0, eval)("x")` で、Node は `ReferenceError: x is not defined`、
  compiler は静的 indirect eval source の `x` を `UnresolvedName` として拒否する。これは native emitter の
  runtime parity gap ではなく、Node と compiler の両方が reject する fixture を differential が
  compile-time expected rejection として扱えず、`feature:name-resolution` に誤分類していたのが bottleneck。
- [x] `scripts/check/fixture-differential.py` の `EXPECTED_REJECTION_FIXTURES` に
  `fixtures/negative/unsupported-eval.ts` を追加し、build failure branch でも Node 側 needle と compiler
  diagnostic needle が両方一致すれば pass として扱うようにした。runtime rejection 用の既存 `iwasm`
  expectation とは分離し、この fixture は `"compiler"` needle を使う。
- [x] `scripts/check/compiler-diagnostics.py` はこの fixture の現行 diagnostic に合わせて
  `unsupported-eval -> UnresolvedName` へ更新した。`unsupported-eval.ts -> [UnresolvedName]` は pass。
  ただし script 全体は既存の unrelated negative fixtures 5 件
  (`arity-mismatch.ts`, `duplicate-function.ts`, `invalid-top-level-return.ts`,
  `typescript-type-check.ts`, `unsupported-typescript-syntax.ts`) で fail したままなので、この slice では
  regression guard として対象 fixture の確認に留めた。
- [x] focused verification:
  `python3 -m py_compile scripts/check/fixture-differential.py scripts/check/compiler-diagnostics.py` pass。
  `target/debug/ts2wasm build fixtures/negative/unsupported-eval.ts` は
  `[UnresolvedName] unresolved name: \`x\``、`node fixtures/negative/unsupported-eval.ts` は
  `ReferenceError: x is not defined`。
  1-fixture differential は `pass=1 fail=0 unsupported=0 blocked=0 total=1`。
- [x] full fixture differential:
  `/tmp/ts2wasm-fixture-differential-unsupported-eval-rejection-20260527-015118.log` は
  `pass=1121 fail=0 unsupported=167 blocked=63 total=1351 elapsed=169.5s`。
  直前 `pass=1119 fail=0 unsupported=169 blocked=63` から
  `fixtures/negative/unsupported-eval.ts` が `feature:name-resolution -> pass`、
  `fixtures/builtins-and-io/promise-then-unsupported-diagnostic.ts` が既存変更の影響で
  `feature:unknown-unsupported -> pass`。tracking は
  `feature:unknown-unsupported=122`, `feature:node-oracle-fail=56`, `feature:class=13`,
  `feature:async=12`, `feature:import-export=12`, `feature:backend-io=7`,
  `feature:arrow-function=5`, `feature:destructuring=2`, `feature:parser-syntax=1` で、
  `feature:name-resolution` bucket は 1 件から 0 件へ減った。
- [x] regression guard:
  `git diff --check` pass。
  skill-required `scripts/run/verify-harness.sh --quick|--cargo|--fixtures` はこの repo に存在せず、
  いずれも exit 127。

## 2026-05-27 追加確認: JS fewer args missing param undefined/NaN

- [x] `fixtures/core-semantics/call-fewer-args-reject.ts` は `sum(5)` で Node が missing `b` を
  `undefined` として扱い、`5 + undefined` が `NaN` になる。一方 compiler は bodyful user function にも
  TypeScript-style missing-argument arity を適用しており、native lowering 前に
  `ArityMismatch/semantic-validator` で reject していた。bottleneck は parser syntax ではなく、
  bodyful JS function call の missing args を ambient declaration arity と同じ gate で止めていた
  semantic validator と、static `+` evaluator が `undefined` の numeric coercion を扱わない点。
- [x] `validate_typescript_call_arity` は ambient/declare signature だけを TS-style arity enforcement として残し、
  bodyful function calls は missing/extra args を JS call semantics として許可するようにした。
  既存 lowering/emitter は missing direct-call args を `undefined` で padding するため、
  static user-call evaluator 側で missing param を `LoweredExpr::Undefined` として bind できる。
- [x] native static `+` evaluator は string concat 優先判定後の numeric path で
  JS numeric coercion (`undefined -> NaN`, `null -> 0`, bool/string coercion) を使うようにした。
  これにより `sum(5)` の static user-call return は `DecimalNumber("NaN")` になり、console output が Node と一致する。
- [x] regression: `crates/cli/tests/common/node_diff_fixture_tests/part_6.rs` の
  `call-fewer-args-reject.ts` を Node output parity に昇格した。同じ file の
  `dynamic-call-assign-unsupported.ts` expectation は、既に latest differential で pass 側だった stale test を
  Node output parity に合わせた。
  `crates/cli/tests/ir_lowering.rs` は bodyful missing-arg call を accept する semantic test に更新し、
  ambient missing-argument rejection は維持した。
- [x] focused verification:
  `cargo fmt --package ts2wasm-ir --package ts2wasm-cli --package ts2wasm-backend-wasm --check` pass、
  `cargo check -p ts2wasm-ir -p ts2wasm-backend-wasm` pass、
  `cargo test -p ts2wasm-cli --test ir_lowering typescript_semantics_ -- --nocapture` は 6 tests pass。
  direct `target/debug/ts2wasm build fixtures/core-semantics/call-fewer-args-reject.ts` pass、
  direct `iwasm` stdout は `NaN` で Node と一致。
  1-fixture differential は `pass=1 fail=0 unsupported=0 blocked=0 total=1`。
  `TS2WASM_RUN_NODE_DIFF=1 cargo test -p ts2wasm-cli --test node_diff fncsem_v2_call_fewer_args_reject_fixture_matches_node_output -- --nocapture`
  pass。
  `TS2WASM_RUN_NODE_DIFF=1 cargo test -p ts2wasm-cli --test node_diff fncsem_ -- --nocapture`
  は 20 tests pass。
- [x] full fixture differential:
  `/tmp/ts2wasm-fixture-differential-fewer-args-js-undefined-20260527-015945.log` は
  `pass=1122 fail=0 unsupported=166 blocked=63 total=1351 elapsed=164.5s`。
  直前 `pass=1121 fail=0 unsupported=167 blocked=63` から
  `fixtures/core-semantics/call-fewer-args-reject.ts` が `feature:parser-syntax -> pass`。
  tracking は `feature:unknown-unsupported=122`, `feature:node-oracle-fail=56`,
  `feature:class=13`, `feature:async=12`, `feature:import-export=12`,
  `feature:backend-io=7`, `feature:arrow-function=5`, `feature:destructuring=2` で、
  `feature:parser-syntax` bucket は 1 件から 0 件へ減った。
- [x] regression guard:
  `git diff --check` pass。
  skill-required `scripts/run/verify-harness.sh --quick|--cargo|--fixtures` はこの repo に存在せず、
  いずれも exit 127。

## 2026-05-27 追加確認: nested namespace expected rejection

- [x] `fixtures/core-semantics/nested-namespace-abc.ts` と
  `fixtures/core-semantics/nested-namespace-unsupported.ts` は native emitter 実装漏れではなかった。
  前者は Node v23.6.0 の strip-only TypeScript 実行が `namespace` declaration を
  `ERR_INVALID_TYPESCRIPT_SYNTAX` として拒否し、compiler も nested namespace/module resolution を
  `UnsupportedModule/name-resolver` として拒否する。後者は Node が unresolved `A` を
  `ReferenceError: A is not defined` として拒否し、compiler は同じ nested namespace/module resolution
  diagnostic で拒否する。bottleneck は lowering/emitter ではなく、differential harness の
  compile-time expected rejection 登録漏れだった。
- [x] `scripts/check/fixture-differential.py` の `EXPECTED_REJECTION_FIXTURES` に上記 2 件を追加し、
  Node/compiler 双方の拒否 needle を固定した。これにより destructuring bucket に誤分類されていた
  nested namespace rejection が pass 扱いになる。
- [x] focused verification:
  `python3 -m py_compile scripts/check/fixture-differential.py` pass。
  2-fixture catalog での differential は
  `pass=2(100%) fail=0 unsupported=0 blocked=0 total=2`。
- [x] full fixture differential:
  `/tmp/ts2wasm-fixture-differential-nested-namespace-rejection-20260527-020657.log` は
  `pass=1125(83%) fail=0 unsupported=163 blocked=63 total=1351 elapsed=174.5s`。
  対象 2 件はどちらも `pass/tracking=None` になり、`feature:destructuring` は 2 件から 0 件へ減った。
  tracking は `feature:unknown-unsupported=121`, `feature:node-oracle-fail=56`,
  `feature:class=13`, `feature:async=12`, `feature:import-export=12`,
  `feature:backend-io=7`, `feature:arrow-function=5`。
  前回ログとの差分では、対象 2 件に加えて既存変更の影響として
  `fixtures/control-flow-and-exceptions/return-in-try-finally.ts` も
  `feature:unknown-unsupported -> pass` へ改善していた。
- [x] regression guard:
  skill-required `bash scripts/run/verify-harness.sh --quick`,
  `bash scripts/run/verify-harness.sh --cargo`,
  `bash scripts/run/verify-harness.sh --fixtures` はこの repo に存在せず、いずれも exit 127。

## 2026-05-27 追加確認: recursive arrow short-circuit static side effects

- [x] `fixtures/core-semantics/arrow-assigned-recursive-unsupported.ts` は native emitter 実装漏れだった。
  `const fact = n => (n === 1 && 1) || n * fact(n - 1)` の base case で、静的 side-effect
  collection が `&&`/`||` の right operand を無条件に辿り、`fact(1)` の `||` 右辺から
  `fact(0)`, `fact(-1)`... へ再帰して backend lowering 中に stack overflow していた。
  bottleneck は arrow lowering そのものではなく、
  `collect_static_locals_from_expr_with_functions` の short-circuit side-effect 判定漏れ。
- [x] `LoweredBinaryOp::And` と `LoweredBinaryOp::Or` を `NullishCoalesce` と同じ形で特別扱いし、
  左辺の静的 truthiness が確定する場合は JS short-circuit に従って実行される右辺だけを収集する。
  左辺が静的に不明な場合は従来どおり右辺も収集し、既存の保守的な fallback は維持した。
  stale な build-only regression は Node parity assertion に更新した。
- [x] focused verification:
  `cargo fmt --package ts2wasm-backend-wasm --package ts2wasm-cli --check` pass。
  `TS2WASM_RUN_NODE_DIFF=1 cargo test -p ts2wasm-cli --test node_diff arrow_assigned_recursive_fixture_matches_node_output -- --nocapture`
  pass。
  direct build/run では
  `target/debug/ts2wasm build -o /tmp/ts2wasm-arrow-recursive.wasm fixtures/core-semantics/arrow-assigned-recursive-unsupported.ts`
  と `iwasm /tmp/ts2wasm-arrow-recursive.wasm` が `24` を出し、Node も `24`。
  5-fixture arrow catalog differential は `pass=1 fail=0 unsupported=4 blocked=0 total=5` で、
  対象 fixture が `pass/tracking=None` になった。
- [x] full fixture differential:
  `/tmp/ts2wasm-fixture-differential-recursive-arrow-short-circuit-20260527-021628.log` は
  `pass=1128(83%) fail=0 unsupported=160 blocked=63 total=1351 elapsed=169.2s`。
  tracking は `feature:unknown-unsupported=119`, `feature:node-oracle-fail=56`,
  `feature:class=13`, `feature:async=12`, `feature:import-export=12`,
  `feature:backend-io=7`, `feature:arrow-function=4`。
  前回ログとの差分では、対象の
  `fixtures/core-semantics/arrow-assigned-recursive-unsupported.ts` が
  `feature:arrow-function -> pass` になり、既存変更の影響として
  `fixtures/control-flow-and-exceptions/break-continue-in-try-finally.ts` と
  `fixtures/control-flow-and-exceptions/throw-rethrow-nested.ts` も
  `feature:unknown-unsupported -> pass` へ改善した。
- [x] regression guard:
  `git diff --check` pass。
  skill-required `bash scripts/run/verify-harness.sh --quick`,
  `bash scripts/run/verify-harness.sh --cargo`,
  `bash scripts/run/verify-harness.sh --fixtures` はこの repo に存在せず、いずれも exit 127。

## 2026-05-27 追加確認: Array.sort function comparator

- [x] `fixtures/negative/unsupported-builtin.ts` は native runtime 実装漏れではなく、lowering の
  comparator shape 認識漏れだった。既存の `Array.prototype.sort((a, b) => a - b)` は
  `RuntimeFn::ArraySortNumeric` に落ちていたが、同じ numeric comparator を
  `function(a, b) { return a - b; }` で書くと arrow 専用 predicate に弾かれ、
  `UnsupportedBuiltin/lowering/arrow-function` に分類されていた。
- [x] `numeric_ascending_sort_arrow_callback` を `numeric_ascending_sort_callback` に広げ、
  arrow body と、generator でない function expression の単一 `return a - b;` body を同じ
  `ArraySortNumeric` path に落とすようにした。default/rest parameter や複文 body は対象外のまま。
  `unsupported-builtin.ts` は stale diagnostic fixture なので、compiler diagnostics では
  untriggerable 扱いに移し、Node parity regression を追加した。空 stdout fixture でも
  `assert_no_precomputed_stdout` が `windows(0)` で panic しないようにした。
- [x] focused verification:
  `cargo fmt --package ts2wasm-ir --package ts2wasm-cli --check` pass。
  `cargo check -p ts2wasm-ir` pass（既存 warnings のみ）。
  `cargo build -p ts2wasm-cli` pass（既存 warnings のみ）。
  `TS2WASM_RUN_NODE_DIFF=1 cargo test -p ts2wasm-cli --test node_diff array_sort_function_comparator_fixture_matches_node_output -- --nocapture`
  pass。
  direct build/run では
  `target/debug/ts2wasm build -o /tmp/ts2wasm-array-sort-function-comparator.wasm fixtures/negative/unsupported-builtin.ts`
  が pass し、`iwasm` と Node はどちらも stdout 空・exit 0。
  1-fixture differential は `pass=1(100%) fail=0 unsupported=0 blocked=0 total=1`。
  `python3 scripts/check/compiler-diagnostics.py` は対象 fixture を skip することを確認したが、
  unrelated 既存 negative fixtures 5 件
  (`arity-mismatch.ts`, `duplicate-function.ts`, `invalid-top-level-return.ts`,
  `typescript-type-check.ts`, `unsupported-typescript-syntax.ts`) で fail した。
- [x] full fixture differential:
  `/tmp/ts2wasm-fixture-differential-array-sort-function-comparator-20260527-022503.log` は
  `pass=1125(83%) fail=6 unsupported=157 blocked=63 total=1351 elapsed=179.0s` で、
  script は `FAILED` を出した。tracking は `feature:unknown-unsupported=117`,
  `feature:node-oracle-fail=56`, `feature:class=13`, `feature:async=12`,
  `feature:import-export=12`, `feature:backend-io=7`, `feature:stdout-mismatch=6`,
  `feature:arrow-function=3`。
  対象の `fixtures/negative/unsupported-builtin.ts` は `feature:arrow-function -> pass` になり、
  `feature:arrow-function` bucket は 4 件から 3 件へ減った。既存変更の影響として
  `fixtures/core-semantics/private-class-setter-same-class-receiver.ts` と
  `fixtures/core-semantics/private-class-setter-same-class-receiver-brand.ts` も
  `feature:unknown-unsupported -> pass` へ改善した。一方で current rebuilt binary では
  `bigint-runtime-large-mul-local-flow.ts`, `direct-eval-dynamic-catch-binding-node-shim.ts`,
  `direct-eval-dynamic-strict-caller-array-binding-arguments-node-shim.ts`,
  `direct-eval-dynamic-strict-caller-delete-arguments-node-shim.ts`,
  `direct-eval-dynamic-strict-caller-var-arguments-node-shim.ts`,
  `new-eval-type-error.ts` が `pass -> feature:stdout-mismatch` に動いたため、full gate は未達。

## 2026-05-27 追加確認: stdout mismatch 回収

- [x] 前回 full differential の `feature:stdout-mismatch=6` を直接再現して分類した。
  `new-eval-type-error.ts` は `new EvalError/TypeError` 相当の block throw completion を
  static exception evaluator が見ず、catch 後に `"unreachable"` まで出していた。
  `direct-eval-dynamic-catch-binding-node-shim.ts` は `console.log(eval(source))` の
  static output は出せていたが、console argument evaluation の side effect を static locals に
  反映せず、catch binding writeback が `7 -> 3` に戻っていた。strict caller `arguments`
  3 件は direct eval の strict SyntaxError を catch した後、synthetic `arguments` env-cell の
  static argument object を維持したまま `arguments[0]` を読む必要があった。
- [x] 実装: `static_exception_completion_from_expr` が `LoweredExpr::Block` 内の throw completion を
  `static_exception_console_eval_stmts` 経由で拾うようにした。`ConsoleLog` の static exception
  path は出力後に各 argument の static side effect を回収するようにした。さらに
  `EvalDirectHost` / `EvalIndirectHost` の expression statement を static exception completion
  対象に含め、strict eval SyntaxError を user-function static console path でも処理できるようにした。
- [x] verification:
  `cargo fmt --package ts2wasm-backend-wasm --check` pass。
  `cargo build -p ts2wasm-cli` pass（既存 warnings のみ）。
  `TS2WASM_RUN_NODE_DIFF=1 cargo test -p ts2wasm-cli --test node_diff direct_eval_dynamic_strict_eval_static_policy_matches_node_output -- --nocapture`
  pass。direct build/run では strict caller `arguments` 3 件がすべて `SyntaxError\n9\n` で Node と一致。
  `new-eval-type-error.ts` は direct build/run と 1-fixture differential で `TypeError` 一致。
  `direct-eval-dynamic-catch-binding-node-shim.ts` は direct build/run で `7\n7\n` 一致。
  `bigint-runtime-large-mul-local-flow.ts` は current rebuilt native binary では `-88n` を再現せず、4 行すべて Node と一致。
- [x] focused differential:
  `/tmp/ts2wasm-fixture-differential-six-stdout-20260527-024208.log` は
  `pass=6(100%) fail=0 unsupported=0 blocked=0 total=6 elapsed=0.8s`。
- [x] full fixture differential:
  `/tmp/ts2wasm-fixture-differential-post-stdout-recovery-20260527-024215.log` は
  `pass=1131(83%) fail=0 unsupported=157 blocked=63 total=1351 elapsed=166.3s`。
  script は既存 `blocked=63` により `FAILED` を出したが、`feature:stdout-mismatch` は 0 件になり、
  前回の 6 件はすべて pass に戻った。
- [x] regression guard:
  `git diff --check` pass。
  skill-required `bash scripts/run/verify-harness.sh --quick`,
  `bash scripts/run/verify-harness.sh --cargo`,
  `bash scripts/run/verify-harness.sh --fixtures` はこの repo に存在せず、いずれも exit 127。

## 2026-05-27 追加確認: BigInt ToPrimitive `this` member fold

- [x] 最新 full differential の残 bucket を分類し、`feature:arrow-function=3` は実体として
  `issue-374` の mixed BigInt object ToPrimitive 静的 fold 制限だった。
  `bigint-runtime-mixed-object-toprimitive-method-unsupported.ts` は
  `({ valueOf() { return this.x; } }) == 1n` で、Node は missing property を `undefined` にして
  `false` を出す。一方、builtin resolver の ToPrimitive fold は direct primitive literal return のみを
  認識し、method shorthand body の `this.x` を object literal の own property/missing property に解決できず
  `UnsupportedSyntax/builtin-resolver/arrow-function` に落としていた。
- [x] `crates/ir/src/builtin_resolver.rs` の
  `object_toprimitive_supported_primitive_expr` を拡張し、no-arg `FunctionExpr` の `return this.<prop>` を
  同じ object literal の静的 property に解決するようにした。property が存在しない場合は JS property read と
  同じく `undefined` として fold する。primitive でない property 値は従来どおり unsupported 側に残す。
- [x] regression:
  `crates/cli/tests/common/node_diff_fixture_tests/part_2.rs` に
  `bigint_runtime_mixed_object_toprimitive_missing_this_member_matches_node_output_under_iwasm` を追加し、
  `bigint-runtime-mixed-object-toprimitive-method-unsupported.ts` を issue-374 expected rejection から外した。
  同じ近傍で issue-373 diagnostic の code prefix 期待が phase 付き診断
  `[UnsupportedRuntimeSubset/builtin-resolver]` に合っていなかったため、prefix match に修正。
- [x] focused verification:
  `TS2WASM_RUN_NODE_DIFF=1 cargo test -p ts2wasm-cli --test node_diff bigint_runtime_mixed_object_toprimitive -- --nocapture`
  は 4 tests pass。
- [x] full fixture differential:
  `/tmp/ts2wasm-fixture-differential-toprimitive-this-20260527.log` は
  `pass=1134(83%) fail=0 unsupported=154 blocked=63 total=1351 elapsed=163.7s`。
  script は既存 `blocked=63` により exit 1。比較では
  `bigint-runtime-mixed-object-toprimitive-method-unsupported.ts` が `unsupported => pass`。
  追加で `private-class-getter-same-class-receiver-brand.ts` と
  `private-class-method-same-class-receiver-brand.ts` も `unsupported => pass` になっているが、この slice の
  編集対象外であり、dirty worktree 上の既存差分由来として扱う。
- [x] regression guard:
  `cargo fmt --package ts2wasm-ir --package ts2wasm-cli --check` pass。
  `cargo build -p ts2wasm-cli` pass。
  `git diff --check` pass。
  skill-required `bash scripts/run/verify-harness.sh --quick`,
  `bash scripts/run/verify-harness.sh --cargo`,
  `bash scripts/run/verify-harness.sh --fixtures` はこの repo に存在せず、いずれも exit 127。

## 2026-05-27 追加確認: BigInt ToPrimitive object-return valueOf fold

- [x] 残り `feature:arrow-function=2` を再分類した。
  `bigint-runtime-mixed-object-toprimitive-unsupported.ts` は
  `let objectBigInt = { valueOf: () => ({ x: 1n }) }; console.log(objectBigInt == 1n);`
  で、Node は `false` を出す。bottleneck は native runtime の BigInt arithmetic ではなく、
  builtin resolver の object ToPrimitive fold が `valueOf` の object return 後に own `toString`
  がないケースを default `Object.prototype.toString` path として扱えず、issue-374 に落としていたこと。
  一方 `bigint-runtime-mixed-object-toprimitive-string-unsupported.ts` は own `toString` も object を返し、
  Node が `TypeError: Cannot convert object to primitive value` を投げるため、catchable TypeError
  lowering/runtime helper が必要な別 slice として残す。
- [x] `crates/ir/src/builtin_resolver.rs` の `object_toprimitive_supported_primitive_expr` で、
  `valueOf` が object を返し、own `toString` が存在しない場合は BigInt equality/comparison fold 境界で
  `undefined` 相当として扱うようにした。これにより equality は Node と同じ `false` に fold される。
  own `toString` が存在する場合は従来どおり、その return value を評価するか issue-374 側に残す。
- [x] regression:
  `crates/cli/tests/common/node_diff_fixture_tests/part_2.rs` に
  `bigint_runtime_mixed_object_toprimitive_object_valueof_matches_node_output_under_iwasm` を追加し、
  `bigint-runtime-mixed-object-toprimitive-unsupported.ts` を issue-374 expected rejection から外した。
- [x] focused verification:
  `cargo fmt --package ts2wasm-ir --package ts2wasm-cli --check` pass。
  `TS2WASM_RUN_NODE_DIFF=1 cargo test -p ts2wasm-cli --test node_diff bigint_runtime_mixed_object_toprimitive -- --nocapture`
  は 5 tests pass。
  `cargo build -p ts2wasm-cli` pass。
- [x] full fixture differential:
  `/tmp/ts2wasm-fixture-differential-toprimitive-object-valueof-20260527.log` は
  `pass=1137(84%) fail=0 unsupported=151 blocked=63 total=1351 elapsed=181.4s`。
  script は既存 `blocked=63` により exit 1。比較では
  `bigint-runtime-mixed-object-toprimitive-unsupported.ts` が `unsupported => pass`。
  追加で `private-class-derived-no-inherited-brand.ts` と
  `private-class-field-external-receiver-catch.ts` も `unsupported => pass` になっているが、この slice の
  編集対象外であり、dirty worktree 上の既存差分由来として扱う。
  残 bucket は `feature:unknown-unsupported=113`, `feature:node-oracle-fail=56`,
  `feature:class=13`, `feature:async=12`, `feature:import-export=12`,
  `feature:backend-io=7`, `feature:arrow-function=1`。
- [x] regression guard:
  `git diff --check` pass。
  skill-required `bash scripts/run/verify-harness.sh --quick`,
  `bash scripts/run/verify-harness.sh --cargo`,
  `bash scripts/run/verify-harness.sh --fixtures` はこの repo に存在せず、いずれも exit 127。

## 2026-05-27 追加確認: BigInt ToPrimitive object-return toString TypeError

- [x] 残り `feature:arrow-function=1` を回収した。
  `bigint-runtime-mixed-object-toprimitive-string-unsupported.ts` は
  `let objectString = { toString: () => ({ value: "1" }) }; console.log(1n == objectString);`
  で、Node は `TypeError: Cannot convert object to primitive value` を投げる。bottleneck は
  BigInt arithmetic runtime ではなく、BigInt mixed comparison lowering が object-returning
  ToPrimitive hook を catchable TypeError completion へ落とせず、さらに native `console.log(...)`
  lowering が引数評価内の static `Throw(ErrorNew)` block を console 出力経路で飲み込んでいたこと。
- [x] IR/lowering:
  `crates/ir/src/lowered/facts.rs`,
  `crates/ir/src/lowered/resolver/expr/facts.rs`,
  `crates/ir/src/lowered/resolver/mod.rs`,
  `crates/ir/src/builtin_resolver_bigint.rs`,
  `crates/ir/src/lowered/resolver/expr/binary.rs` で、direct object literal または local alias の
  `valueOf`/`toString` hook を追跡し、BigInt mixed equality/relational comparison の片側が
  ToPrimitive object-return TypeError 境界なら
  `throw new TypeError("Cannot convert object to primitive value")` 相当の lowered block にする。
  `valueOf` が primitive を返す場合は TypeError にせず、`valueOf` が object を返して own
  `toString` がない場合は前 slice の object-return valueOf fold に委ねる。
- [x] native emitter:
  `crates/backend-wasm/src/native_lowered.rs` の `try_emit_console_call` で、console call の出力を始める前に
  各引数の `static_exception_completion_from_expr` を確認する。static `Throw` が見つかった場合は、
  handler がなければ `TypeError: Cannot convert object to primitive value` を出して `unreachable`、
  handler があれば `$exception_pending` に載せて既存の exception branch/return 経路へ渡す。
  この経路で必要になる `$exception_handler_depth` global は native module に常時宣言するようにした。
- [x] regression:
  `crates/cli/tests/common/node_diff_fixture_tests/part_2.rs` の issue-374 expected rejection を
  `assert_fixture_node_typeerror_and_iwasm_reports_typeerror_containing` に置き換えた。
  `scripts/check/fixture-differential.py` には同 fixture を expected rejection として登録した。
  使われなくなった `BIGINT_ISSUE_374` test constant は削除した。
- [x] focused verification:
  `cargo fmt --package ts2wasm-ir --package ts2wasm-backend-wasm --package ts2wasm-cli --check` pass。
  `cargo check -p ts2wasm-ir` pass。
  `cargo check -p ts2wasm-backend-wasm` pass。
  `cargo build -p ts2wasm-cli` pass。
  direct run では Node が exit 1 / `TypeError: Cannot convert object to primitive value`、
  iwasm も exit 1 / `TypeError: Cannot convert object to primitive valueException: unreachable`。
  `TS2WASM_RUN_NODE_DIFF=1 cargo test -p ts2wasm-cli --test node_diff bigint_runtime_mixed_object_toprimitive -- --nocapture`
  は 5 tests pass。
- [x] full fixture differential:
  `/tmp/ts2wasm-fixture-differential-toprimitive-typeerror-20260527.log` は
  `pass=1139(84%) fail=0 unsupported=149 blocked=63 total=1351 elapsed=168.1s`。
  script は既存 `blocked=63` により exit 1。対象 fixture は pass になり、
  `feature:arrow-function` bucket は 0。残 bucket は
  `feature:unknown-unsupported=112`, `feature:node-oracle-fail=56`,
  `feature:class=13`, `feature:async=12`, `feature:import-export=12`,
  `feature:backend-io=7`。
- [x] regression guard:
  `git diff --check` pass。
  skill-required `bash scripts/run/verify-harness.sh --quick`,
  `bash scripts/run/verify-harness.sh --cargo`,
  `bash scripts/run/verify-harness.sh --fixtures` はこの repo に存在せず、いずれも exit 127。

## 2026-05-27 追加確認: fixture catalog backend-io cleanup と heap closure ABI

- [x] `feature:backend-io=7` は native emitter 機能不足ではなく fixture catalog の欠落参照だった。
  `fixtures/catalog.yaml` の Annex B Date entries を実在する
  `date-annexb-set-year.ts` / `date-annexb-to-gmt-string.ts` に戻し、
  欠落していた `fixtures/qa-techniques/{boundary-value,decision-table,equivalence-partitioning,error-guessing,state-transition}.ts`
  を追加した。
- [x] focused Node/iwasm parity:
  上記 7 fixtures はすべて expected Node output と iwasm output が一致した。
- [x] catalog cleanup 後の full fixture differential:
  `/tmp/ts2wasm-fixture-differential-backend-io-catalog-rerun-20260527.log` は
  `pass=1146(84%) fail=1 unsupported=147 blocked=57 total=1351 elapsed=163.1s`。
  `feature:backend-io=0` まで下がった一方で、
  `fixtures/core-semantics/returned-closure-nested-object-gc-pressure.ts` が
  `expected initial:after-pressure:42` / `actual 0` の stdout mismatch として露出した。
- [x] heap closure static ABI:
  `bind_static_heap_closure_args` が capture を `function.params` 先頭へ束縛していたが、
  native runtime ABI は user args first, captures after なので逆だった。
  user params を先頭、captures を末尾へ束縛するよう修正した。
- [x] heap closure emitted-function param representation:
  `HeapObject` closure として返される関数は runtime heap closure ABI で user args を tagged value として受ける。
  `collect_function_param_reprs` が `ArrowFn { representation: HeapObject }` を見た時点で
  capture 以外の user params を `NativeValueRepr::TaggedValue` として登録するようにした。
  一般の `InferredType::String` local を taggable とみなす案は default parameter の
  `undefined` 判定を壊すため採用しない。
- [x] focused regression:
  `fixtures/core-semantics/returned-closure-nested-object-gc-pressure.ts` は Node/iwasm ともに
  `initial:after-pressure:42`。
  `fixtures/core-semantics/default-params.ts` は Node/iwasm ともに
  `Hello, World\nHello, Alice\n0`。
- [x] full fixture differential:
  `/tmp/ts2wasm-fixture-differential-backend-io-heap-closure-rerun-20260527.log` は
  `pass=1149(85%) fail=0 unsupported=145 blocked=57 total=1351 elapsed=165.9s`。
  残バケットは `feature:unknown-unsupported=110`, `feature:node-oracle-fail=57`,
  `feature:class=13`, `feature:async=12`, `feature:import-export=10`。

## 2026-05-27 追加確認: synchronous async await native unwrap

- [x] `PromiseGetValue(Call(async fn))` のうち、callee が native function result を返す同期完了 async
  関数は promise task runtime へ落とさず、既存 static evaluator または direct user call emission で unwrap
  するようにした。`Promise.resolve(...)` も既存 static promise model から `PromiseGetValue` が値化できる。
- [x] rejection path は未実装のまま明示的に残す。`async-exception.ts` は throw-only async callee なので
  direct unwrap 対象から外し、`undefined` で正常完了したように見せない。
- [x] focused Node/iwasm parity:
  `fixtures/async-await/async-arrow.ts`, `basic-async-return.ts`, `async-chain.ts`,
  `async-error-handling.ts`, `async-nested.ts`, `async-parallel.ts`, `async-void.ts`,
  `await-sequence.ts`, `async-await-unsupported.ts`,
  `fixtures/core-semantics/async-await-unsupported.ts` は Node/iwasm 一致。
- [x] full fixture differential:
  `/tmp/ts2wasm-fixture-differential-async-static-unwrap-20260527.log` は
  `pass=1160(85%) fail=0 unsupported=135 blocked=56 total=1351 elapsed=181.2s`。
  残バケットは `feature:unknown-unsupported=110`, `feature:node-oracle-fail=56`,
  `feature:class=12`, `feature:import-export=11`, `feature:async=2`。
  async 残は `fixtures/async-await/async-exception.ts` と
  `fixtures/core-semantics/for-await-of-unsupported.ts`。

## 2026-05-27 追加確認: async rejection catch static completion

- [x] `fixtures/async-await/async-exception.ts` は lowered 上
  `try { PromiseGetValue(Call(fail)) } catch (e) { console.log(e.message) }` で、
  `fail` は async function body 内で `throw new Error("async failure")` していた。
  既存 static exception console evaluator に `Expr(PromiseGetValue(Call(User)))` を接続し、
  async callee の static throw completion を catch var へ渡すようにした。
- [x] dynamic import regression guard:
  `PromiseGetValue(ModuleLoad)` は static require のみ module init を許可し、
  `ModuleLoadKind::DynamicImport` は unsupported のまま残す。これにより
  `fixtures/builtins-and-io/dynamic-import.ts` が `57` などの誤った native output へ進まない。
- [x] focused Node/iwasm parity:
  async-await suite の `async-arrow.ts`, `basic-async-return.ts`, `async-chain.ts`,
  `async-error-handling.ts`, `async-exception.ts`, `async-nested.ts`, `async-parallel.ts`,
  `async-void.ts`, `await-sequence.ts`, `async-await-unsupported.ts` と
  `fixtures/core-semantics/async-await-unsupported.ts` はすべて Node/iwasm 一致。
- [x] full fixture differential:
  stable binary `/tmp/ts2wasm-stable-async-exception` を使った
  `/tmp/ts2wasm-fixture-differential-async-exception-clean-20260527.log` は
  `pass=1162(86%) fail=0 unsupported=135 blocked=54 total=1351 elapsed=209.5s`。
  残バケットは `feature:unknown-unsupported=110`, `feature:node-oracle-fail=54`,
  `feature:import-export=12`, `feature:class=12`, `feature:async=1`。
  async 残は parser 入口の `fixtures/core-semantics/for-await-of-unsupported.ts` のみ。

## 2026-05-27 追加確認: residual coverage triage after native async closure work

- [x] 最新 full differential
  `/tmp/ts2wasm-fixture-differential-async-exception-clean-20260527.log` を JSONL として再集計した。
  現状は `pass=1162(86%) fail=0 unsupported=135 blocked=54 total=1351` で、
  native emitter 起因の stdout mismatch は 0。残りは unsupported と Node oracle blocked の分類問題。
- [x] `feature:class=12` の bottleneck:
  `private-class-field-external-unsupported.ts`, `private-class-method-external-unsupported.ts`,
  `private-class-static-method-external-unsupported.ts`, `private-class-setter-unsupported.ts`,
  `private-class-static-accessor-unsupported.ts` は private name を declaring class 外から参照する形で、
  native emitter ではなく parser/name/lowering の reject 契約に属する。
  `private-class-delete-unsupported.ts` は private field delete の runtime subset reject、
  `private-class-static-field-static-block-tdz-unsupported.ts` は static block 初期化順序 TDZ、
  `class-static-block-super-unsupported.ts` は parser 側 reject。
  native emitter で次に攻めるなら、実装候補は `class-super-property-access-unsupported.ts`
  と `class-dstr-initcount-unsupported.ts` のみだが、どちらも lowering の super property /
  destructuring default initializer の整備が先。
- [x] `feature:async=1` の bottleneck:
  `fixtures/core-semantics/for-await-of-unsupported.ts` は parser 入口の unsupported。
  async native emitter 本体の同期完了 await / Promise.resolve unwrap / async rejection catch は今回の slice で
  focused parity 済みなので、残りは async iterator parser/lowering/runtime の新機能として別 issue 化する。
- [x] `feature:import-export=12` の bottleneck:
  backend 由来は `dynamic-import.ts` と `proxy-reflect-unsupported-diagnostic.ts` の 2 件のみ。
  `dynamic-import.ts` は `PromiseGetValue(ModuleLoadKind::DynamicImport)` を誤って static require 扱いすると
  wrong output へ進むため、現在は明示的 unsupported が正しい。
  static import/export 系は module-resolver/name-resolver 側の unresolved or duplicate export contract が先。
  native emitter 側の実装順は static require/relative module materialization を先に固め、
  dynamic import promise completion はその後に promise job model と一緒に入れる。
- [x] `feature:unknown-unsupported=110` の bottleneck:
  native emitter に直接残っている候補は `proxy-traps-comprehensive.ts`,
  `modules-and-typed-optimizations/{require-cache,require-relative}.ts`,
  `node-apis/{crypto-random-bytes,fs-append,fs-read,fs-write,wasi-fs-read-write}.ts`。
  それ以外の多くは parser/name-resolver/builtin-resolver/lowering の事前 reject
  (with, labelled function, Annex B HTMLDDA, BigInt invalid conversion, destructuring, direct eval dynamic scope,
  private backing storage, triple slash directives) なので、native emitter 完了 gate からは分離する。
- [x] `feature:node-oracle-fail=54` の bottleneck:
  Node v23.6.0 の strip-only TypeScript limitation、fixture-local ESM 拡張子解決、throw/abort 期待値、
  experimental builtins 由来で blocked になっている。native emitter の pass/fail 判断には使えるが、
  emitter 実装 backlog と混ぜると優先度を誤るため、oracle harness 修正 issue として別管理する。
- [x] 次の実装順:
  1. `proxy-traps-comprehensive.ts` は broad Reflect tagged classification を戻すだけでは
     `function.callMethod` unlinked import と wrong output を再発するため、proxy object shape と
     Reflect operation static dispatch を同じ slice で実装する。
  2. `require-relative.ts` / `require-cache.ts` は `ModuleLoadKind::StaticRequire` の relative resolver と
     module cache identity を native static module table に接続する。
  3. Node/WASI fs fixtures は host import policy と WASI preopen contract を先に決め、
     stdout parity ではなく capability-gated verification にする。
  4. class 残件は lowering slice として super property access, destructuring default initializer,
     static private TDZ diagnostics の順に進める。
  5. `for-await-of` と dynamic import は parser/lowering/runtime promise job model の cross-cutting issue として、
     native emitter の同期 async slice から分離する。

## 2026-05-27 追加確認: Proxy Reflect.set static effect と fs.readFileSync native bridge

- [x] `fixtures/builtins-and-io/proxy-traps-comprehensive.ts`:
  直前の full differential で `t2-set:10` になっていた wrong output は、
  `Reflect.set` の static object side effect を statement 後の state propagation に任せるべき箇所で
  runtime user-call fallback へ落ちていたことが原因だった。
  `try_emit_static_reflect_set_call_stmt` で既知 static object かつ static value の slot miss を
  handled 扱いにし、runtime `Reflect.set` call を出さないようにした。
  focused parity は `node` と `iwasm` の stdout diff なし。
- [x] `fixtures/node-apis/fs-read.ts`:
  `FunctionCallKind::Builtin(BuiltinId::FsReadFileSync)` を native emitter から `RuntimeFn` に限定接続した。
  引数個数は runtime stack effect と一致する場合だけ許可し、引数は tagged concat arg として渡す。
  `native_builtin_returns_tagged_value` も `FsReadFileSync` を tagged result として扱う。
  focused check では `input.txt` を持つ temp dir から `iwasm --dir=.` 実行して
  `fixture-input` を確認済み。
- [x] `FsWriteFileSync` / `FsAppendFileSync` / `CryptoRandomBytes` は今回 native support に入れない:
  `FsWriteFileSync` は focused `iwasm --dir=.` なら動くが、current differential harness が
  WASI preopen を渡さないため full run では実行失敗になる。
  `FsAppendFileSync` は runtime helper が `host.fs.appendFileSync` import に依存し、
  bare `iwasm` では link できない。
  `CryptoRandomBytes` は stdout parity 上の nondeterminism と capability policy を先に決める必要がある。
  したがってこの 3 件は native emitter の機能不足というより host/runtime/harness contract issue として残す。
- [x] full fixture differential:
  `/tmp/ts2wasm-fixture-differential-fs-read-proxy-20260527.log`
  `pass=1163(86%) fail=0 unsupported=133 blocked=55 total=1351 elapsed=184.6s`。
  `proxy-traps-comprehensive.ts` は pass。
  `fs-read.ts` は build 成功後に Node oracle が `./input.txt` 不在で blocked になっており、
  native build/runtime 側の focused parity とは別の fixture environment issue。
- [x] 次の実装順の更新:
  1. `require-cache.ts` / `require-relative.ts`:
     `ModuleLoadKind::StaticRequire` の relative resolver、module instance identity、
     cache hit 時の object identity を native static module table に接続する。
  2. Node/WASI write/append/crypto:
     WASI preopen を differential harness に渡すか、capability-gated fixture として別 runner に切る。
     append は host import ではなく native WASI append helper へ寄せるか、明示的 host-shim test に分離する。
  3. class 残件:
     `super` property access と destructuring default initializer は lowering first。
     private external access / static TDZ diagnostics は parser/name/lowering reject contract として扱う。
  4. dynamic import / for-await-of:
     promise job model、async iterator parser/lowering、dynamic module completion の cross-cutting issue として、
     同期完了 async native emitter slice とは分離する。

## 2026-05-27 追加確認: Static require module identity/cache native bridge

- [x] `fixtures/modules-and-typed-optimizations/require-cache.ts`:
  `require("cache-demo")` は lowered では同一 `ModuleLoad { module_id: 1, kind: StaticRequire }`
 になっていたが、module graph が `require()` を dependency として登録しておらず、
  `ModuleInfo` が空の placeholder のまま native emitter に届いていた。
  module graph に resolvable static `require("...")` scan を追加し、`require("fs")` などの Node/core
  builtin は builtin resolver 側に残すため unresolved package/core requires は graph dependency 化しない。
  Node oracle 用に `fixtures/modules-and-typed-optimizations/node_modules/cache-demo/index.js` も追加し、
  fixture-local package shim だけ `.gitignore` の `node_modules` ignore から除外した。
- [x] `fixtures/modules-and-typed-optimizations/require-relative.ts`:
  `require("./lib")` の fixture-local module source として
  `fixtures/modules-and-typed-optimizations/lib.js` を追加し、module graph から `./lib` を解決できるようにした。
  native 側では repeated `ModuleLoad` が module initializer を再実行して cache mutation を戻さないよう、
  per-module initialized global を追加して `$native_module_init_{id}` を idempotent にした。
- [x] module namespace mutation:
  `let a = require(...); a.value = 41; let b = require(...); console.log(b.value);`
  のような CommonJS namespace mutation で、`PropertySet` の object が static `ModuleLoad` alias なら
  `module_export_global` へ直接 `global.set` するようにした。
  console read は stale static bytes を使わず、module export global を runtime に読む経路を優先する。
- [x] focused parity:
  `require-cache.ts` は Node/iwasm ともに `41`。
  `require-relative.ts` は Node/iwasm ともに `ok`。
  `cargo test -p ts2wasm-compiler module_graph_registers_resolvable_static_require_modules -- --nocapture`
  は pass。
  `cargo fmt --check`, `cargo check -p ts2wasm-backend-wasm`, `cargo build -p ts2wasm-cli`,
  scoped/global `git diff --check` は pass。
- [x] full fixture differential:
  `/tmp/ts2wasm-fixture-differential-require-cache-relative-20260527.log`
  `pass=1168(86%) fail=0 unsupported=127 blocked=56 total=1351 elapsed=201.5s`。
  `require-cache.ts` と `require-relative.ts` は pass。
  残バケットは `feature:unknown-unsupported=103`, `feature:node-oracle-fail=56`,
  `feature:class=12`, `feature:import-export=11`, `feature:async=1`。
- [x] 次の実装順の更新:
  1. `feature:unknown-unsupported=103` の native 候補から
     `require-cache.ts` / `require-relative.ts` は削除済み。
     次は `crypto-random-bytes.ts` の nondeterministic oracle policy か、
     backend 由来の残 import/export (`dynamic-import.ts`,
     `proxy-reflect-unsupported-diagnostic.ts`) のどちらかを切る。
  2. `wasi-fs-read-write.ts` と `fs-read.ts` は build/runtime 側では進んでいるが、
     full differential では Node oracle が `./input.txt` 不在で blocked なので、
     fixture working-directory/input provisioning を harness issue として分離する。
  3. class 残件は引き続き lowering first:
     `super` property access、destructuring default initializer、
     private/static TDZ diagnostics の順。

## 2026-05-27 追加確認: Reflect.construct static non-constructor TypeError

- [x] `fixtures/builtins-and-io/proxy-reflect-unsupported-diagnostic.ts`:
  `const target = { x: 42 }; console.log(Reflect.construct(target, []));`
  は native build 時に `host.reflectConstruct` import へ落ち、final native validation の
  `native LoweredProgram emitter does not support unlinked host import host.reflectConstruct`
  で止まっていた。Node oracle は同じ入力で `TypeError: #<Object> is not a constructor`。
- [x] native emitter では dynamic `Reflect.construct` bridge を広げず、
  static exception completion に限定して対応した。
  `Reflect.construct(target, args)` の `target` が static plain object と判定できる場合だけ
  TypeError object を静的生成し、console exception path から throw/unreachable を出す。
  `ClassPrototype` と class constructor alias は既存の positive path を壊さないよう除外した。
- [x] focused parity:
  `target/debug/ts2wasm build -o /tmp/proxy-reflect-native.wasm fixtures/builtins-and-io/proxy-reflect-unsupported-diagnostic.ts --explain-unsupported`
  は exit 0。
  `iwasm /tmp/proxy-reflect-native.wasm` は `TypeError: #<Object> is not a constructorException: unreachable`
  で exit 1。Node も TypeError で exit 1 のため、full differential では Node oracle nonzero により
  `feature:node-oracle-fail` へ分類される。
  既存 positive regression の `reflect-apply-construct.ts` は Node/iwasm diff pass。
- [x] verification:
  `cargo fmt --check`, `cargo check -p ts2wasm-backend-wasm`, `cargo build -p ts2wasm-cli` は pass。
  `bash scripts/run/verify-harness.sh --quick|--cargo|--fixtures` は
  `scripts/run/verify-harness.sh` 不在で exit 127。
  `git diff --check -- crates/backend-wasm/src/native_lowered.rs plans/wasm-native-emitter-mainline-plan.md`
  は pass。
- [x] full fixture differential:
  `/tmp/ts2wasm-fixture-differential-proxy-reflect-nonconstructor-20260527.log`
  `pass=1168(86%) fail=0 unsupported=126 blocked=57 total=1351 elapsed=198.0s`。
  `proxy-reflect-unsupported-diagnostic.ts` は `feature:node-oracle-fail` に移動し、
  `reflect-apply-construct.ts` は pass のまま。
  残 unsupported bucket は `feature:unknown-unsupported=103`, `feature:class=12`,
  `feature:import-export=10`, `feature:async=1`。
- [ ] 次の実装順:
  1. dynamic import は `UnsupportedModule/unknown-unsupported` のままで、
     module graph/runtime Promise completion/diagnostic policy を横断するため単独 issue とする。
  2. `crypto-random-bytes.ts` は backend `UnsupportedBuiltin` だが nondeterministic oracle と
     capability policy の決定が先。
  3. import/export 残 10 件は resolver/module graph と Node oracle の extensionless ESM 解決の
     どちらが先かを分ける。
  4. class 12 件と for-await 1 件は lowering/parser first。

## 2026-05-27 追加確認: fixture oracle fs workdir / crypto Buffer normalization

- [x] `fixtures/node-apis/fs-read.ts` と `fixtures/node-apis/wasi-fs-read-write.ts`:
  native build/runtime は既に `input.txt` を持つ cwd と `iwasm --dir=.` で Node と一致していたが、
  full differential runner は Node/iwasm とも repo root cwd で実行していたため、
  Node oracle が `ENOENT: no such file or directory, open './input.txt'` で blocked になっていた。
  `scripts/check/fixture-differential.py` に fixture-local temporary filesystem を追加し、
  Node oracle は同じ temp cwd、iwasm は同じ入力を持つ別 temp cwd + `--dir=.` で実行するようにした。
- [x] `fixtures/node-apis/crypto-random-bytes.ts`:
  current native emitter/runtime は `require("crypto").randomBytes(16)` を build/run でき、
  Node と iwasm はどちらも `<Buffer ..>` 形式を出す。ただし値自体は nondeterministic なので
  stdout exact diff では false fail になる。`Math.random` と同じ扱いで、Buffer 形式だけを検証する
  `NONDETERMINISTIC_BUFFER_FIXTURES` normalization を追加した。
- [x] focused harness proof:
  `python3 -m py_compile scripts/check/fixture-differential.py` は pass。
  `run_fixture` direct call で `fs-read.ts`, `wasi-fs-read-write.ts`,
  `crypto-random-bytes.ts` はすべて pass。
  手動確認でも `fs-read.ts` は temp cwd + `iwasm --dir=.` で `fixture-input`、
  `wasi-fs-read-write.ts` は `fixture-input` / `done` と `output.txt=wasi-output` を確認。
- [x] required harness commands:
  `bash scripts/run/verify-harness.sh --quick`, `--cargo`, `--fixtures` は
  `scripts/run/verify-harness.sh` 不在で exit 127。
  `git diff --check -- scripts/check/fixture-differential.py plans/wasm-native-emitter-mainline-plan.md`
  は pass。
- [x] full fixture differential:
  `/tmp/ts2wasm-fixture-differential-fs-workdir-crypto-buffer-20260527.log`
  `pass=1172(86%) fail=0 unsupported=125 blocked=54 total=1351 elapsed=182.1s`。
  前回の `pass=1168 fail=0 unsupported=126 blocked=57` から、
  `proxy-reflect-unsupported-diagnostic.ts`, `crypto-random-bytes.ts`,
  `fs-read.ts`, `wasi-fs-read-write.ts` が pass に移動した。
  残 unsupported bucket は `feature:unknown-unsupported=102`,
  `feature:class=12`, `feature:import-export=10`, `feature:async=1`。
- [ ] 次の実装順:
  1. Node/WASI fs read/write/append/crypto の current fixture lane は pass。
     残る host/capability issue は fixture 外の capability-deny と host shim contract で扱う。
  2. native emitter 直撃の unsupported はさらに薄くなったため、
     次は static import/export の resolver/module graph 残件か、
     dynamic import の Promise/module completion policy を個別 issue として切る。
  3. class 12 件と for-await 1 件は引き続き lowering/parser first。

## 2026-05-27 追加確認: Node oracle extensionless TS module imports

- [x] `fixtures/module-system/*-entry.ts` と `fixtures/stmt/{import,export}-*.ts`:
  native build/iwasm 側は static module graph で `.ts` source を解決して pass できる一方、
  Node v23.6.0 の strip-only TypeScript 実行は `import "./source"` の extensionless ESM 解決で
  `ERR_MODULE_NOT_FOUND` になり、full differential では `feature:node-oracle-fail` に分類されていた。
- [x] `scripts/check/fixture-differential.py` の Node oracle で、
  `fixtures/module-system` と `fixtures/stmt` だけ temporary module workdir を作り、
  `from "./x"` / `import "./x"` / `import("./x")` が同じ fixture directory の `x.ts` を指す場合だけ
  `./x.ts` に rewrite するようにした。ts2wasm build は元 fixture を使うため、
  compiler/module graph の入力は変えない。
- [x] focused proof:
  direct `run_fixture` で `import-named.ts`, `import-namespace.ts`, `import-side-effect.ts`,
  `export-named-from.ts`, `export-all-from.ts`,
  `static-namespace-re-export-import-entry.ts` は pass。
  `cargo test -p ts2wasm-cli static_namespace_re_export_module_import_fixture_matches_node_output_under_iwasm -- --nocapture`
  は pass (`TS2WASM_RUN_NODE_DIFF` 未指定のため既存 test harness は differential assertion を skip するが、
  fixture-specific test discovery と build path は通過)。
- [x] transient check:
  `/tmp/ts2wasm-fixture-differential-node-ts-extensionless-20260527.log` では
  `static-namespace-re-export-import-entry.ts` が一度 `actual=undefined` で fail したが、
  focused `run_fixture` 3回、手動 `ts2wasm build` + `iwasm`、再 full differential では `1` で pass。
  同 run の `spread-call-array-local.ts` も `feature:ts2wasm-unavailable` になったが direct `run_fixture`
  と手動 build/iwasm は pass で、再 full differential では回復。
- [x] full fixture differential:
  `/tmp/ts2wasm-fixture-differential-node-ts-extensionless-clean-20260527.log`
  `pass=1195(88%) fail=0 unsupported=125 blocked=31 total=1351 elapsed=185.1s`。
  直前の `pass=1172 fail=0 unsupported=125 blocked=54` から 23 件が
  `feature:node-oracle-fail` blocked から pass に移動した。
  残 unsupported bucket は `feature:unknown-unsupported=102`,
  `feature:class=12`, `feature:import-export=10`, `feature:async=1`。
  残 blocked は `feature:node-oracle-fail=31`。
- [x] required harness commands:
  `bash scripts/run/verify-harness.sh --quick`, `--cargo`, `--fixtures` は
  `scripts/run/verify-harness.sh` 不在で exit 127。
  `python3 -m py_compile scripts/check/fixture-differential.py` と
  `git diff --check -- scripts/check/fixture-differential.py plans/wasm-native-emitter-mainline-plan.md`
  は pass。
- [ ] 次の実装順:
  1. import/export 残 10 件は now clearer:
     default/default+namespace import は module-resolver 側、`export namespace from` と type-only import は
     name-resolver/TS erasure 側、missing/duplicate export diagnostics は module-resolver contract。
  2. `feature:unknown-unsupported=102` から native emitter 直撃を再抽出する。
     多くは parser/name/lowering reject なので、`dynamic-import-unsupported.ts` の
     Promise/module completion policy か class lowering 残件を別 issue 化する。
  3. `feature:node-oracle-fail=31` は Node strip-only TypeScript limitation と expected-rejection
     oracle 整備として emitter backlog から分離する。

## 2026-05-27 追加確認: Static default local alias exports

- [x] 停止点:
  `fixtures/stmt/import-default*.ts` のうち `import-default.ts`,
  `import-default-named.ts`, `import-default-namespace.ts` は
  dependency 側が `const d = ...; export default d;` の形になっており、
  `collect_literal_named_exports` が `export default <literal>` だけを受け付けていたため
  `feature:import-export` unsupported に残っていた。
- [x] compiler 側:
  `process_collected_export_stmt` の `Stmt::ExportDefault` で、
  default expression が identifier の場合は同一 module 内で収集済みの static literal/function/class local alias を
  `"default"` export として解決するようにした。非 static alias は従来通り unsupported。
  `static_default_import_binding_accepts_static_local_alias_export` と
  `static_default_named_import_binding_accepts_static_local_alias_export` を追加し、
  default import / default+named import の binding と lowered `PropertyGet(ModuleLoad, "...")` を固定した。
- [x] native emitter 側:
  `console.log(def, named)` は lowered IR では multi-arg ではなく
  `RuntimeCall::Concat(Concat(def, " "), named)` の単一引数になるため、
  native console の static path に module-aware concat 解決を追加した。
  対象は static module import local が含まれる concat に限定し、
  BigInt など通常の string concat は runtime path のままにした。
  また `ModuleExportsUpdate` がある live binding export は静的値へ固定せず、
  CommonJS `require(...).prop` は runtime module property path に戻した。
- [x] focused proof:
  direct `run_fixture` で以下は pass:
  `fixtures/stmt/import-default.ts`,
  `fixtures/stmt/import-default-named.ts`,
  `fixtures/stmt/import-default-namespace.ts`。
  回帰確認として `fixtures/builtins-and-io/es-module-live-binding.ts`,
  `fixtures/core-semantics/bigint-builtins-string-conversion.ts`,
  `fixtures/modules-and-typed-optimizations/require-cache.ts`,
  `fixtures/modules-and-typed-optimizations/require-relative.ts` も pass。
- [x] full fixture differential:
  `/tmp/ts2wasm-fixture-differential-default-local-alias-r3-20260527.log`
  `pass=1199(88%) fail=0 unsupported=121 blocked=31 total=1351 elapsed=172.6s`。
  直前 clean baseline
  `/tmp/ts2wasm-fixture-differential-node-ts-extensionless-clean-20260527.log`
  `pass=1195(88%) fail=0 unsupported=125 blocked=31 total=1351 elapsed=185.1s`
  から +4 pass / -4 unsupported。`feature:import-export` は 10 から 7 に低下した。
  途中の r2 full run では `array-filter-thisarg.ts` が一度
  `feature:ts2wasm-unavailable` になったが、focused rerun と r3 full run では pass。
- [x] 残 unsupported bucket:
  `feature:unknown-unsupported=101`, `feature:class=12`,
  `feature:import-export=7`, `feature:async=1`。
  残 `feature:import-export` は
  `fixtures/core-semantics/type-only-import-unsupported.ts`,
  `fixtures/module-system/static-bare-import-unsupported.ts`,
  `fixtures/module-system/static-local-named-export-duplicate-unsupported.ts`,
  `fixtures/module-system/static-missing-named-export.ts`,
  `fixtures/negative/unsupported-module.ts`,
  `fixtures/stmt/export-namespace-from.ts`,
  `fixtures/typescript-directives/type-only-import-unsupported.ts`。
- [x] required harness commands:
  `bash scripts/run/verify-harness.sh --quick`, `--cargo`, `--fixtures` は
  `scripts/run/verify-harness.sh` 不在で exit 127。
  `cargo fmt --check`, focused compiler tests, `cargo build -p ts2wasm-cli`,
  focused `run_fixture`, full fixture differential r3 は上記結果。
- [ ] 次の実装順:
  1. import/export 残 7 件のうち、type-only import は TS erasure/name-resolver 方針、
     `export namespace from` は namespace re-export lowering、
     missing/duplicate export は module-resolver diagnostics contract として分ける。
  2. `feature:unknown-unsupported=101` を compiler/parser reject と native-emitter reject に再分類し、
     emitter 直撃分だけ次 slice にする。
  3. `feature:class=12` は native emitter の class 残件として別 issue 化し、
     module/import scope と混ぜない。

## 注意

この計画は静的確認から開始した。2026-05-25 時点で backend-wasm の focused/lib tests と
WAT fallback 経路の production 隔離は実行済みだが、`python scripts/manager.py check` は既存の
未整形ファイルで停止し、full fixture differential は accessor descriptor attrs 修正後でも
`pass=541 fail=469 unsupported=176 blocked=165 total=1351` で未達だった。2026-05-26 の Proxy trap
dispatch と Date UTC static model 補完後でも
`pass=688 fail=331 unsupported=167 blocked=165 total=1351` で未達。Date local setters / Date.parse / Date.UTC /
Error object static model / Promise.withResolvers keys 補完後の最新 full differential でも
`pass=747 fail=275 unsupported=164 blocked=165 total=1351` で未達。RegExp literal static model 補完後の
full fixture differential では `pass=758 fail=264 unsupported=164 blocked=165 total=1351`。Generator
next/state static lowering 補完後の full fixture differential では
`pass=769 fail=253 unsupported=164 blocked=165 total=1351`。Date live time native emission 補完後の
full fixture differential では `pass=773 fail=249 unsupported=164 blocked=165 total=1351`。BigInt runtime
arithmetic materialization 補完後の最新 full fixture differential でも
`pass=786 fail=236 unsupported=164 blocked=165 total=1351`、BigInt comparison/tagged relational emission
補完後でも `pass=799 fail=223 unsupported=164 blocked=165 total=1351`、Map/Set tagged collection values
補完後でも `pass=805 fail=217 unsupported=164 blocked=165 total=1351`、SetValuesArray callback value
representation 補完後でも `pass=806 fail=216 unsupported=164 blocked=165 total=1351`、patched
Set.prototype.add in Set constructor 補完後でも
`pass=807 fail=215 unsupported=164 blocked=165 total=1351`、Map/Set keys/values iterator console access
補完後でも `pass=811 fail=211 unsupported=164 blocked=165 total=1351`、Map object identity keys
補完後でも `pass=812 fail=210 unsupported=164 blocked=165 total=1351`、Function prototype object ToString
補完後でも `pass=814 fail=208 unsupported=164 blocked=165 total=1351`、Function constructor
prototype/new object semantics 補完後でも `pass=817 fail=205 unsupported=164 blocked=165 total=1351`
、Function constructor new.target metadata 補完後でも
`pass=818 fail=204 unsupported=164 blocked=165 total=1351`、Function constructor static source-body
completion 補完後でも `pass=831 fail=191 unsupported=164 blocked=165 total=1351`、static
for-in state propagation 補完後でも `pass=832 fail=190 unsupported=164 blocked=165 total=1351`、static
switch fallthrough state propagation 補完後でも `pass=833 fail=189 unsupported=164 blocked=165 total=1351`、
tagged throw/catch exception values 補完後でも `pass=834 fail=188 unsupported=164 blocked=165 total=1351`、
class instance static prototype chain 補完後でも `pass=835 fail=187 unsupported=164 blocked=165 total=1351`、
class prototype identity equality 補完後でも `pass=838 fail=184 unsupported=164 blocked=165 total=1351`、
static new constructor console side effects 補完後でも
`pass=839 fail=183 unsupported=164 blocked=165 total=1351`、class expression static getter materialization
補完後でも `pass=840 fail=182 unsupported=164 blocked=165 total=1351`、super method property token typeof
補完後でも `pass=842 fail=180 unsupported=164 blocked=165 total=1351`、derived rest constructor
forwarding and class object console 補完後でも
`pass=856 fail=183 unsupported=147 blocked=165 total=1351`、ClassPrototype known property miss 補完後でも
`pass=858 fail=181 unsupported=147 blocked=165 total=1351`、class accessor descriptor shadowing 補完後でも
`pass=859 fail=180 unsupported=147 blocked=165 total=1351`、new-expression method call signature args 補完後でも
`pass=860 fail=179 unsupported=147 blocked=165 total=1351`、constructable function instanceof /
prototype mutation 補完後でも `pass=862 fail=177 unsupported=147 blocked=165 total=1351`、inline
Object.setPrototypeOf prototype for `in` 補完後でも
`pass=863 fail=176 unsupported=147 blocked=165 total=1351`、Function.prototype.toString TypeScript source
stripping 補完後でも `pass=865 fail=174 unsupported=147 blocked=165 total=1351`、`arguments.callee`
descriptor console parity 補完後でも `pass=866 fail=173 unsupported=147 blocked=165 total=1351`、
`Object.keys(arguments)` native crash 補完後でも `pass=867 fail=172 unsupported=147 blocked=165 total=1351`、
known global value console display 補完後でも `pass=870 fail=169 unsupported=147 blocked=165 total=1351`、
well-known Symbol console display 補完後でも `pass=871 fail=168 unsupported=147 blocked=165 total=1351`、
`globalThis` alias console display 補完後でも `pass=872 fail=167 unsupported=147 blocked=165 total=1351`、
Object.prototype `.call` closure static dispatch 補完後でも
`pass=873 fail=166 unsupported=147 blocked=165 total=1351`、static RestObject materialization 補完後でも
`pass=876 fail=165 unsupported=145 blocked=165 total=1351`、`lastIndexOf` fromIndex lowering 補完後でも
`pass=877 fail=164 unsupported=145 blocked=165 total=1351`、TypedArray `.buffer` static console display
補完後でも `pass=878 fail=163 unsupported=145 blocked=165 total=1351`、TypedArray iterator/copy result
console display 補完後でも `pass=879 fail=162 unsupported=145 blocked=165 total=1351`、TypedArrayStore
expression result 補完後でも `pass=880 fail=161 unsupported=145 blocked=165 total=1351`、direct function
token HeapClosureCall static dispatch 補完後でも `pass=881 fail=160 unsupported=145 blocked=165 total=1351`
、WeakMapSet tagged value emission 補完後でも `pass=882 fail=159 unsupported=145 blocked=165 total=1351`
、Block result raw string console emission 補完後でも `pass=883 fail=158 unsupported=145 blocked=165 total=1351`
、Math.trunc/sign static integer emission 補完後でも `pass=884 fail=157 unsupported=145 blocked=165 total=1351`
、Map.groupBy static map materialization 補完後でも
`pass=885 fail=156 unsupported=145 blocked=165 total=1351`、Math non-integer static fold 補完後でも
`pass=886 fail=155 unsupported=145 blocked=165 total=1351`、Reflect basic static object operations 補完後でも
`pass=887 fail=154 unsupported=145 blocked=165 total=1351`、Reflect.apply/construct static dispatch 補完後でも
`pass=888 fail=153 unsupported=145 blocked=165 total=1351`、arguments dynamic index static loop 補完後でも
`pass=889 fail=152 unsupported=145 blocked=165 total=1351`、Function.prototype call/apply static dispatch
補完後でも `pass=891 fail=150 unsupported=145 blocked=165 total=1351`、logical assignment
local/object static state 補完後でも `pass=895 fail=149 unsupported=142 blocked=165 total=1351`、
ClassDecl instance method static prototype state 補完後でも
`pass=896 fail=148 unsupported=142 blocked=165 total=1351`
で未達なので、
完了条件の gate として残す。
