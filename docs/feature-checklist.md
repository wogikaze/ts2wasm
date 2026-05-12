# Feature Implementation Checklist

新機能を追加する際は、以下の checklist を 1 項目ずつ確認する。
各項目は「終わった」ではなく「証拠がある」で判断する。

## 1. Syntax Impact

- [ ] Parser 変更が必要か？（AST variant / token / grammar）
- [ ] 必要な場合、parse snapshot を更新した
- [ ] 不要な場合、既存構文で表現できることを確認した

## 2. Name Resolution

- [ ] 新しい global name が必要か？（default_allowed_globals に追加）
- [ ] 新しい scope / binding rule が必要か？

## 3. Builtin Resolution

- [ ] source pattern （console.log / Math.* / obj.method 等）を定義した
- [ ] arity / result contract を定義した
- [ ] unsupported diagnostics を追加した
- [ ] negative test （誤認識防止）を追加した

## 4. HIR / Lowered IR

- [ ] semantic op / LoweredExpr variant の定義
- [ ] validator の更新
- [ ] debug / snapshot printer の更新
- [ ] parse snapshot の更新
- [ ] semantic snapshot の更新
- [ ] lowered snapshot の更新

## 5. Runtime Catalog

- [ ] RuntimeFn variant を追加した
- [ ] RuntimeSpec （deps / imports / capabilities / runtime_strings / result）を定義した
- [ ] emission_order / all に追加した
- [ ] emit function を追加した
- [ ] linker structure test を追加した

## 6. Capability / Host Import

- [ ] 新しい host import が必要か？（HostImport enum に追加）
- [ ] 新しい capability が必要か？（Capability enum に追加）
- [ ] capability manifest test を追加した
- [ ] host import が必要時だけ WAT に出る test を追加した
- [ ] host import が不要時に WAT に出ない test を追加した

## 7. Differential Test

- [ ] Node vs iwasm の差分テスト（semantic_diff）を追加した
- [ ] fixture を作成し console.log で観測可能にした

## 8. Negative Tests

- [ ] unsupported callback shape の診断 test
- [ ] unsupported receiver の診断 test
- [ ] arity mismatch の診断 test

## 9. Documentation

- [ ] docs/05-compatibility-and-semantics.md を更新した
- [ ] docs/language-reference/ の該当機能表を更新した
- [ ] current-state.md を更新した
- [ ] 設計判断を docs/ に残した（ADR または該当 design doc）

## 10. Coverage

- [ ] reference coverage の diagnostic 分類が改善した
- [ ] build_pass が増えた（または既存件数が維持された）
- [ ] semantic_pass が増えた（または既存件数が維持された）
