# child-062e cycle report

## 状態

PROGRESS。

`issues/open/062e-function-closures.md` は完了扱いにしていない。今回の
コミットは `062e-a` 相当の狭い進捗で、非 escape の nested ordinary
function が immutable outer local を capture して、宣言元 activation が
戻る前に呼ばれるケースだけを実装した。

## 実装内容

- nested ordinary function declaration を AST validation/name resolution/lowering
  で通すようにした。
- nested ordinary function の immutable capture を、既存の local-arrow
  devirtualized closure と同じく generated function + hidden capture params
  へ lowering した。
- returned escaping closure は `issue-062e:` diagnostic として拒否した。
- captured outer local mutation は hidden param では JS の共有環境 semantics を
  表せないため `issue-062e:` diagnostic として拒否した。
- Node/iwasm differential fixture を追加した。
- heap closure object ABI/rooting 用の follow-up として
  `issues/open/062g-heap-closure-object-abi-and-rooting.md` を追加した。
- `current-state.md` と `issues/index.md` を同期した。

## 検証

- `cargo fmt --all --check`
  - PASS
- `cargo nextest run -E 'test(closure) or test(function) or test(node_diff)'`
  - PASS: 27 passed, 403 skipped
- `cargo nextest run`
  - PASS: 426 passed, 4 skipped
- `mise run update-issue-index`
  - PASS: `issues/index.md` updated
- `mise run check issues`
  - FAIL: unrelated existing issue-health failures
  - failures:
    - `issues/open/052-implement-json.md` references missing local report paths:
      - `reports/runs/052-json-array-object-20260428T074900Z/`
      - `reports/runs/052-json-stringify-nested-20260428T080100Z/`
      - `reports/runs/052-json-replacer-array-20260428T083349Z/`
      - `reports/runs/052-json-number-space-20260428T094954Z/`
      - `reports/runs/052-json-close-slice-20260428T133852Z/`
      - `reports/runs/052-json-replacer-next-20260428T135136Z/`
    - `issues/done/228-implement-logical-assignment-operators.md` references missing
      `reports/runs/228-logical-assignment-audit-20260428T100229Z/cycle_report.md`
- `mise run check issue-index`
  - FAIL with the same unrelated missing report path failures.

## 残作業 / blocker

`062e` の original acceptance はまだ満たしていない。returned closure が
declaring scope return 後も captured values を保持するには、現在の hidden
param lowering では足りず、heap closure object + environment ABI + GC rooting
が必要。これは follow-up `062g` に分離した。

次アクション:

- 親へこの PROGRESS commit の merge を依頼する。
- `062g` で heap closure object ABI/rooting を設計・実装する。
- unrelated issue-health failure は `052` / `228` の report path 整合を別途直す。
