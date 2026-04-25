# Reference sources

各プロジェクトのGitHubリポジトリで見るべきファイルを構造化。

## WebAssembly

### wasm-micro-runtime (WAMR)

- Repo: <https://github.com/bytecodealliance/wasm-micro-runtime>
- Key files:
  - `README.md` - 概要、ビルド手順
  - `doc/` - ドキュメント
  - `core/iwasm/` - VM core 実装
  - `samples/` - サンプル

### wasm-tools

- Repo: <https://github.com/bytecodealliance/wasm-tools>
- Key files:
  - `README.md` - 概要、CLIツール一覧
  - `crates/` - Rustライブラリ実装
  - `tests/` - テスト

### wasmtime

- Repo: <https://github.com/bytecodealliance/wasmtime>
- Key files:
  - `README.md` - 概要
  - `docs/` - ドキュメント
  - `crates/` - Rust実装
  - `examples/` - サンプル

### jco

- Repo: <https://github.com/bytecodealliance/jco>
- Key files:
  - `README.md` - 概要
  - `docs/` - Jco Book
  - `crates/` - Rust実装

### wasm-bindgen

- Repo: <https://github.com/wasm-bindgen/wasm-bindgen>
- Key files:
  - `README.md` - 概要
  - `guide/` - wasm-bindgen Guide
  - `crates/` - Rust実装

## Relative Projects

### quickjs

- Repo: <https://github.com/bellard/quickjs>
- Key files:
  - `README.md` - 概要
  - `doc/quickjs.html` - ドキュメント
  - `quickjs.c` - 単一ファイル実装

### quickjs-ng

- Repo: <https://github.com/quickjs-ng/quickjs>
- Key files:
  - `README.md` - 概要
  - `docs/` - ドキュメント
  - `src/` - 実装

### assemblyscript

- Repo: <https://github.com/AssemblyScript/assemblyscript>
- Key files:
  - `README.md` - 概要
  - `docs/` - ドキュメント
  - `std/` - 標準ライブラリ
  - `compiler/` - コンパイラ実装

### javy

- Repo: <https://github.com/bytecodealliance/javy>
- Key files:
  - `README.md` - 概要
  - `docs/` - ドキュメント
  - `src/` - 実装

## JavaScript + TypeScript tests

### typescript-go

- Repo: <https://github.com/microsoft/typescript-go>
- Key files:
  - `README.md` - 概要
  - `src/` - Go実装
  - `test/` - テスト

### emscripten

- Repo: <https://github.com/emscripten-core/emscripten>
- Key files:
  - `README.md` - 概要
  - `docs/` - ドキュメント
  - `system/` - ランタイム実装

## Spec

### WebAssembly Spec

- Repo: <https://github.com/WebAssembly/spec>
- Key files:
  - `README.md` - 概要
  - `core/` - Core spec
  - `js-api/` - JavaScript API spec

### test262

- Repo: <https://github.com/tc39/test262>
- Key files:
  - `README.md` - 概要
  - `test/` - テストケース
  - `tools/` - テストツール
