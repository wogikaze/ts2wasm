# Cycle report: 20260428-020c-hir-validation

## 状態

Issue 020c と親 issue 020 を完了。Semantic HIR の validation pass を追加し、対応済み subset では build pipeline から検証するようにした。

## 目的

020b で追加した Semantic HIR に対し、IR contract の破損を検出できる validation pass と docs/13 の契約記述を揃える。

## 実施内容

- `validate_hir` を追加し、invalid local/function ID、top-level return、暗黙 truthiness branch、関数 arity/local table 破損を検出するようにした。
- HIR に許可済み builtin receiver 用の `LoadBuiltin` を追加し、`Math` / `JSON` / `String` の method call を未解決 local と誤判定しないようにした。
- compiler build pipeline に、対応済み HIR subset の validation を追加した。未対応 syntax は既存 `LoweredProgram` pipeline を維持する。
- `docs/13-ir-contracts.md` に IR contracts validation summary を追加し、HIR validation の責務を明記した。
- `fixtures/core-semantics/ir-test.ts` を Node differential の core semantic fixture list に追加し、親 issue 020 の acceptance を満たした。
- issue 020c と 020 を done に移動し、issue index を同期した。

## 判断と根拠

HIR coverage はまだ初期 slice なので、build pipeline では HIR lowering が `UnsupportedSyntax` を返した場合は既存 pipeline を継続する。これにより、対応済み HIR の invariant は早期に検出しつつ、未対応構文の既存 build は壊さない。

## 検証

- PASS: `cargo nextest run -p ts2wasm-ir`（13 passed）
- PASS: `cargo nextest run -p ts2wasm-cli --test m6_builtin_methods build_smoke_math_floor_method build_smoke_json_parse_method string_from_char_code_method_emits`（3 passed / 24 skipped）
- PASS: `cargo nextest run -p ts2wasm-cli --test m2_node_diff m3_semantic_fixtures_match_node_output_under_iwasm`
- PASS: `cargo run -q -p ts2wasm-cli -- build fixtures/core-semantics/ir-test.ts -o /tmp/ir-test.wasm`
- PASS: `grep -A 20 "IR contracts" docs/13-ir-contracts.md`
- PASS: `cargo nextest run --no-fail-fast`（211 passed, 4 skipped）

## リスク

backend はまだ `LoweredProgram` を消費しており、Semantic HIR の全面的な backend 接続は後続作業。今回の validation は対応済み HIR subset に限定している。

## 次にやるべきこと

Ready queue から次の実装 slice を選び、引き続き最小の acceptance 単位で進める。

## 完了・追加

完了: issue 020c、issue 020。追加: `validate_hir`、HIR builtin receiver 表現、docs/13 validation summary、Node differential fixture 登録。
