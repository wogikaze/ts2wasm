# TS transpiler to WASM

## Project Success Criteria

This project is considered successful when:
- TypeScript/JavaScript assets can be compiled to WASM without Node.js dependency
- Generated WASM runs correctly in iwasm (WAMR runtime)
- Differential testing against Node.js shows semantic equivalence for supported features
- Capability manifest provides auditable security model
- Reference coverage meets gate thresholds defined in docs/15-coverage-matrix.md
- All gates (fmt, nextest, check-repo-smoke) pass consistently

See docs/08-roadmap-and-success.md for detailed success criteria.

## Project position

TypeScript / JavaScript の既存資産を、Node.js に処理を丸投げせず、可能な限り WebAssembly 実行環境へ持ち込む compiler/runtime プロジェクトである。AssemblyScript のような TypeScript 風の別言語ではなく、QuickJS/Javy のような JS engine 同梱にも寄せすぎない。生成 WASM、WASM 側 runtime、必要最小限の host shim を分離する。

## Documentation map

| Document | Role |
|---|---|
| `docs/00-docs-list.md` | docs 全体の分割方針、情報保持ルール、各 doc の責務 |
| `docs/01-project-definition.md` | 概要、目標、非目標、禁止事項、まとめ |
| `docs/02-execution-model-and-targets.md` | 三層実行モデル、WASI/iwasm/Node host/将来 backend、出力形式、CLI |
| `docs/03-api-and-host-capability.md` | API 対応、WASI-compatible Node idioms、host shim trimming、capability manifest |
| `docs/04-compiler-architecture-and-runtime.md` | frontend/semantic/lowering/backend、値表現、memory management、runtime ABI |
| `docs/05-compatibility-and-semantics.md` | TypeScript 構文・型、JavaScript 意味論、module/npm 対応境界 |
| `docs/06-testing-and-coverage.md` | テスト分類、coverage state、differential testing、skip policy |
| `docs/07-performance-and-optimization.md` | performance goal、optimization levels、optimization strategy、benchmark state |
| `docs/08-roadmap-and-success.md` | 実装ロードマップ、成功条件、workstream/gate 運用 |
| `docs/09-security-and-capability-model.md` | host capability、WASI preopen、Node host security、manifest 監査 |
| `docs/10-related-projects.md` | QuickJS、AssemblyScript、Emscripten、Javy、tsc などとの比較 |
| `docs/11-shared-definitions.md` | test status schema、capability manifest、optimization mode、benchmark policy |
| `current-state.md` | 現在の実装事実、未実装範囲、検証状況（root 管理） |
| `docs/12-coding-standard.md` | Rust コード規約。panic 禁止、Diagnostic、Span、IR variant 追加、backend WAT 直書き禁止、RuntimeFn catalog |
| `docs/13-ir-contracts.md` | AST / HIR / MIR / Wasm IR の責務と不変条件。validate_* の仕様 |
| `docs/14-runtime-abi.md` | RawValue tagged encoding、heap layout、RuntimeFn catalog、host import ABI |
| `docs/15-coverage-matrix.md` | coverage 運用ポリシーと gate 判定基準 |

## Development Init

### Nix（推奨・ツールの共有）

Nix が入っている場合は、リポジトリの devshell で Rust / Node / `iwasm` / よく使う CLI をまとめて揃えられる。

- **Flakes あり**: `nix develop`
- **Flakes なし / 従来どおり**: `nix-shell`（ルートの `shell.nix`）

devshell に含まれる主なもの: `rustc` / `cargo` / `rustfmt` / `clippy` / `cargo-nextest` / `nodejs`（`npm` 同梱）/ `git` / `wamr`（`iwasm`）/ `ripgrep` / `ast-grep`。定義は `nix/devshell.nix`。

nixpkgs の版は `nix/nixpkgs-tarball.nix` の `builtins.fetchTarball` で固定している。コミットやハッシュを上げる手順は同ファイル先頭のコメントに従う（`sha256` は tarball の生バイトではなく展開後向けなので、`nix-prefetch-url --unpack` や評価時の `got:` を使う）。

Flakes で `nix develop` する場合、評価対象の Nix ファイルは **Git に追跡されている必要がある**（未 `git add` のファイルは見えない）。

**対話シェルについて**: `nix develop` と対話型の `nix-shell` は、nixpkgs の慣習で **中に入るシェルが bash になる**（普段 zsh でも、ここだけ bash）。zsh のまま同じ環境を使うなら `nix develop -c zsh` や `nix-shell --run zsh`。外側の zsh を替えずに PATH だけ足すなら **direnv**（`use flake` / `use nix`）が向いている。

README などの fenced code（言語タグが `bash` と付いているもの）は、コピー用の例示であってログインシェルを bash に切り替えない。`scripts/*.sh` を走らせたときだけ、そのプロセスが shebang どおり bash になる。

### リポジトリ初期化（Nix の有無に共通）

```bash
# git clone reference
cat <<'EOF' | xargs -P 6 -n 2 sh -c 'git clone "$0" "$1" --depth 1'           
https://github.com/bytecodealliance/wasm-micro-runtime.git reference/wasm-micro-runtime
https://github.com/microsoft/typescript-go.git reference/typescript-go
https://github.com/AssemblyScript/assemblyscript.git reference/assemblyscript
https://github.com/bytecodealliance/javy.git reference/javy
https://github.com/bellard/quickjs.git reference/quickjs
https://github.com/bytecodealliance/wasm-tools.git reference/wasm-tools
https://github.com/bytecodealliance/wasmtime.git reference/wasmtime
https://github.com/bytecodealliance/jco.git reference/jco
https://github.com/emscripten-core/emscripten.git reference/emscripten
https://github.com/rustwasm/wasm-bindgen.git reference/wasm-bindgen
https://github.com/quickjs-ng/quickjs.git reference/quickjs-ng
EOF

# install repo-managed git hooks
scripts/dev/install-git-hooks.sh

# optional: one entry for all scripts/ without opening each .sh
# scripts/manager help
# Install mise (https://mise.jdx.dev) and: mise trust; mise tasks; mise run nextest
```

### 手動でツールを入れる場合（Nix を使わないとき）

```bash
npm install --global @ast-grep/cli
curl -LsSf https://get.nexte.st/latest/linux | tar zxf - -C ${CARGO_HOME:-~/.cargo}/bin
# ripgrep は OS のパッケージマネージャで入れる（例: apt install ripgrep, brew install ripgrep）
# mold はビルド高速化のために推奨（例: apt install mold）
```

`iwasm`（WAMR）は OS のパッケージマネージャやソースビルドで用意する。`cargo` / `rustc` は [rustup](https://rustup.rs/) 等で揃える。

**ビルド高速化**: プロジェクトは `mold` linker を使用してビルド時間を短縮しています。Nix devshell には含まれていますが、手動環境では別途インストールが必要です。

`pre-commit` では `cargo fmt --all --check`、ステージした Markdown 向け `markdownlint`、必要に応じた `issues/index.md` の再生成、および `mise run check-issue-health`（`issues/` の番号・パス等の不変条件）を実行する。hook を有効にするには init 時に `scripts/dev/install-git-hooks.sh` を実行する。

**Note**: `scripts/manager check-agent-state` requires `jsonschema` for validating `.agents/state/` JSON files. This is included in the Nix devshell (`python3Packages.jsonschema`). Without Nix, install with: `python -m pip install jsonschema`.

## FAQ

### Q: なぜ AssemblyScript や QuickJS ではなく独自の compiler/runtime を作るのか？

A: AssemblyScript は TypeScript 風の別言語であり、既存の JavaScript 資産をそのまま変換できない。QuickJS/Javy は JS engine 同梱であり、WASM サイズと capability boundary の観点で最適ではない。このプロジェクトは、既存の TS/JS 資産を可能な限りそのまま WASM に持ち込み、必要最小限の host shim で実行することを目指す。

### Q: TypeScript の型情報はどう扱うのか？

A: 初期段階では型を「実行に必要な情報」と「診断に必要な情報」に分ける。実行に必要な情報は優先して compiler pipeline に取り込み、診断互換は段階的に強化する。詳細は `docs/04-compiler-architecture-and-runtime.md` を参照。

### Q: Node.js との semantic equivalence はどう確認するのか？

A: differential testing を使用する。同じ TS/JS コードを Node.js と生成された WASM で実行し、stdout を比較する。詳細は `docs/06-testing-and-coverage.md` を参照。  
  Reference coverage の運用は `python scripts/manager.py reference-coverage <suite> [--limit N]`（ramp）を再実行し、  
  `python scripts/manager.py update-coverage-matrix --check`（check）で matrix の整合を確認する。

### Q: standalone 対象プログラムとは何か？

A: Node host import を必要としないプログラム。WASI のみで実行可能。capability manifest により監査可能。詳細は `docs/03-api-and-host-capability.md` を参照。

### Q: どの issue から着手すべきか？

A: `current-state.md` の「Next Priority Slice」セクションに優先度順のリストがある。AI エージェントや自律開発ループではこのリストを参照。

### Q: テストはどう実行するのか？

A: `cargo nextest run` で実行。differential testing が必要な場合は `python scripts/manager.py reference-coverage` を使用。詳細は `docs/06-testing-and-coverage.md` を参照。

### Q: Nix がなくても開発できるか？

A: 可能。手動でツールを入れる場合の例が README にある。ただし Nix を使うとツールの共有が容易になる。

### Q: Windows で開発できるか？

A: 可能。`scripts/manager.py` と `docs/17-windows-development.md` を参照。主要なコマンド（fmt, clippy, nextest, check-fast-gate）は Python で動作する。ただし一部のスクリプトはまだ bash 依存で、Windows では動作しない。完全な機能には WSL2 の使用を推奨。
