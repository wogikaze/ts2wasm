# Compatibility and semantics

この文書は TypeScript 構文、TypeScript 型、JavaScript 実行意味論、module/npm ecosystem の扱いをまとめる。

## TypeScript 構文対応

構文対応は、単純な文法から始めるが、対応予定を削らない。未対応構文は明示的な診断を出し、テスト上も `expected unsupported` として管理する。

構文の基準は TypeScript compiler が受理する TypeScript と、実行時の ECMAScript semantics である。AssemblyScript 固有の `i32`、`i64`、`f32`、`usize`、`changetype`、明示メモリ操作 API、AssemblyScript 標準ライブラリは入力言語として扱わない。内部最適化で primitive 表現を使う場合も、ユーザー可視の構文・型は TypeScript に限定する。

構文ごとの対応方針と実装状況の詳細は `docs/language-reference/typescript-features.md` および `docs/language-reference/javascript-features.md` を参照。

初期段階で重要なのは、「簡単な構文しか対応しない」ことではなく、「構文ごとの未対応理由を潰せる形で管理する」ことである。たとえば `async/await` が未対応なら、parser が読めないのか、IR が表現できないのか、runtime に Promise がないのか、host event loop がないのかを分ける。

## TypeScript 型対応

TypeScript の型は実行時に消えるが、コンパイル時には重要である。型情報を利用することで、WASM 生成の品質を上げられる。たとえば、`number[]` と分かる配列は generic object array より効率よく扱える可能性がある。`string` と分かる値への `+` は文字列結合に落とせる。`boolean` と分かる値の branch は truthiness 判定を単純化できる。

ただし、TypeScript の型は sound ではない。`any`、型アサーション、構造的部分型、union、intersection、generic、conditional type などがあるため、型だけを信じて runtime check を完全に消すと壊れる。optimization level と semantic safety mode の対応は `docs/11-shared-definitions.md` を正とする。`unsafe-fast` を標準動作にしてはいけない。これは性能を捨てないためではなく、性能改善と互換性維持を分離するためである。

TypeScript 型は、最適化のヒントであって別言語への opt-in ではない。`number` を内部的に `i32` fast path へ落とす場合は、範囲解析、overflow、`NaN`、`-0`、`Infinity`、property escape、function call boundary を検査する。検査できない場合は JS `number` semantics に戻す。

| 入力 | TypeScript としての扱い | 方針 |
|---|---|---|
| `let x: number = 1` | 標準 TypeScript | fast path 候補 |
| AssemblyScript 固有型名 | TypeScript 上は通常の型参照 | 組み込み primitive 型として特別扱いしない |
| `arr: number[]` | 標準 TypeScript | packed number array 候補 |
| `arr: Int32Array` | 標準 JS typed array | typed array runtime として扱う |
| AssemblyScript intrinsic 風 API | TypeScript 上は通常の識別子参照 | intrinsic として扱わない |

## JavaScript 意味論

このプロジェクトの難所は TypeScript 構文ではなく JavaScript 意味論である。特に、`this`、prototype、property lookup、dynamic object shape、truthiness、`==`、`===`、`NaN`、`-0`、exception、closure、`eval`、`with`、getter / setter、Proxy などは WASM への直接変換を難しくする。

対応方針として、まず通常の TypeScript コードでよく使われる範囲を正確に実装する。`eval` や `with` のような最適化を破壊する機能は、最初から最重要扱いにはしない。ただし、仕様から削除はしない。明示的に `unsupported-dynamic-code` として扱う。

WAMR は multi-thread (wasi-threads)、socket API (Berkeley/Posix Socket) をサポートしており、WASI 経由でネットワーク機能や並列処理も実行可能である。wasm-tools は既に Wasm GC、reference-types、function-references、multi-memory、multi-value、SIMD、tail-call、threads などの提案を実装しており（多くは Stage 4+）、これらを活用することでより効率的な実装が可能になる。

| 機能              | 方針                                        |
| --------------- | ----------------------------------------- |
| `this`          | call site ごとに receiver を明示して IR に落とす      |
| prototype       | class 対応後に object model に統合               |
| property lookup | string key lookup から開始し、shape cache を後で追加 |
| `==`            | runtime helper で primitive coercion を実装し、object ToPrimitive は object model 安定後に追加 |
| `===`           | primitive fast path を用意                   |
| `NaN` / `-0`    | number semantics のテスト対象にする                |
| exception       | runtime stack と wasm exception の両案を検討     |
| `eval`          | 初期非対応、診断必須                                |
| `with`          | 初期非対応、診断必須                                |
| Proxy           | 初期非対応、object model 安定後に検討                 |

Compatibility evidence distinguishes syntax/build support from semantic parity. A feature that parses, lowers, or builds with placeholder behavior is recorded as `部分実装` in `docs/language-reference/javascript-features.md` and must have an open issue with Node differential acceptance criteria before it can count toward semantic gates. Current placeholder or partial semantic trackers include issues 207-214 for `instanceof`, switch fall-through, labeled control flow, arrow closures, `this`, rest parameters, template interpolation, and string methods.

`Math.random` is capability-gated for standalone WASI output: use of `Math.random()` imports `wasi_snapshot_preview1.random_get`, sets `wasi.random: true` in the capability manifest, and remains valid under host-deny because it does not require a Node host import. The current tagged-int runtime can only expose an integer-backed random payload; full ECMAScript fractional double parity is part of the broader number representation model, not a silent deterministic placeholder.

Abstract equality (`==` / `!=`) supports the current primitive runtime value set: `undefined`, `null`, booleans, tagged integer numbers, and strings that coerce to tagged integers. Full object `ToPrimitive`, floating point, `NaN`, and `-0` behavior remain tied to the broader object and number-model work.

BigInt は heap object representation として設計済みだが、runtime 値と操作は段階実装である。BigInt literal runtime values は issue 259、arithmetic は issue 260、equality/comparison/coercion の最初の境界は issue 261、builtin/string conversion は issue 262 が所有する。BigInt と Number の arithmetic は暗黙変換せず TypeError path にする。issue-261 では BigInt 同士の mathematical value strict equality、abstract equality、relational comparison と、Number など非 BigInt との `===` / `!==` false/true 境界を実装済みである。Literal BigInt/String `==` / `!=` は supported StringToBigInt subset で fold し、invalid string は Node と同じ false/true 境界にする。Literal BigInt/Boolean `==` / `!=` は `false -> 0n`、`true -> 1n` の境界で fold する。Literal BigInt/Number `==` / `!=` は representable tagged-int number literals と unary-negative integer literals（`-0` を含む）だけを fold する。Literal BigInt/Number `<` / `<=` / `>` / `>=` も同じ静的 integer subset だけを fold する。Literal BigInt/nullish `==` / `!=` は Node と同じ false/true 境界に fold する。その他の mixed BigInt abstract equality と relational comparison は ECMA-262 の BigInt/Number/String/Boolean coercion rules が必要なため、broader number model limits (`NaN`, `Infinity`, fractional number tokens) は issue 281、dynamic StringToBigInt parsing と object `ToPrimitive` を含む runtime mixed coercion は issue 282 に分割し、statically visible case は issue-linked diagnostic、runtime-only case は trap として止める。

## Array / object semantics（実装済み範囲）

現行 lowering がカバーする array と object の semantics 要件を記録する。

| 機能 | 実装状態 | 備考 |
|---|---|---|
| array literal `[e0, e1, ...]` | 実装済み | heap block `[i32 len, elem₀, ...]` tagged `ptr\|5` |
| numeric array index `arr[n]` | 実装済み | tag check あり; 範囲外は `undefined` |
| `arr.length` | 実装済み | tag check あり; 非 array/string は `undefined` |
| `str.length` | 実装済み (basic) | UTF-8 byte storage を使う。完全な UTF-16 code unit parity は追跡中 |
| object literal `{k: v}` | 実装済み | heap block `[i32 count, (key_raw, value)×n]` tagged `ptr\|7` |
| data property read `obj.key` | 実装済み | reverse scan; last duplicate key wins (JS 仕様) |
| dynamic property key | 実装済み (basic) | string key による `obj[key]` / assignment をサポート |
| prototype / method call | 部分実装 (basic) | `[[Prototype]]` slot と method receiver の basic path をサポート。`instanceof` full traversal and `this` receiver parity are tracked by issues 207 and 211 |
| non-ASCII string literal | 実装済み (basic) | UTF-8 byte storage。decode/encode runtime helper は追跡中 |
| object literal key (string literal) | 未実装 | `{key: v}` の key は identifier only; `{"x": v}` は parse error |
| `obj["key"]` computed property | 実装済み (basic) | object property lookup path を使う |
| heap OOM check | 実装済み | `$alloc_heap` は memory.size を検査し、超過時に trap する |

`$property_get` の reverse scan により、`{a:1, a:2}.a === 2` が成立する (JS 仕様準拠)。

> **残タスク**: RuntimeLinkPlan と WatEmitter の分離、AST に一貫した `Span`、BuiltinResolver pass の整理、
> capability manifest の本番出力などは、`current-state.md` と `docs/11-shared-definitions.md` の gate / issue で追跡する。

## Standard idiom semantics

docs/03 の WASI-compatible idiom は、host shim 不要という意味で standalone 候補である。ただし、そこに含まれる JS 意味論は runtime 側で別途実装されている必要がある。

| Idiom | 必要な JS semantics | 未実装時の扱い |
|---|---|---|
| `input.trim()` | `String.prototype.trim` | `unsupported-string-trim` |
| `input.split(/\s+/)` | RegExp literal、RegExp split | `unsupported-regexp-split` |
| `.map(Number)` | Array iteration、callback call、`Number` conversion | `unsupported-array-map` または `unsupported-number-constructor` |
| `console.log(sum)` | string conversion、WASI stdout | host shim 不要 |

この分類により、「Node.js host が不要」と「JS runtime semantics が実装済み」を混同しない。前者は capability の問題であり、後者は互換性・runtime 実装の問題である。

## 追加設計: module and npm ecosystem

既存 TypeScript/JavaScript 資産を活用するには、構文だけでなく module 解決と package ecosystem の扱いが必要である。初期から npm 全体を扱う必要はないが、段階を明確にする。

| Phase | Scope |
|---|---|
| Phase 1 | single file only |
| Phase 2 | relative `import` / `export` |
| Phase 3 | builtin module lowering: `fs`, `path`, `process`, `buffer` |
| Phase 4 | `package.json` based resolution |
| Phase 5 | selected npm package compatibility |

扱う必要がある論点:

| Topic | Policy |
|---|---|
| ESM / CommonJS | 段階的に両対応。最初は静的に解けるものを優先 |
| `require()` | literal require を compile-time builtin resolution から開始 |
| dynamic require | 初期は unsupported-dynamic-module |
| `package.json exports` | npm package 対応段階で導入 |
| native addon | 初期非対応。host capability として扱う |
| optional dependency | package compatibility 段階で明示管理 |
| side-effect import | module graph に side-effect bit を持たせる |
| tree shaking | host shim trimming とは別に module DCE として扱う |
