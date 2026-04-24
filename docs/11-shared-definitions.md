# Shared definitions

この文書は、複数の設計文書から参照する横断的な定義を集約する。個別文書はここにある表や schema を再定義せず、必要な文脈だけを説明する。

## Canonical milestones

`M0` から `M10` までの milestone はこの表を正とする。元 README 由来の旧 milestone は `docs/99-original-plan.md` に保持し、運用上の進捗・成功判定・テスト計画ではこの表を参照する。

M0 の機械可読な実装定義は `crates/shared/` に置く。文書上の schema を変更した場合は、同じ変更で Rust の型・validation・単体テストを更新する。

| 段階 | 成功条件 |
|---|---|
| M0 | runtime ABI、capability manifest、test status schema を決める |
| M1 | single-file TS/JS から WASI `.wasm` を生成し、`console.log("hi")` が iwasm で動く |
| M2 | number/string/boolean/if/while/function の fixtures が Node と一致する |
| M3 | `undefined` / `null` / truthiness / `===` / `+` の semantic tests が通る |
| M4 | compile-time evaluator をやめ、`undefined` / `null` / truthiness / `===` / `+` の M3 fixtures が WASM runtime 上の JS value execution で通る |
| M5 | array、string、object literal、data property access の基本操作が WASM runtime 上で通る。prototype はまだ対象外でよい |
| M6 | `require("fs").readFileSync(0, "utf8")` と `console.log` が Node host なしで動く。stdin は WASI `fd_read` で読み、入力処理は WASM/runtime 側で実行する |
| M7 | differential test runner で Node との差分を分類できる |
| M8 | test262 の小 shard を canonical test status schema で管理できる |
| M9 | TypeScript 型情報を使った primitive fast path が入る |
| M10 | Node host が必要な API を manifest 付きで実行できる |

### M6 entry gate (required)

M6 への着手条件として、M5 で先送りした P0 技術負債を返済する。順序は次を推奨する。

1. RuntimeLinkPlan を `WatEmitter` から分離する
2. BuiltinResolver pass を分離する（`console.log` / `.length` / property read）
3. capability manifest 出力を導入する（`fd_write` / `fd_read` を catalog + plan + manifest で一貫管理）
4. AST node span を導入し、source 起因 diagnostic に span を付与する

この gate を満たすまでは M6 の成功条件を評価しない。

## Test status schema

すべてのテスト結果は、次の状態のいずれかに分類する。単なる skip は禁止する。

| 状態 | 意味 | 必須情報 |
|---|---|---|
| `pass` | 仕様通り成功 | suite、case、target |
| `fail` | 実装バグまたは仕様差分 | expected、actual、再現 target |
| `unsupported` | 未実装機能 | reason、feature label、issue ID または tracking label |
| `blocked` | runtime / host / toolchain の外部制約 | blocking condition、owner または upstream |
| `skip-with-reason` | テスト環境上の除外 | reason、除外条件、再確認条件 |

機械可読な test record は、少なくとも `suite`、`case`、`target`、`status`、`reason`、`tracking` を持つ。`reason` と `tracking` は `pass` では省略できるが、`unsupported`、`blocked`、`skip-with-reason` では必須とする。

## Capability manifest schema

生成物は、使用する外部能力を manifest として監査できる必要がある。filesystem は read/write/preopen を分離し、Node host import は関数単位で列挙する。

```json
{
  "schema_version": 1,
  "target": "wasm32-wasi",
  "standalone": true,
  "wasi": {
    "stdin": true,
    "stdout": true,
    "stderr": false,
    "args": false,
    "env": false,
    "filesystem": {
      "read": [],
      "write": [],
      "preopens": []
    },
    "random": false
  },
  "node_host": {
    "required": false,
    "imports": []
  },
  "capability_reasons": {
    "wasi.stdin": [
      "fs.readFileSync(0, \"utf8\")"
    ],
    "wasi.stdout": [
      "console.log"
    ]
  }
}
```

Node host が必要な場合は、`standalone` を `false` にし、`node_host.imports` に `host.<domain>.<function>` 形式で必要な関数だけを列挙する。

```json
{
  "schema_version": 1,
  "target": "wasm32-wasi+node-host",
  "standalone": false,
  "wasi": {
    "stdin": false,
    "stdout": true,
    "stderr": false,
    "args": false,
    "env": false,
    "filesystem": {
      "read": [],
      "write": [],
      "preopens": []
    },
    "random": false
  },
  "node_host": {
    "required": true,
    "imports": [
      "host.timer.setTimeout"
    ]
  },
  "capability_reasons": {
    "wasi.stdout": [
      "console.log"
    ],
    "host.timer.setTimeout": [
      "setTimeout"
    ]
  }
}
```

## Optimization and safety modes

CLI の optimization level と semantic safety mode は別概念として扱う。`-O3` でも observable JavaScript semantics は壊さない。意味論差分を許す実験は `unsafe-fast` として明示的に分離する。

| CLI level | default safety mode | 方針 |
|---|---|---|
| `-O0` | `safe` | デバッグ性と差分確認を優先し、ほぼ素直に lowering する |
| `-O1` | `safe` | 明らかに安全な局所最適化だけを適用する |
| `-O2` | `typed` | 型情報と制御フローを使い、runtime check を保ちながら fast path を増やす |
| `-O3` | `typed` / proven `strict-wasm` | 証明できる範囲で特殊化、インライン化、表現最適化を強める |
| explicit `unsafe-fast` | `unsafe-fast` | 意味論差分を許容する実験モード。デフォルトにしない |

`strict-wasm` は、型と runtime guard によって観測可能な意味論差分がないと示せる範囲だけで使う。property store、function call、object escape、host boundary では canonical `JsValue` へ戻す。

## Benchmark policy

性能比較は、測定条件を固定して継続的に記録する。少なくとも benchmark 名、input size、target、runner version、cold/warm 区分、iteration count、median、p95、peak memory、wasm size、host call count を記録する。

M8 までは performance regression を report-only にしてよい。M8 以降は、所有 benchmark の median が基準値から 10% を超えて悪化した場合、明示的な tracking label がない限り CI failure として扱う。
