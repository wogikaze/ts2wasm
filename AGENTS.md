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
python scripts/manager.py nextest
python scripts/manager.py coverage-report --format markdown
python scripts/manager.py repo-metrics
```

`mise` が使える環境では同名の `mise run <task>` を使ってよい。Windows では `python scripts/manager.py ...` を優先する。

## プロジェクト構造

| Path | 役割 |
|---|---|
| `crates/frontend/` | lexer/parser、TypeScript erasure、tsc/tsgo 連携 |
| `crates/resolve/` | binding/name resolution、direct eval source handling |
| `crates/semantics/` | builtin identity、host API 分類、意味論補助 |
| `crates/ir/` | builtin-resolved AST、HIR、MIR、legacy LoweredProgram、optimizer |
| `crates/runtime-abi/` | tagged value と linear-memory ABI 定数 |
| `crates/runtime-catalog/` | RuntimeFn、依存関係、host import、capability、link plan |
| `crates/backend-wasm/` | Lowered/MIR から WAT/WASM への emission と runtime templates |
| `crates/compiler/` | end-to-end pipeline、module graph、server、dump |
| `crates/cli/` | `ts2wasm` CLI |
| `fixtures/` | semantic/differential/parser fixtures と catalog |
| `tests/` | reference runner 用 manifest と test262 smoke |
| `docs/` | 現在の設計・運用・仕様 docs |

## タスク別ルーティング

| タスク | 最初に読む |
|---|---|
| 全体理解 | `README.md`, `docs/INDEX.md`, `docs/01-project-definition.md` |
| compiler pipeline | `docs/04-compiler-architecture-and-runtime.md`, `crates/compiler/src/pipeline.rs` |
| frontend/parser | `docs/28-frontend-syntax-ownership.md`, `crates/frontend/src/parser.rs` |
| name/builtin resolution | `docs/13-ir-contracts.md`, `crates/resolve/src/name_resolver.rs`, `crates/ir/src/builtin_resolver.rs` |
| runtime/ABI | `docs/14-runtime-abi.md`, `crates/runtime-abi/src/*`, `crates/runtime-catalog/src/*` |
| host capability | `docs/03-api-and-host-capability.md`, `crates/shared/src/capability.rs`, `crates/runtime-catalog/src/link_plan.rs` |
| tests/coverage | `docs/06-testing-and-coverage.md`, `docs/15-coverage-matrix.md`, `fixtures/catalog.yaml` |
| docs 更新 | `docs/00-docs-list.md`, `docs/25-robust-test-design.md`, `docs/templates/feature-slice.md` |
| web dashboard | `docs/18-web-ui-reporting.md`, `web-ui/README.md`, `site/README.md` |

## Do

- 仕様判断はコードコメントだけでなく docs に残す。
- AST/HIR/MIR/runtime ABI の境界をまたぐ変更では、対応する docs を同時に更新する。
- `Diagnostic` は source/span/phase を保ち、unsupported を曖昧な文字列だけにしない。
- RuntimeFn 追加時は catalog spec、deps/imports/capabilities/signature/link-plan tests を一緒に見る。
- Capability manifest は auditable reason を必ず持たせる。

## Never

- backend から frontend AST (`Stmt`/`Expr`) を新たに直接読む path を増やさない。IR 経由にする。
- runtime ABI のタグ、layout offset、memory address を magic number として散らさない。`runtime-abi` の型/定数を使う。
- host import を capability manifest なしに追加しない。
- generated coverage/artifact を根拠なく手編集しない。
- `issues/` を bulk docs rewrite の巻き添えにしない。

## Ask First

- 新しい外部 crate/npm dependency を追加する。
- target ABI、manifest schema、runtime ABI version を変更する。
- `issues/` の状態や priority を一括変更する。
- reference corpus の lock や大量 fixture status を変更する。

## コンテキスト圧縮時に残す情報

- 変更したファイル一覧。
- 実行した検証コマンドと exit status。
- 未解決の blocker と次に読むべき 1 ファイル。
