# ts2wasm Agent Router

このファイルは agent 向けの短い作業契約である。詳しい説明をここに詰め込まない。まず `README.md` と `docs/INDEX.md` を読み、タスクに必要なファイルだけを追加で参照する。

## 最重要ルール

- 調査だけで終わらせない。ユーザーが明示的に調査のみを求めない限り、変更・検証・成果物作成まで進める。
- `issues/` は作業台帳であり、ユーザーが指定しない限り broad docs rewrite の巻き添えにしない。
- 既存のユーザー変更を上書きしない。作業前に `git status --short` を確認する。
- 生成物と手書き docs を混ぜない。生成物は生成コマンドを明記し、手で直す場合は理由を書く。
- gate/hook を `--no-verify` で迂回しない。

## コアコマンド

```bash
python scripts/manager.py check
python scripts/manager.py gate-fast
python scripts/manager.py gate
python scripts/manager.py verify          # arch DAG + complexity + docs + architecture
python scripts/manager.py verify --quick  # 軽量版 (pre-commit相当)
python scripts/manager.py nextest
python scripts/manager.py coverage-report --format markdown
python scripts/manager.py repo-metrics
```

### アーキテクチャチェック

```bash
python scripts/check/check-arch-dag.py            # 依存DAG + 禁止import + ファイルサイズ
python scripts/check/architecture-rules.py        # crate境界 + freeze監視
python scripts/check/check-runtimefn-deprecation.py # RuntimeFn廃止追跡
```

`mise` が使える環境では同名の `mise run <task>` を使ってよい。Windows では `python scripts/manager.py ...` を優先する。

## プロジェクト構造 (新設計)

| Path | 役割 | 制約 |
|---|---|---|
| `crates/frontend/` | lexer/parser、TypeScript erasure | JS semantics を入れない |
| `crates/resolve/` | binding/name resolution | 実行意味を決めない |
| `crates/semantics/` | builtin identity、host API 分類 | 意味論補助 |
| `crates/ir/` | **LEGACY**: LoweredProgram、旧MIR | frozen、bugfix のみ |
| `crates/runtime-abi/` | tagged value / linear-memory ABI 定数 | 安定、変更不可 |
| `crates/runtime-catalog/` | **LEGACY**: RuntimeFn registry | frozen、RuntimeFn 追加禁止 |
| `crates/backend-wasm/` | **LEGACY**: native_lowered + typed.rs (+ new spec_emit) | JS semantics 禁止。薄い emitter へ移行中 |
| `crates/compiler/` | pipeline orchestration | builtin 実装を書かない |
| `crates/cli/` | `ts2wasm` CLI | 入出力だけ |
| **新設計 core:** | | |
| `crates/runtime-core/` | JS engine substrate (Value, heap, shape, env, realm, GC) | frontend/backend に非依存 |
| `crates/semantic-ir/` | ECMAScript 意味論 IR (CFG-based) | wasm instruction 禁止 |
| `crates/spec-kernel/` | SpecOp + internal method dispatch | backend に非依存 |
| `crates/backend-correctness/` | SemIR → SpecOp / runtime call | JS semantics は書かない |
| `crates/opt-mir/` | Guard/deopt 最適化 IR | Proxy 意味論禁止 |
| **テスト/設定:** | | |
| `fixtures/` | semantic/differential/parser fixtures | |
| `tests/` | reference runner / test262 smoke | |
| `docs/` | 設計・運用・仕様 docs | |
| `COVERAGE.md` | Coverage Driver プロトコル | |

## タスク別ルーティング

| タスク | 最初に読む |
|---|---|
| 全体理解 | `README.md`, `docs/INDEX.md`, `docs/01-project-definition.md` |
| 新設計アーキテクチャ | `COVERAGE.md`, `docs/superpowers/plans/2026-06-18-architecture-rewrite.md`, `arch-rules.toml` |
| compiler pipeline | `docs/04-compiler-architecture-and-runtime.md`, `crates/compiler/src/pipeline.rs` |
| frontend/parser | `docs/28-frontend-syntax-ownership.md`, `crates/frontend/src/parser.rs` |
| name/builtin resolution | `docs/13-ir-contracts.md`, `crates/resolve/src/name_resolver.rs`, `crates/ir/src/builtin_resolver.rs` |
| **新設計 runtime-core** | `crates/runtime-core/src/*`, `docs/14-runtime-abi.md` |
| **新設計 semantic-ir** | `crates/semantic-ir/src/*`, `COVERAGE.md` |
| **新設計 spec-kernel** | `crates/spec-kernel/src/*`, `arch-rules.toml` |
| **新設計 backend-correctness** | `crates/backend-correctness/src/*` |
| **新設計 opt-mir** | `crates/opt-mir/src/*` |
| runtime/ABI (legacy) | `docs/14-runtime-abi.md`, `crates/runtime-abi/src/*`, `crates/runtime-catalog/src/*` |
| host capability | `docs/03-api-and-host-capability.md`, `crates/shared/src/capability.rs`, `crates/runtime-catalog/src/link_plan.rs` |
| tests/coverage | `docs/06-testing-and-coverage.md`, `COVERAGE.md`, `fixtures/catalog.yaml` |
| docs 更新 | `docs/00-docs-list.md`, `docs/25-robust-test-design.md`, `docs/templates/feature-slice.md` |
| web dashboard | `docs/18-web-ui-reporting.md`, `web-ui/README.md`, `site/README.md` |

## Do

- 仕様判断はコードコメントだけでなく docs に残す。
- AST/HIR/MIR/runtime ABI の境界をまたぐ変更では、対応する docs を同時に更新する。
- `Diagnostic` は source/span/phase を保ち、unsupported を曖昧な文字列だけにしない。
- (LEGACY only) RuntimeFn 保守時は catalog spec、deps/imports/capabilities/signature/link-plan tests を一緒に見る。新規追加は禁止 (SpecOp を使う)。
- Capability manifest は auditable reason を必ず持たせる。

## Never

- backend から frontend AST (`Stmt`/`Expr`) を新たに直接読む path を増やさない。IR 経由にする。
- runtime ABI のタグ、layout offset、memory address を magic number として散らさない。`runtime-abi` の型/定数を使う。
- host import を capability manifest なしに追加しない。
- generated coverage/artifact を根拠なく手編集しない。
- `issues/` を bulk docs rewrite の巻き添えにしない。
- **`RuntimeFn` variant を追加しない。** 新しい仕様操作は `SpecOp` に追加する。既存の `RuntimeFn` は frozen。
- **`native_lowered.rs`、`typed.rs`、`native_runtime_embed.rs` を変更しない。** 新機能の追加は禁止。bugfix と削除のみ許可。
- **backend-wasm で JS 仕様判断を書かない。** backend は `SpecOp`/`OptOp` を wasm に落とすだけ。
- **semantic-ir に wasm instruction を置かない。** ここは ECMAScript semantics を表す層。
- **opt-mir に Proxy 意味論を書かない。** Guard/deopt だけを持つ。
- **spec-kernel を backend-wasm に依存させない。** 仕様は backend に依存しない。
- **compiler に個別 builtin 実装を書かない。** orchestration だけ。
- **coverage だけのために旧経路を変更しない。** `COVERAGE.md` のプロトコルに従う。

## Ask First

- 新しい外部 crate/npm dependency を追加する。
- target ABI、manifest schema、runtime ABI version を変更する。
- `issues/` の状態や priority を一括変更する。
- reference corpus の lock や大量 fixture status を変更する。

## コンテキスト圧縮時に残す情報

- 変更したファイル一覧。
- 実行した検証コマンドと exit status。
- 未解決の blocker と次に読むべき 1 ファイル。
