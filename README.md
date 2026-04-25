# TS transpiler to WASM

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
| `docs/13-coding-standard.md` | Rust コード規約。panic 禁止、Diagnostic、Span、IR variant 追加、backend WAT 直書き禁止、RuntimeFn catalog |
| `docs/14-ir-contracts.md` | AST / HIR / MIR / Wasm IR の責務と不変条件。validate_* の仕様 |
| `docs/15-runtime-abi.md` | RawValue tagged encoding、heap layout、RuntimeFn catalog、host import ABI |
| `docs/16-coverage-matrix.md` | coverage 運用ポリシーと gate 判定基準 |

## Development Init

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
scripts/install_git_hooks.sh

# run test

```

`pre-commit` では `cargo fmt --all --check` を強制する。hook を有効にするには init 時に `scripts/install_git_hooks.sh` を実行する。
