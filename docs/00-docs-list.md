# Improved docs list

この docs list は、元 README の情報量を維持したまま、保守しやすい粒度へ分割するための一覧である。README 本体は入口に寄せ、設計密度の高い本文は `docs/` 配下へ移す。

## Information density rule

| Rule | 内容 |
|---|---|
| no summary-only move | 元 README の詳細説明・表・コード例は要約だけにしない |
| source section preserved | 原文セクションは対応 doc に本文ごと移動する |
| new material labeled | レビューで追加した観点は `追加設計` として明示する |
| original retained | 原文を `docs/99-original-plan.md` に保持し、差分確認できるようにする |
| repeated claims consolidated | 重複主張は README から減らすが、根拠・表・設計判断は削らない |

## Docs

| File | Responsibility | Source sections | Added review material |
|---|---|---|---|
| `README.md` | 入口、立ち位置、docs map、最初の実装 slice | 概要・まとめの圧縮配置 | docs map |
| `docs/01-project-definition.md` | project identity、目標、非目標、禁止事項 | 概要、目標、非目標、禁止事項、まとめ | 互換性レベル、transpiler/compiler 用語整理 |
| `docs/02-execution-model-and-targets.md` | generated wasm/runtime/host shim、target、output、CLI | 基本方針、実行ターゲット、出力形式、CLI 設計 | target matrix |
| `docs/03-api-and-host-capability.md` | API lowering、WASI-compatible idiom、host trimming | API 対応方針、WASI-compatible Node Idioms、Host Shim Trimming | `process.env` の矛盾修正、capability audit |
| `docs/04-compiler-architecture-and-runtime.md` | compiler pipeline、IR、runtime ABI、value/memory | コンパイラ構成、値表現、メモリ管理 | runtime ABI 章 |
| `docs/05-compatibility-and-semantics.md` | TS/JS 対応範囲、未対応管理、意味論 | TypeScript 構文対応、TypeScript 型対応、JavaScript 意味論 | module/npm ecosystem 章 |
| `docs/06-testing-and-coverage.md` | test taxonomy、coverage dashboard、oracle | テスト方針、Coverage State | differential execution、host-deny、ABI tests |
| `docs/07-performance-and-optimization.md` | optimization levels、benchmark、regression | Performance Goal、Optimization Strategy、Performance State | cold/warm 分離 |
| `docs/08-roadmap-and-success.md` | roadmap narrative と成功条件 | 実装ロードマップ、成功条件 | canonical milestone への参照 |
| `docs/09-security-and-capability-model.md` | sandbox/capability/security policy | Host Shim Trimming、API 対応方針から抽出 | preopen/env/fs/network threat model |
| `docs/10-related-projects.md` | related projects と差分 | Relative Projects | comparison criteria |
| `docs/11-shared-definitions.md` | milestone、test status schema、capability manifest、optimization mode、benchmark policy | 複数 doc から参照される横断定義 | 重複定義の集約 |
| `docs/12-current-implementation-status.md` | 現在の実装事実、未実装範囲、検証状況 | なし | status tracking |
| `docs/13-coding-standard.md` | Rust コード規約。panic 禁止、Diagnostic、Span、IR variant 追加、backend WAT 直書き禁止、RuntimeFn catalog | なし | 新規追加 |
| `docs/14-ir-contracts.md` | AST / HIR / MIR / Wasm IR の責務と不変条件。validate_* の仕様 | なし | 新規追加 |
| `docs/15-runtime-abi.md` | RawValue tagged encoding、heap layout、RuntimeFn catalog、host import ABI | なし | 新規追加 |
| `docs/16-coverage-matrix.md` | reference test coverage dashboard。test262/TypeScript/tests を分母に進捗管理 | `docs/06` の coverage 方針を実測値で運用 | 新規追加 |
| `docs/99-original-plan.md` | lossless source | 全文 | なし |

## Recommended maintenance rule

- README は 150〜250 行程度を上限にする。
- 仕様・設計・テスト・性能の詳細は docs に置く。
- 新しい設計判断は ADR または該当 docs に置く。
- 実装状況が変わった場合、`docs/12-current-implementation-status.md` を同じ変更で更新する。
- milestone、test status schema、capability manifest、optimization mode、benchmark policy を更新する場合、`docs/11-shared-definitions.md` を更新し、個別 doc では再定義しない。
- `docs/13-coding-standard.md`、`docs/14-ir-contracts.md`、`docs/15-runtime-abi.md` を追加した場合は、コード規約・IR 不変条件・runtime ABI が実装に反映されているか確認する。
- coverage 進捗更新は `docs/16-coverage-matrix.md` の reference suite 行を同じ変更で更新する。
- host API を増やした場合、`docs/03-api-and-host-capability.md`、`docs/09-security-and-capability-model.md`、`docs/11-shared-definitions.md` の capability manifest を同時に確認する。
