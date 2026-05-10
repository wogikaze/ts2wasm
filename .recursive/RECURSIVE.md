# recursive-mode workflow

`ts2wasm` では、大規模なリファクタリング、非同期セマンティクスの実装、または複雑なバグ修正において `recursive-mode` を使用する。

## Non-negotiable Rules

1. **Repo docs are the source of truth.**
    要件と計画は静的ドキュメント（`TRACKING.yaml` または `plans/*.md`）に記述し、プロンプトに長大な仕様を貼り付けない。
2. **One-way phases.**
    フェーズが進んだ後、以前のアーティファクトを直接編集しない。変更が必要な場合は `addenda/` を使用する。
3. **Explicit gates.**
    各フェーズの完了には、Coverage Gate（要件充足）と Approval Gate（承認）が必須。
4. **Worktree isolation.**
    メインブランチを汚さないよう、独立した git worktree で作業を行う。

## Global Artifacts

- `/.recursive/DECISIONS.md`: 完了または中断された全ランの決定ログとインデックス。
- `/.recursive/STATE.md`: アプリケーションとコードベースの現在の真実。

## Recursive Run Layout

`/.recursive/run/<run-id>/`
- `00-requirements.md` / `00-worktree.md`
- `01-as-is.md` / `01.5-root-cause.md` (バグ修正時)
- `02-to-be-plan.md`
- `03-implementation-summary.md` (TDD 必須)
- `04-test-summary.md`
- `05-manual-qa.md`
- `06-decisions-update.md` / `07-state-update.md` / `08-memory-impact.md`

## Phase Definitions

### Phase 0: Isolation & Requirements

- `NEVER WORK ON MAIN/MASTER BRANCH WITHOUT EXPLICIT CONSENT`.
- `mise run check-fast-gate` 等でクリーンなベースラインを確認する。

### Phase 1: Analysis

- 現状の挙動、関連コード、既知の不明点をドキュメント化。

### Phase 2: TO-BE Plan

- 具体的な編集箇所、実行コマンド、追加テストを策定。

### Phase 3: Implementation (TDD)

- **THE IRON LAW: NO PRODUCTION CODE WITHOUT A FAILING TEST FIRST.**
- `strict` モード（RED-GREEN ログ必須）をデフォルトとする。

### Phase 4: Validation

- `cargo nextest`, `mise run gate` 等の全テストを実行。

### Phase 6-8: Closeout

- `DECISIONS.md`, `STATE.md`, メモリを更新。

## Locking

フェーズ完了時は `scripts/recursive-lock.py` を使用して `LOCKED` 状態にし、`LockHash` を記録する。ロックされたファイルは以降編集禁止。
