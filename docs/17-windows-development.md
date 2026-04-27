## Windows 開発環境セットアップ

### 1. 必要ツールのインストール

#### Python 3

```powershell
# python.org から Python 3.10+ をインストール
# https://www.python.org/downloads/

# インストール確認
python --version
```

#### Rust Toolchain

```powershell
# rustup-init.exe をダウンロードして実行
# https://rustup.rs/
# または winget 使用:
winget install Rustlang.Rustup

# cargo-nextest インストール
cargo install cargo-nextest
```

#### Node.js

```powershell
# nodejs.org から LTS 版をインストール
# https://nodejs.org/

# 確認
node --version
npm --version
```

#### Git

```powershell
# git-scm.com からインストール
# https://git-scm.com/

# 確認
git --version
```

#### 追加ツール

```powershell
# jq: https://stedolan.github.io/jq/download/ からダウンロード
# ripgrep: https://github.com/BurntSushi/ripgrep/releases からダウンロード
# wasm-tools:
cargo install wasm-tools
# ast-grep:
cargo install ast-grep
# WAMR (iwasm): https://github.com/bytecodealliance/wasm-micro-runtime/releases から Windows バイナリをダウンロード
```

すべてのツールを PATH に追加すること。

### 2. jsonschema のインストール（重要）

```powershell
python -m pip install jsonschema
```

これは `check-agent-state` で必須です。

### 3. リポジトリのクローン

```powershell
git clone <repository-url>
cd ts2wasm
```

### 4. Python スクリプトマネージャーの使用

bash 依存を避けるため、[scripts/manager.py](cci:7://file:///home/wogikaze/ts2wasm/scripts/manager.py:0:0-0:0) を直接使用します。

#### 基本コマンド

```powershell
# コードフォーマット
python scripts/manager.py fmt

# Clippy 実行
python scripts/manager.py clippy

# テスト実行
python scripts/manager.py nextest

# ファストゲート（fmt + issue health + architecture + coverage matrix + tests）
python scripts/manager.py check-fast-gate

# テストをスキップしてファストゲート
python scripts/manager.py check-fast-gate --skip-nextest

# ヘルプ表示
python scripts/manager.py help
```

#### 利用可能なコマンド

- `fmt` - cargo fmt --all --check
- `clippy` - cargo clippy --all-targets
- `nextest` - cargo nextest run
- `check-fast-gate` - fmt + issue health + architecture + coverage matrix + nextest
- `check-issue-health` - issues/ ディレクトリの検証
- `update-issue-index` - issues/index.md の再生成
- `check-agent-state` - エージェント状態の検証
- `check-repo-smoke` - fmt + check-scripts + check-issue-health
- その他多数（`python scripts/manager.py help` で確認）

### 5. 開発ワークフロー

#### 典型的な開発サイクル

```powershell
# 1. コードフォーマット
python scripts/manager.py fmt

# 2. リンター実行
python scripts/manager.py clippy

# 3. テスト実行
python scripts/manager.py nextest

# 4. フルゲート実行（コミット前）
python scripts/manager.py check-fast-gate
```

#### Issue 管理

```powershell
# Issue 検証
python scripts/manager.py check-issue-health

# Issue index 更新
python scripts/manager.py update-issue-index

# カバレッジから Issue 生成
python scripts/manager.py gen-issues-from-coverage --suite test262
```

### 6. 開始前の最終確認

```powershell
# Issue index 検証
python scripts/manager.py update-issue-index --check

# Issue health 検証
python scripts/manager.py check-issue-health

# Agent state 検証
python scripts/manager.py check-agent-state

# Repo smoke 検証
python scripts/manager.py check-repo-smoke

# ファストゲート（テストスキップ）
python scripts/manager.py check-fast-gate --skip-nextest
```

これら5つがすべて通れば開発を開始できます。

#### "bash not found" でスクリプト失敗

実行しようとしているスクリプトが bash 依存。Python マネージャー（`python scripts/manager.py`）を使用するか、完全機能のために WSL2 を使用。

### 8. 制限事項

すべての高優先度スクリプトは Python に移行済みで Windows 互換です。

**Python 移行済み（Windows互換）:**

- check-toolchain, check-manifest-imports, check-test-records-schema
- check-fixture-catalog, check-architecture-rules, check-compiler-diagnostics
- check-harness-installation, check-fixture-differential, check-host-deny
- check-runtimefn-invariants, check-wasm-validation, check-agent-state
- check-issue-health, update-issue-index, gen-issues-from-coverage
- update-coverage-matrix, install-hooks, check-scripts
- check-repo-smoke, check-coverage-gate, coverage-report
- reference-coverage, test262, test-differential-reporter
- test-regression-gate, benchmark-tracker, fmt, clippy, nextest
- check-fast-gate

test262 とカバレッジレポートの完全機能には WSL2 の使用を検討してください。WSL セットアップはメイン README を参照。

### 9. 代替案: Mise 使用（実験的）

```powershell
# mise インストール
winget install jdx.mise

# mise tasks 使用
mise tasks
mise run fmt
mise run nextest
```

注意: 一部の mise タスクはまだ bash スクリプトを呼び出し、Windows で動作しない可能性があります。
