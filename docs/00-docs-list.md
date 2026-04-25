# Docs List

## Docs

| File | Responsibility | Source sections | Added review material |
|---|---|---|---|
| `README.md` | 入口、docs map | 概要・まとめの圧縮配置 | docs map |
| `docs/01-project-definition.md` | project identity、目標、非目標、禁止事項 | 概要、目標、非目標、禁止事項、まとめ | 互換性レベル、transpiler/compiler 用語整理 |
| `docs/02-execution-model-and-targets.md` | generated wasm/runtime/host shim、target、output、CLI | 基本方針、実行ターゲット、出力形式、CLI 設計 | target matrix |
| `docs/03-api-and-host-capability.md` | API lowering、WASI-compatible idiom、host trimming | API 対応方針、WASI-compatible Node Idioms、Host Shim Trimming | `process.env` の矛盾修正、capability audit |
| `docs/04-compiler-architecture-and-runtime.md` | compiler pipeline、IR、runtime ABI、value/memory | コンパイラ構成、値表現、メモリ管理 | runtime ABI 章 |
| `docs/05-compatibility-and-semantics.md` | TS/JS 対応範囲、未対応管理、意味論 | TypeScript 構文対応、TypeScript 型対応、JavaScript 意味論 | module/npm ecosystem 章 |
| `docs/06-testing-and-coverage.md` | test taxonomy、coverage dashboard、oracle | テスト方針、Coverage State | differential execution、host-deny、ABI tests |
| `docs/07-performance-and-optimization.md` | optimization levels、benchmark、regression | Performance Goal、Optimization Strategy、Performance State | cold/warm 分離 |
| `docs/08-roadmap-and-success.md` | roadmap narrative と成功条件 | 実装ロードマップ、成功条件 | canonical workstream/gate への参照 |
| `docs/09-security-and-capability-model.md` | sandbox/capability/security policy | Host Shim Trimming、API 対応方針から抽出 | preopen/env/fs/network threat model |
| `docs/10-related-projects.md` | related projects と差分 | Relative Projects | comparison criteria |
| `docs/11-shared-definitions.md` | project goal、workstreams、gates、test status schema、capability manifest、optimization mode、benchmark policy | 複数 doc から参照される横断定義 | 重複定義の集約 |
| `docs/12-coding-standard.md` | Rust コード規約。panic 禁止、Diagnostic、Span、IR variant 追加、backend WAT 直書き禁止、RuntimeFn catalog | なし | 新規追加 |
| `docs/13-ir-contracts.md` | AST / HIR / MIR / Wasm IR の責務と不変条件。validate_* の仕様 | なし | 新規追加 |
| `docs/14-runtime-abi.md` | RawValue tagged encoding、heap layout、RuntimeFn catalog、host import ABI | なし | 新規追加 |
| `docs/15-coverage-matrix.md` | reference coverage の運用ポリシーと gate 基準 | `docs/06` の coverage 方針を運用化 | generated artifact 参照 |
| `current-state.md` | 現在の実装事実、未実装範囲、検証状況 | なし | status tracking |

## Source-of-truth boundaries（責務の切り分け）

| 種別 | 正本 | 個別 doc / artifact の役割 |
|---|---|---|
| Policy（何を pass と呼ぶか、gate の意味） | `docs/11-shared-definitions.md` | `docs/06`, `docs/15` は運用・taxonomy を補足のみ |
| Coverage schema と列定義 | `docs/15-coverage-matrix.md` + `scripts/run/reference-coverage.sh` の stdout キー | `docs/06` はテスト分類。実測行は artifact のみ |
| Coverage 実測値 | `artifacts/coverage/reference-coverage-matrix.md` | `docs/15` に数値を複製しない |
| Capability / manifest schema | `docs/11`, `docs/09` | `docs/03` は API 方針 |
| 実装の現在地・代表コマンド | `current-state.md` | README は入口に留める |
| Runtime ABI / RawValue | `docs/14`, `docs/04` | `crates/shared` の型とテストで機械的に固定 |

## Recommended maintenance rule

- README は 150〜250 行程度を上限にする。
- 仕様・設計・テスト・性能の詳細は docs に置く。
- 新しい設計判断は ADR または該当 docs に置く。
- 実装状況が変わった場合、`current-state.md` を同じ変更で更新する。
- project goal、workstreams、gates、test status schema、capability manifest、optimization mode、benchmark policy を更新する場合、`docs/11-shared-definitions.md` を更新し、個別 doc では再定義しない。
- `docs/12-coding-standard.md`、`docs/13-ir-contracts.md`、`docs/14-runtime-abi.md`、`docs/15-coverage-matrix.md` のいずれかを変更した場合は、コード規約・IR 不変条件・runtime ABI・coverage ポリシーが実装・artifact と一致しているか確認する。
- coverage 進捗更新は `artifacts/coverage/reference-coverage-matrix.md` を同じ変更で更新する。
- host API を増やした場合、`docs/03-api-and-host-capability.md`、`docs/09-security-and-capability-model.md`、`docs/11-shared-definitions.md` の capability manifest を同時に確認する。
