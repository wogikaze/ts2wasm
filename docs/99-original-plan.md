# TS transpiler to WASM

## 概要

本プロジェクトは、TypeScript を WebAssembly にトランスパイルする処理系を実装する。目的は、TypeScript / JavaScript の既存資産を可能な限り活用しながら、Node.js や Bun の上でしか動かない実行モデルから離れ、WASI や iwasm などの WebAssembly 実行環境で動作可能な `.wasm` を生成することである。

このプロジェクトは「TypeScript を別言語に寄せる」ものではない。TypeScript の構文、JavaScript の実行意味論、既存テスト資産、既存エコシステムを尊重しつつ、出力先を WebAssembly にする。したがって、単に TypeScript 風の新言語を作るのではなく、TypeScript / JavaScript として書かれたコードを、できる限りそのまま WebAssembly 実行系に持ち込むことを目標にする。

ただし、すべての機能を Node.js API に逃がすことは禁止する。Node.js は補助的な host としてのみ使う。標準入出力、ファイルシステム、基本的なメモリ管理、数値演算、配列、オブジェクト、文字列、関数呼び出し、制御構文などは、可能な限り生成された WASM とランタイムライブラリ上で実行する。

Node.js API が必要になる場合は、WASM 側から明示的な import として呼び出す。つまり、Node.js は「実行本体」ではなく「不足 API の host provider」である。Node.js に処理を丸投げする設計、JavaScript ソースをそのまま Node に渡す設計、WASM の中に JS 実行のふりをした薄い wrapper だけを置く設計は、このプロジェクトの目的に反する。

## 目標

このプロジェクトの中心目標は、TypeScript を `.wasm` に変換し、WASI 環境または最小限の Node.js host 環境で実行できるようにすることである。

最終的には、Node.js API に依存しないコードは iwasm 上で直接実行できるようにする。たとえば、標準入力を読み、計算し、標準出力に書く CLI プログラムは、Node.js を介さず `.wasm` 単体で動作することを目指す。一方で、`process.env`、高度な OS 情報、Node 固有の path 解決、非同期 I/O、ネットワーク、タイマーなど、WASI だけでは十分に扱えない API は、Node.js host を明示的に併用する。

このプロジェクトでは、TypeScript の型情報を単なるコメントとして捨てるのではなく、最適化・診断・変換戦略に利用する。TypeScript の型は実行時には消えるが、コンパイル時には豊富な情報を与える。これを使って、数値演算、配列アクセス、オブジェクト形状、関数呼び出し、クラス、ジェネリクスの単相化、不要な runtime check の削減などに活かす。

## 非目標

このプロジェクトは、TypeScript 互換の別言語を作るものではない。構文を都合よく変えたり、JavaScript の面倒な意味論を削ったり、既存 TypeScript コードを書き直さないと動かない前提にしたりしない。

また、初期段階で対応できない機能が出たとしても、それを仕様から削除したことにはしない。未対応機能は `unsupported`、`planned`、`blocked-by-runtime`、`requires-host-api` のように状態を分けて管理する。これは機能を減らすためではなく、実装順序と失敗理由を明確にするためである。

さらに、性能を捨てる設計は採用しない。初期実装が遅くてもよいが、測定しない、比較しない、改善できない構造にすることは避ける。Node.js や Bun に常に勝てるとは限らないが、競プロ・数値計算・短命 CLI・標準入出力中心のプログラムでは、最終的に Node.js / Bun より速い実行を目指す。特に、型が静的に確定するコード、配列と数値演算が中心のコード、GC 負荷が小さいコード、host API 呼び出しが少ないコードでは、WASM backend が明確に勝つことを性能目標にする。

## 基本方針

このプロジェクトの実行モデルは、次の三層で考える。

| 層               | 役割                                             | Node.js 依存         |
| --------------- | ---------------------------------------------- | ------------------ |
| generated wasm  | TypeScript から生成される実行本体                         | なし、または明示 import のみ |
| ts2wasm runtime | JS 値、文字列、配列、オブジェクト、例外、GC 補助などを提供する WASM 側ランタイム | なし                 |
| host shim       | WASI では足りない API を提供する薄い層                       | 必要な場合のみ            |

重要なのは、host shim を薄く保つことである。たとえば `console.log` を呼ぶために Node.js を使うのは許容できるが、関数本体の実行やオブジェクト操作を Node.js に戻すのは許容しない。`fs.readFileSync` を Node host 経由で提供する場合でも、読み込んだデータの処理は WASM 側で行う。

## 実行ターゲット

第一ターゲットは WASI 対応の core wasm とする。iwasm での実行を重視するため、初期段階では Wasm GC や Component Model に強く依存しすぎない。線形メモリ上に JS 値表現を実装し、WASI の `fd_read`、`fd_write`、ファイル API、環境変数、引数などを扱う。

第二ターゲットとして、Node.js host 付き WASM を用意する。このターゲットでは、WASI だけでは表現しにくい API を Node.js import として補う。`process`、一部の `fs`、`path`、`Buffer`、タイマー、非同期処理などはこの層で段階的に扱う。

第三ターゲットとして、将来的に Wasm GC / Component Model / WIT を利用したより型付きの host interface を検討する。ただし、これは初期の iwasm 実行可能性を壊さない範囲で進める。iwasm で動く `.wasm` と、より高機能な runtime 向け `.wasm` は、同じ compiler pipeline から別 backend として出す。

## API 対応方針

API は「standalone で動く API」と「Node.js host が必要な API」に分ける。

standalone で動く API は、WASI と WASM runtime だけで実行する。標準入出力、基本的なファイル読み書き、引数、環境変数、文字列処理、数値処理、配列、Map / Set の基本操作、JSON、TextEncoder / TextDecoder の基礎部分はこの領域に入る。

Node.js 併用 API は、WASI 単体では扱いにくいものを対象にする。`process`、Node 固有の `fs` 挙動、`Buffer`、`crypto`、`path` の細部、イベントループ、タイマー、非同期 I/O、ネットワークなどが該当する。ただし、これらも「Node.js で全部処理する」のではなく、WASM 側から必要な host function を呼ぶ形にする。

| API 領域  | standalone WASI | Node host 併用 | 方針                                        |
| ------- | --------------: | -----------: | ----------------------------------------- |
| stdio   |              対応 |           不要 | `console.log` も最終的には WASI `fd_write` に落とす |
| argv    |              対応 |           不要 | WASI args を runtime に渡す                   |
| env     |            部分対応 |      必要な場合あり | `process.env` 互換層を runtime 上に実装           |
| fs      |            部分対応 | Node 固有挙動は併用 | 同期 API から優先                               |
| path    |            対応可能 | Node 完全互換は併用 | POSIX / Windows 差分を明示管理                   |
| process |            部分対応 |           併用 | `exit`, `cwd`, `env`, `argv` から開始         |
| Buffer  |      runtime 実装 |     必要に応じて併用 | `Uint8Array` との関係を明確化                     |
| crypto  |             難しい |           併用 | WASI random と Node crypto を分離             |
| timer   |             難しい |           併用 | event loop 設計後に対応                         |
| network |             非初期 |           併用 | host capability として扱う                     |

| API / idiom                        | 実行方法                                       | Node.js host 必要性 |
| ---------------------------------- | ------------------------------------------ | ---------------: |
| `fs.readFileSync(0, "utf8")`       | WASI `fd_read` + WASM runtime UTF-8 decode |               不要 |
| `fs.readFileSync("/path", "utf8")` | WASI preopen dir 経由の file read             |          条件付きで不要 |
| `console.log(...)`                 | WASI `fd_write`                            |               不要 |
| `process.argv`                     | WASI args                                  |               不要 |
| `process.env`                      | WASI environ                               |               不要 |
| `process.cwd()`                    | WASI だけでは弱い。preopen / host policy 依存       |            場合による |
| `fs.existsSync`, `statSync`        | WASI filesystem API に対応可能                  |             条件付き |
| `path.join`                        | WASM runtime builtin                       |               不要 |
| `Buffer.from`                      | WASM runtime builtin                       |               不要 |
| `setTimeout`                       | WASI だけでは不足                                |  Node host などが必要 |
| network                            | WASI Preview 1 では不足                        |         host が必要 |

## WASI-compatible Node Idioms

本プロジェクトでは、Node.js の API 名で書かれているコードであっても、必ず Node.js host を必要とするとは限らない。`fs.readFileSync(0, "utf8")`、`console.log`、`process.argv`、`process.env` のような idiom は、WASI の標準機能に対応付けられるため、Node.js なしで `.wasm` 単体実行できる。

この場合、`require("fs")` を実行時に Node.js の module system へ渡すのではなく、compiler が builtin module として解決する。`readFileSync(0, "utf8")` は WASI `fd_read` に lowering され、UTF-8 decoding は WASM runtime 側で行う。

したがって、次のコードは Node.js host なしで実行できる対象に含める。

```ts
const input = require("fs").readFileSync(0, "utf8");
const nums = input.trim().split(/\s+/).map(Number);

let sum = 0;
for (let i = 0; i < nums.length; i++) {
    sum += nums[i];
}

console.log(sum);
```

このコードの理想的な実行分担はこうなる。

| 処理                        | 実行場所                            |
| ------------------------- | ------------------------------- |
| `require("fs")` の解決       | compile-time builtin resolution |
| `readFileSync(0, "utf8")` | WASI `fd_read` + WASM runtime   |
| `trim`                    | WASM runtime                    |
| `split(/\s+/)`            | WASM runtime                    |
| `map(Number)`             | WASM runtime / 最適化後 inline      |
| `for` loop                | generated WASM                  |
| `console.log`             | WASI `fd_write`                 |

この場合、JavaScript host shim は不要です。必要なのは WASI imports だけで、iwasm がそれを提供する。

> 本プロジェクトは、Node.js 風の API を使った TypeScript コードであっても、WASI に対応可能な idiom は Node.js host に逃がさず、WASI import と WASM runtime に lowering する。`fs.readFileSync(0, "utf8")`、`console.log`、`process.argv`、`process.env` などは standalone WASI 実行の対象とする。JavaScript host shim は、WASI で表現できない API に限って使用する。

## Host Shim Trimming

本プロジェクトでは、host shim を固定の巨大 runtime として配布しない。compiler はソースコードと lowering 結果を解析し、実行に必要な host capability だけを manifest として列挙する。その manifest に基づいて、必要な host shim 関数だけを生成または link する。

WASI に lowering できる API は Node.js host shim に含めない。たとえば `console.log`、`fs.readFileSync(0, "utf8")`、`process.argv`、`process.env` は standalone WASI execution の対象であり、Node.js shim を要求しない。

Node.js host shim は、WASI では表現できない API に限って使う。さらに、その場合でも `fs` 全体、`process` 全体、`node:*` 全体をまとめて import するのではなく、必要な関数単位で import する。

```text
bad:
  import node_fs_all
  import node_process_all
  import node_crypto_all

good:
  import host.timer.set_timeout
  import host.crypto.random_bytes
  import host.fs.watch
```

これにより、生成物は次の性質を持つ。

| 性質          | 内容                              |
| ----------- | ------------------------------- |
| 最小依存        | 使っていない host API は出力に含めない        |
| 監査しやすい      | どの外部 API に依存しているか manifest で分かる |
| iwasm 判定が容易 | Node shim が空なら standalone 実行可能  |
| 性能劣化を防ぐ     | 不要な JS bridge を通らない             |
| セキュリティが強い   | 不要な capability を渡さない            |

manifest はこういう形がよいです。

```json
{
  "target": "wasm32-wasi",
  "standalone": true,
  "wasi": {
    "stdin": true,
    "stdout": true,
    "stderr": false,
    "args": false,
    "env": false,
    "filesystem": false
  },
  "node_host": {
    "required": false,
    "imports": []
  }
}
```

Node host が必要な場合はこうです。

```json
{
  "target": "wasm32-wasi+node-host",
  "standalone": false,
  "wasi": {
    "stdout": true
  },
  "node_host": {
    "required": true,
    "imports": [
      "timer.setTimeout"
    ]
  }
}
```

compiler の内部では、API ごとに capability を割り当てると管理しやすいです。

| ソース上の API                    | capability                          | standalone |
| ---------------------------- | ----------------------------------- | ---------: |
| `console.log`                | `wasi.stdout`                       |        yes |
| `console.error`              | `wasi.stderr`                       |        yes |
| `fs.readFileSync(0, "utf8")` | `wasi.stdin`                        |        yes |
| `process.argv`               | `wasi.args`                         |        yes |
| `process.env`                | `wasi.env`                          |        yes |
| `fs.readFileSync(path)`      | `wasi.filesystem.read`              |       条件付き |
| `setTimeout`                 | `host.timer`                        |         no |
| `crypto.randomBytes`         | `host.crypto.random` or WASI random |       条件付き |
| `fetch`                      | `host.http`                         |         no |

設計としては、host shim trimming は DCE というより capability-based linking に近いです。単に「使っていない JS 関数を消す」だけではなく、「そもそもどの外部能力を要求しているか」を compiler が把握する。

なので、提案書にはこの一文を入れるとよいです。

> Host shim は monolithic にしない。compiler は必要な host capability を解析し、WASI で表現できる API は WASI import に lowering し、WASI で表現できない API のみを関数単位で Node.js host shim として生成する。未使用の host shim 関数は出力に含めない。

## コンパイラ構成

コンパイラは、frontend、semantic layer、lowering、runtime binding、wasm backend の五段階に分ける。

Frontend は TypeScript / JavaScript の構文を読む。初期段階では既存 parser を oracle として使ってよいが、本体を Node.js の TypeScript compiler API に完全依存させる設計にはしない。`tsc` の parser / checker は比較対象、テスト oracle、差分検出のために利用する。プロダクションの変換器は、最終的には WASM に向いた IR を持つ独自 pipeline として成立させる。

Semantic layer では、TypeScript の型注釈、推論結果、制御フロー、スコープ、symbol、module 解決を扱う。TypeScript の型システムを完全再実装するのは重いが、型情報を無視すると最適化も診断も弱くなる。したがって、初期段階では型を「実行に必要な情報」と「診断に必要な情報」に分ける。実行に必要な情報は優先して compiler pipeline に取り込み、診断互換は段階的に強化する。

Lowering では、TypeScript / JavaScript の意味論を WASM に落としやすい IR に変換する。ここで重要になるのは、JavaScript の値表現、truthiness、`undefined`、`null`、number、string、object、array、function、closure、prototype、class、exception の扱いである。単純な型付き言語よりも runtime の責任が大きいため、IR には JS semantics を表現できる命令を持たせる。

WASM backend は、IR から `.wasm` を生成する。初期は linear memory ベースで実装し、iwasm で動くことを優先する。値表現は `i32` handle または tagged value を基本とし、runtime heap に object / string / array / closure を置く。将来的に Wasm GC backend を追加する場合も、IR は共有する。

## 値表現

TypeScript は実行時には JavaScript であるため、値表現は JS 値の多様性を扱える必要がある。`number`、`string`、`boolean`、`undefined`、`null`、`object`、`function`、`symbol`、`bigint` を同じ実行系で扱う必要がある。

初期実装では、すべての値を `JsValue` として扱う。`JsValue` は tagged representation にする。小さい整数や boolean、null、undefined は immediate value として表現し、string、object、array、function は heap object への handle として表現する。

| 値         | 表現方針                | 初期対応 |
| --------- | ------------------- | ---: |
| undefined | immediate tag       |   必須 |
| null      | immediate tag       |   必須 |
| boolean   | immediate tag       |   必須 |
| number    | immediate または boxed |   必須 |
| string    | heap object         |   必須 |
| object    | heap object         |   必須 |
| array     | heap object         |   必須 |
| function  | closure object      |   必須 |
| bigint    | heap object         | 段階対応 |
| symbol    | interned value      | 段階対応 |

性能を考えると、すべてを boxed value にすると遅くなる。そのため、型情報が十分にある場合は fast path を生成する。たとえば `let x: i32` 相当の扱いができる数値演算は、runtime call ではなく WASM の `i32.add` や `f64.add` に落とす。ただし、JavaScript の `number` は基本的に IEEE 754 double なので、TypeScript の型注釈だけで勝手に整数意味論に変えてはいけない。最適化は意味論を壊さない範囲で行う。

## メモリ管理

初期 runtime は linear memory 上に heap を実装する。GC は最初から完全な高性能 GC を作る必要はないが、object / string / array / closure を扱う以上、メモリ管理の設計は避けられない。

最初の実装では、単純な mark-and-sweep または arena + 明示 lifetime 管理に近い方式を採用する。CLI や短命プログラムでは arena 的な管理でも十分に動くが、長時間動くプログラムや Node host と連携するプログラムでは回収が必要になる。したがって、初期から heap object の header、type tag、mark bit、size、field layout を決めておく。

メモリ管理は後から差し替え可能にする。runtime の public interface が `alloc_string`、`alloc_object`、`get_prop`、`set_prop`、`call_function` のように整理されていれば、内部 GC の改善は compiler 全体に波及しにくい。

## TypeScript 構文対応

構文対応は、単純な文法から始めるが、対応予定を削らない。未対応構文は明示的な診断を出し、テスト上も `expected unsupported` として管理する。

| 構文                                |      初期 |      中期 |  最終 |
| --------------------------------- | ------: | ------: | --: |
| `let` / `const` / `var`           |      対応 |      対応 |  対応 |
| number / string / boolean literal |      対応 |      対応 |  対応 |
| object literal                    |      対応 |      対応 |  対応 |
| array literal                     |      対応 |      対応 |  対応 |
| function                          |      対応 |      対応 |  対応 |
| arrow function                    |      対応 |      対応 |  対応 |
| closure                           |      部分 |      対応 |  対応 |
| class                             |      部分 |      対応 |  対応 |
| interface                         |     型のみ |     型のみ | 型のみ |
| type alias                        |     型のみ |     型のみ | 型のみ |
| generics                          | 型消去から開始 |    特化検討 |  対応 |
| enum                              |      対応 |      対応 |  対応 |
| namespace                         |     非初期 |      部分 |  対応 |
| module import/export              |      部分 |      対応 |  対応 |
| async/await                       |     非初期 | host 併用 |  対応 |
| exception                         |      部分 |      対応 |  対応 |
| destructuring                     |      部分 |      対応 |  対応 |
| spread/rest                       |      部分 |      対応 |  対応 |
| optional chaining                 |      対応 |      対応 |  対応 |
| nullish coalescing                |      対応 |      対応 |  対応 |

初期段階で重要なのは、「簡単な構文しか対応しない」ことではなく、「構文ごとの未対応理由を潰せる形で管理する」ことである。たとえば `async/await` が未対応なら、parser が読めないのか、IR が表現できないのか、runtime に Promise がないのか、host event loop がないのかを分ける。

## TypeScript 型対応

TypeScript の型は実行時に消えるが、コンパイル時には重要である。型情報を利用することで、WASM 生成の品質を上げられる。たとえば、`number[]` と分かる配列は generic object array より効率よく扱える可能性がある。`string` と分かる値への `+` は文字列結合に落とせる。`boolean` と分かる値の branch は truthiness 判定を単純化できる。

ただし、TypeScript の型は sound ではない。`any`、型アサーション、構造的部分型、union、intersection、generic、conditional type などがあるため、型だけを信じて runtime check を完全に消すと壊れる。したがって、最適化には conservative mode と aggressive mode を用意する。

| モード         | 方針                                  | 用途       |
| ----------- | ----------------------------------- | -------- |
| safe        | JS semantics を優先し、runtime check を残す | デフォルト    |
| typed       | TypeScript 型を利用して fast path を増やす    | 通常最適化    |
| strict-wasm | 型が十分明確な箇所を WASM primitive に落とす      | ベンチ・数値処理 |
| unsafe-fast | 意味論差分を許容する実験モード                     | 非デフォルト   |

デフォルトは safe または typed とする。unsafe-fast を標準動作にしてはいけない。これは性能を捨てないためではなく、性能改善と互換性維持を分離するためである。

## JavaScript 意味論

このプロジェクトの難所は TypeScript 構文ではなく JavaScript 意味論である。特に、`this`、prototype、property lookup、dynamic object shape、truthiness、`==`、`===`、`NaN`、`-0`、exception、closure、`eval`、`with`、getter / setter、Proxy などは WASM への直接変換を難しくする。

対応方針として、まず通常の TypeScript コードでよく使われる範囲を正確に実装する。`eval` や `with` のような最適化を破壊する機能は、最初から最重要扱いにはしない。ただし、仕様から削除はしない。明示的に `unsupported-dynamic-code` として扱う。

| 機能              | 方針                                        |
| --------------- | ----------------------------------------- |
| `this`          | call site ごとに receiver を明示して IR に落とす      |
| prototype       | class 対応後に object model に統合               |
| property lookup | string key lookup から開始し、shape cache を後で追加 |
| `==`            | runtime helper で正確性を優先                    |
| `===`           | primitive fast path を用意                   |
| `NaN` / `-0`    | number semantics のテスト対象にする                |
| exception       | runtime stack と wasm exception の両案を検討     |
| `eval`          | 初期非対応、診断必須                                |
| `with`          | 初期非対応、診断必須                                |
| Proxy           | 初期非対応、object model 安定後に検討                 |

## 出力形式

出力形式は少なくとも三種類を用意する。

| 出力                | 内容                               | 用途                        |
| ----------------- | -------------------------------- | ------------------------- |
| `.wasm`           | standalone または WASI 向け core wasm | iwasm / wasmtime / wasmer |
| `.wasm + host.js` | Node.js host shim 付き WASM        | Node API 併用               |
| `.wat`            | デバッグ用テキスト出力                      | compiler 開発・差分確認          |

`.wasm` 単体で動くものは、iwasm 実行を必須ゲートにする。Node.js host が必要なものは、host import の一覧を manifest として出す。これにより、「このプログラムはなぜ iwasm 単体で動かないのか」を説明できる。

出力には `capabilities.json` のようなメタデータを付けるとよい。たとえば、使用している API、必要な WASI capability、Node host import、メモリ初期値、export 関数、entrypoint などを記録する。

## CLI 設計

CLI 名は仮に `ts2wasm` とする。

```bash
ts2wasm build main.ts -o main.wasm
ts2wasm run main.ts
ts2wasm check main.ts
ts2wasm test tests/**/*.ts
ts2wasm emit-wat main.ts -o main.wat
ts2wasm explain main.ts
```

`build` は `.wasm` を生成する。`run` はビルド後に適切な runtime で実行する。Node host が不要なら iwasm または wasmtime で実行し、Node host が必要なら生成された host shim を使う。`check` は parser / semantic / unsupported feature を確認する。`explain` は、そのソースが standalone で動くのか、Node host が必要なのか、どの API が原因なのかを表示する。

たとえば、`process.env` を使っているコードに対して `explain` を実行すると、次のような情報を出す。

```text
target: wasm32-wasi
standalone: no
required host APIs:
  - node.process.env
reason:
  process.env requires host-provided environment object
suggestion:
  use WASI env mapping or compile with --host node
```

## テスト方針

テストは緩くしない。ただし、最初から test262 や TypeScript 全体を全 pass できる前提にも置かない。重要なのは、失敗を曖昧にしないことである。

すべてのテストは、`pass`、`fail`、`unsupported`、`blocked`、`skip-with-reason` に分類する。単なる skip は禁止する。未対応機能による skip には issue ID または tracking label を必ず付ける。これにより、coverage が増えているのか、ただ skip が増えているのかを区別できる。

| 状態               | 意味                             | 許容条件          |
| ---------------- | ------------------------------ | ------------- |
| pass             | 仕様通り成功                         | 常に記録          |
| fail             | 実装バグ                           | 原則として修正対象     |
| unsupported      | 未実装機能                          | 理由と issue が必要 |
| blocked          | runtime / host / toolchain の制約 | 外部条件を明記       |
| skip-with-reason | テスト環境上の除外                      | 無理由 skip 禁止   |

## Coverage State

Coverage は、単なる数字ではなく、どの意味論領域をどれだけ通過しているかを示す。test262、TypeScript compiler tests、tsc parser/checker、TypeScript-Go は、それぞれ役割が違う。

| テスト資産              | 目的                         | このプロジェクトでの使い方                      |
| ------------------ | -------------------------- | ---------------------------------- |
| test262            | JavaScript 仕様互換            | JS runtime semantics の検証           |
| TypeScript tests   | TS 構文・型・emit の互換           | parser / checker / transform の差分検出 |
| tsc parser/checker | TypeScript 公式挙動の oracle    | 自前 frontend の比較対象                  |
| TypeScript-Go      | 実装構造・高速化方針の参考              | parser/checker 設計の比較対象             |
| 独自 fixtures        | WASM / WASI / host API の検証 | このプロジェクト固有の回帰テスト                   |

test262 は巨大なので、最初から全件 pass を要求するのではなく、対象領域ごとに shard を切る。たとえば、`language/expressions`、`built-ins/Array`、`built-ins/String`、`built-ins/Number`、`built-ins/Promise` のように分ける。各 shard には現在の pass / fail / unsupported / blocked を記録する。

TypeScript tests は、構文と型の互換性を見るために使う。`tsc` と同じ診断を最初から完全再現する必要はないが、parser の AST 差分、checker の symbol 差分、emit 前 IR の差分は追跡する。

## Performance Goal

本プロジェクトは、TypeScript を WebAssembly へ変換するだけでなく、生成コードの実行性能を主要な価値として扱う。目標は、TypeScript / JavaScript の既存コードを可能な限り保ちながら、WASM backend によって Node.js / Bun と同等以上の性能を達成することである。

特に、型情報が静的に利用できるコード、数値演算が多いコード、配列アクセスが多いコード、標準入出力中心の短命 CLI、host API 呼び出しが少ないコードでは、Node.js / Bun より高速な実行を目指す。これは専用 subset を導入するためではなく、通常の TypeScript コードに対して最適化レベルを上げることで達成する。

本プロジェクトでは、用途別 profile ではなく、一般的な compiler optimization level を採用する。

```bash
ts2wasm build main.ts -O0 -o main.wasm
ts2wasm build main.ts -O1 -o main.wasm
ts2wasm build main.ts -O2 -o main.wasm
ts2wasm build main.ts -O3 -o main.wasm
```

`-O0` はデバッグ性とコンパイル速度を優先する。`-O1` は軽量な最適化を行う。`-O2` は実用的な高速化を標準的に適用する。`-O3` はコンパイル時間やコードサイズの増加を許容し、より積極的な特殊化・インライン化・表現最適化を行う。

| optimization level | 方針                         | 主な用途                |
| ------------------ | -------------------------- | ------------------- |
| `-O0`              | ほぼ素直な lowering。デバッグしやすさを優先 | compiler 開発、診断、差分確認 |
| `-O1`              | 明らかに安全な最適化のみ適用             | 通常の開発実行             |
| `-O2`              | 性能と安定性のバランスを取る             | 標準の release build   |
| `-O3`              | 型情報を最大限利用し、特殊化を強める         | 高性能実行、数値・配列・短命 CLI  |

`-O2` では、TypeScript の型情報、制御フロー、到達可能性、escape analysis、定数畳み込み、不要 runtime call の削除、primitive fast path、packed array の選択などを行う。`-O3` では、さらに関数インライン化、monomorphization、shape specialization、loop optimization、bounds check elimination、runtime helper の特殊化を行う。

性能目標は「全 JavaScript プログラムで常に Node.js / Bun に勝つ」ことではない。動的 property access、`any`、prototype mutation、Proxy、`eval`、高度な reflection を多用するコードでは、互換性を保つために runtime cost が発生する。一方で、TypeScript の型が十分に効くコードでは、JIT に依存せず、事前コンパイルされた WASM と軽量 runtime により、安定して高速な実行を狙う。

| 領域                  | 性能目標                                                       |
| ------------------- | ---------------------------------------------------------- |
| 数値演算                | `number` / `i32` / `f64` fast path により Node.js / Bun と同等以上 |
| 配列処理                | packed representation と bounds check 最適化により高速化             |
| 文字列処理               | runtime 実装を最適化し、不要な host call を避ける                         |
| 短命 CLI              | 起動時間込みで Node.js より軽い実行を目指す                                 |
| 標準入出力               | WASI stdio を直接使い、Node.js 依存を避ける                            |
| object-heavy code   | shape specialization により改善。ただし動的性が高い場合は互換性優先               |
| dynamic JS features | 正確性優先。性能目標は限定的                                             |

前の提案で出した `Competitive Programming Target` は削除して、代わりにこれを入れるのがよいです。

## Optimization Strategy

本プロジェクトでは、用途別 profile ではなく、通常の最適化レベルによって性能を制御する。特定用途向けの subset を導入すると、TypeScript to WASM transpiler という主目的が曖昧になるためである。

最適化は、TypeScript の通常コードに対して適用される。ユーザーは特別な競技用 API や専用 profile を使う必要はない。`-O2` または `-O3` を指定することで、compiler が型情報と実行形状を解析し、WASM に適した表現へ変換する。

主な最適化は次の通り。

| 最適化                      | 内容                                                                 |
| ------------------------ | ------------------------------------------------------------------ |
| primitive fast path      | `number`、`boolean`、`string` などの型が明確な処理を runtime call なしで実行         |
| numeric specialization   | 整数的に扱える演算を `i32` / `i64` / `f64` に落とす                              |
| packed array             | 要素型が安定した配列を linear memory 上の連続領域に配置                                |
| shape specialization     | object の property layout が安定している場合に lookup を高速化                    |
| function inlining        | 小さい関数や hot path の runtime helper を inline 化                        |
| monomorphization         | generic 関数を利用型ごとに特殊化                                               |
| escape analysis          | heap allocation を stack-like allocation または scalar replacement に変換 |
| bounds check elimination | 安全に証明できる配列境界チェックを削除                                                |
| dead code elimination    | 未使用 builtin / runtime helper を link しない                            |
| host call reduction      | Node.js / WASI import 呼び出しをまとめる、または削る                              |

> 本プロジェクトの性能戦略は、用途別 subset ではなく、TypeScript の型情報と最適化レベルに基づく汎用的な高速化である。結果として、数値計算、配列処理、短命 CLI、標準入出力中心のプログラムでは、競技用途にも耐える性能を目指す。

## Performance State

性能は、最初から測る。初期実装が遅くても、測定項目を固定しておけば改善できる。比較対象は Node、Bun、ts2wasm とする。

| 指標                          | 意味                                 |
| --------------------------- | ---------------------------------- |
| compile time                | `.ts` から `.wasm` 生成までの時間           |
| startup time                | 実行開始から main 到達まで                   |
| execution time              | 実処理時間                              |
| memory usage                | heap / linear memory / host memory |
| wasm size                   | 生成 `.wasm` サイズ                     |
| host calls                  | Node.js / WASI import 呼び出し回数       |
| iwasm compatibility         | iwasm で実行できるか                      |
| correctness under benchmark | 高速化で意味論が壊れていないか                    |

ベンチマークは、数値計算、文字列処理、配列処理、オブジェクト操作、JSON、ファイル I/O、CLI 入出力に分ける。Node や Bun より遅いこと自体は問題ではない。問題なのは、どの層で遅いか分からない状態である。

性能比較では、少なくとも次のような表を継続的に出す。

| benchmark     | Node | Bun | ts2wasm/wasi | ts2wasm/node-host | 備考              |
| ------------- | ---: | --: | -----------: | ----------------: | --------------- |
| fib           |   計測 |  計測 |           計測 |                計測 | 関数呼び出し          |
| array-sum     |   計測 |  計測 |           計測 |                計測 | typed fast path |
| string-concat |   計測 |  計測 |           計測 |                計測 | runtime string  |
| json-parse    |   計測 |  計測 |           計測 |                計測 | builtin         |
| fs-read       |   計測 |  計測 |           計測 |                計測 | WASI / Node 差分  |
| cli-stdio     |   計測 |  計測 |           計測 |                計測 | iwasm 重視        |

## Relative Projects

関連プロジェクトは、競合というより比較対象として扱う。特に QuickJS と AssemblyScript は重要だが、どちらもこのプロジェクトと完全には一致しない。

| Project        | 概要                        | 強み                           | このプロジェクトとの差分                                  |
| -------------- | ------------------------- | ---------------------------- | --------------------------------------------- |
| QuickJS        | 小型 JavaScript engine      | JS 互換性が高い                    | JS を WASM にトランスパイルするのではなく、JS engine を動かす方向    |
| AssemblyScript | TypeScript 風構文から WASM を生成 | WASM 向けに設計されている              | TypeScript / JavaScript 完全互換ではなく、サポート範囲が別言語寄り |
| Emscripten     | C/C++ から WASM             | runtime / libc / JS glue が成熟 | TypeScript 入力ではない                             |
| wasm-bindgen   | Rust と JS の橋渡し            | JS interop が強い               | TypeScript を WASM にする compiler ではない           |
| Javy           | JS を WASM で実行             | WASI 上で JS を動かせる             | transpiler というより JS runtime 同梱に近い             |
| tsc            | TypeScript 公式 compiler    | 仕様挙動の基準                      | 出力は JS であり WASM ではない                          |
| TypeScript-Go  | TypeScript 実装の再構成         | parser/checker 実装の参考         | WASM backend を目的としているわけではない                   |

このプロジェクトの立ち位置は、AssemblyScript より TypeScript 互換に寄せ、QuickJS より compiler / transpiler に寄せる位置にある。つまり、「TypeScript 風の WASM 言語」でも「WASM 上の JS interpreter」でもなく、「TypeScript / JavaScript の実行意味論を保ったまま WASM に落とす compiler」を目指す。

## 禁止事項

すべてを Node.js で処理することは禁止する。Node.js は host API provider としてのみ使う。生成された `.wasm` が実質的に何もせず、Node.js 側に JavaScript ソースを渡して実行する構造は認めない。

JavaScript を文字列として保持し、実行時に `eval`、`Function`、Node.js VM、外部 JS engine に渡すだけの実装も認めない。それは TypeScript to WASM transpiler ではなく、WASM 起動 wrapper である。

テストを通すために仕様を緩めることも禁止する。失敗するテストは失敗として記録し、未対応なら未対応として記録する。`skip` を増やして coverage が良くなったように見せることは禁止する。

性能を諦める提案も採用しない。互換性のために初期実装が遅くなることは許容するが、性能測定をしない、改善余地を潰す、すべて runtime call に逃がす、型情報を使わない、という設計は避ける。

機能を削る提案も採用しない。初期対応の順序を決めることと、機能を捨てることは別である。`async/await`、class、module、exception、builtin、Node API 互換などは段階的に扱うが、プロジェクトの目標から外さない。

## 実装ロードマップ

最初の段階では、TypeScript の小さなプログラムを `.wasm` に変換し、stdio で結果を出せる状態を作る。ここでは `let`、`const`、数値、文字列、boolean、if、while、function、array、object literal の基本を扱う。出力は WASI `.wasm` とし、iwasm で動作することを重視する。

次の段階では、JS 値 runtime を整える。`undefined`、`null`、truthiness、`===`、`+`、property access、array access、function call、closure を安定させる。ここで test262 の小さな shard を導入し、仕様差分を見える化する。

その次に、TypeScript の型情報を pipeline に取り込む。型による fast path、診断、unsupported feature の分類を行う。`tsc` parser/checker との差分を取り、公式挙動とのズレを管理する。

中期では、module、class、exception、JSON、fs、process、Buffer、path を広げる。Node host 併用ターゲットを整え、WASI だけで動くものと Node host が必要なものを明確に分ける。

後期では、test262 / TypeScript tests の coverage dashboard を整備し、performance dashboard を継続的に更新する。Wasm GC backend、Component Model / WIT backend、より強い最適化、shape cache、inline cache、typed array 最適化などを追加する。

## 成功条件

このプロジェクトの成功条件は、単に `.wasm` を出せることではない。TypeScript / JavaScript として意味のあるコードが、WASM 側で実行され、Node.js への依存が明示的に分離され、テストと性能の状態が継続的に測定されることが成功条件である。

最初の明確な成功ラインは、以下のように置く。

| 段階  | 成功条件                                                  |
| --- | ----------------------------------------------------- |
| M1  | `main.ts` から `.wasm` を生成し、iwasm で `console.log` 相当が動く |
| M2  | 数値・文字列・配列・関数・if/while の基本 fixtures が通る                |
| M3  | object / property access / closure が動く                |
| M4  | test262 の小 shard を分類付きで実行できる                          |
| M5  | TypeScript parser/checker oracle との差分を記録できる           |
| M6  | standalone WASI と Node host 併用を自動判定できる                |
| M7  | fs / process / path の基本 API が動く                       |
| M8  | Node / Bun / ts2wasm の performance dashboard が出る      |
| M9  | module / class / exception の主要ケースが通る                  |
| M10 | 大きめの既存 TypeScript package の一部を変換・実行できる                |

## まとめ

このプロジェクトは、TypeScript の資産を WebAssembly 実行環境へ持ち込むための transpiler である。AssemblyScript のように TypeScript 風の別言語へ寄せるのではなく、QuickJS のように JS engine を丸ごと動かす方向にも寄せすぎない。TypeScript / JavaScript の意味論を保ちながら、生成された WASM ができる限り自律的に動くことを目指す。

Node.js は使うが、逃げ道にはしない。WASI で動くものは WASI で動かし、Node.js が必要なものは host API として明示する。テストは緩めず、失敗理由を分類する。性能は捨てず、最初から測る。機能は削らず、段階的に到達する。

この方針なら、「TypeScript の巨大な資産を活用したい」「しかし Node.js に閉じ込められたくない」「iwasm で動く WASM を生成したい」という原案の価値を保ったまま、研究プロジェクトではなく実装可能な処理系プロジェクトとして進められる。
