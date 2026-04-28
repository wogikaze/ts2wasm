# 開発ループレポート: 052-json-replacer-array-multikey-20260428T090325Z

## 状態

- Issue: 052
- 状態: PROGRESS
- Branch: agent/052-json-replacer-array-multikey-20260428T090325Z
- Implementation commit: 3e3c4ae

## 目的

`JSON.stringify({ a: 1, b: 2, c: 3 }, ["c", "a"])` の object-literal array replacer subset を実装し、Node/iwasm differential evidence を追加する。

## 実施内容

- `crates/ir/src/lowered.rs` で string-literal array replacer を複数キーまで許可した。
- object literal filtering は replacer array order を優先し、重複 replacer key は一度だけ出力する。
- `fixtures/builtins-and-io/json-stringify-replacer-array-multikey.ts` を追加し、`crates/cli/tests/m2_node_diff.rs` の JSON fixture set に追加した。
- issue 052 に progress evidence を追記した。

## 判断と根拠

- 現行 runtime helper は replacer callback/property-list 全体を表現していないため、既存の IR-time object-literal rewrite を拡張する最小 slice にした。
- Lowered `ObjectNew` の property order を使えるため、assigned case の replacer order `["c", "a"]` は安全に保持できる。
- 非 string array entry と function replacer は引き続き issue-052 `UnsupportedSyntax` diagnostic として拒否する。

## 詰まり・ロス

なし。

## リスク

- Full replacer semantics は未実装のまま。今回の対応は object literal + string-literal array property-list subset に限定している。
- Runtime object/string model の制約により、JSON number/string representation gaps は issue 052 の残作業として継続する。

## 次にやるべきこと

- issue 052 の残 gap: non-integer number representation, full UTF-16/surrogate handling, broader replacer semantics, throw-compatible parse diagnostics。
- Parent branch への merge review を依頼する。

## 完了・追加

done: なし
progress: issue 052 JSON.stringify multi-key array replacer object-literal subset
new: なし

## Evidence

Pre-change reproduction:

```text
node -e 'console.log(JSON.stringify({ a: 1, b: 2, c: 3 }, ["c", "a"]))'
=> {"c":3,"a":1}

cargo run -q -p ts2wasm-cli -- build /tmp/ts2wasm-json-replacer-array-multikey.ts -o /tmp/ts2wasm-json-replacer-array-multikey.pre.wasm
=> [UnsupportedSyntax] issue-052: JSON.stringify array replacer property lists outside the single string-literal object subset are not supported yet
```

New fixture output:

```text
{"c":3,"a":1}
```

Validation:

- `cargo fmt --all --check`: pass
- `cargo nextest run -E 'test(json)'`: pass, 17 passed
- `cargo nextest run -p ts2wasm-cli json`: pass, 14 passed
- `node fixtures/builtins-and-io/json-stringify-replacer-array-multikey.ts`: pass
- `cargo run -q -p ts2wasm-cli -- build fixtures/builtins-and-io/json-stringify-replacer-array-multikey.ts -o /tmp/ts2wasm-json-replacer-array-multikey.wasm`: pass
- `iwasm /tmp/ts2wasm-json-replacer-array-multikey.wasm`: pass
- `cargo run -q -p ts2wasm-cli -- build fixtures/builtins-and-io/json-stringify-replacer-array-unsupported.ts -o /tmp/ts2wasm-json-replacer-array-unsupported.wasm`: expected `UnsupportedSyntax`, status 1
- `scripts/manager update-issue-index --check`: pass
- `scripts/manager check-issue-health`: pass
- `scripts/manager check-agent-state`: pass
- `cargo nextest run`: pass, 362 passed, 4 skipped
