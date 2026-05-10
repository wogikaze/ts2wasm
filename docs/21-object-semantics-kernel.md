# Object Semantics Kernel (W5)

この文書は ts2wasm における JavaScript オブジェクト意味論の中核レイヤー（Kernel）の設計を定める。

## 背景と目的

現在の ts2wasm のオブジェクト実装は、単純なプロパティのマップ操作に基づいている。しかし、JavaScript の完全なセマンティクス（プロトタイプ継承、プロパティ記述子、ゲッター/セッター等）を実現するには、個別の操作がバラバラに実装されている状態では限界がある。

本 Epic の目的は、すべてのオブジェクト操作を ECMAScript 仕様に準拠した共通の **Runtime Internal Operations (Ordinary*)** に集約し、今後の高度な機能拡張のための堅牢な土台を作ることである。

## 設計方針

### 1. 集中管理 (Centralization)

`obj.x`, `obj.x = v`, `delete obj.x`, `"x" in obj` などの操作を下層のマップ操作に直接変換せず、必ずランタイムの共通内部関数を経由させる。

### 2. 段階的機能解放

データ記述子（writable, enumerable, configurable）を先に導入し、その後にプロトタイプチェーン、アクセス記述子（getter/setter）を順次統合する。

### 3. Backend 独立性

意味論をランタイム関数（WAT/WASM）レベルで固定することで、将来的な Wasm GC バックエンドへの移行時にも、コンパイラ側の Lowering ロジックを最小限の変更で維持できる。

## 中核となる内部操作 (Core Operations)

ECMAScript 仕様に基づき、以下の内部操作をランタイムに定義する。

| 操作 | 論理シグネチャ | 説明 |
|---|---|---|
| `OrdinaryGet` | `(obj, key, receiver) -> jsval` | プロトタイプ探索を含む値の取得 |
| `OrdinarySet` | `(obj, key, value, receiver) -> status` | 属性と継承を考慮した値の設定 |
| `OrdinaryHasProperty` | `(obj, key) -> bool` | `in` 演算子の意味論 |
| `OrdinaryDelete` | `(obj, key) -> bool` | プロパティの削除 |
| `OrdinaryDefineOwnProperty` | `(obj, key, desc) -> status` | プロパティ記述子の定義 |
| `OrdinaryGetOwnProperty` | `(obj, key) -> desc_obj` | 自身のプロパティ記述子の取得 |
| `OrdinaryOwnPropertyKeys` | `(obj) -> array` | 自身のキー一覧（文字列）の取得 |
| `OrdinaryGetPrototypeOf` | `(obj) -> proto_obj` | `[[Prototype]]` の取得 |
| `OrdinarySetPrototypeOf` | `(obj, proto) -> status` | `[[Prototype]]` の変更 |

## 実装フェーズ (Phases)

### W5.0: 現行パスの監査（done 2026-05-09）

- 既存の `get`/`set`/`delete`/`in` のコンパイラ・ランタイムパスを棚卸しし、共通化を阻害している箇所を特定する。
- **監査結果**:
  - `$property_get`（`runtime_collections.rs:191-258`）: プロトタイプチェーン探索、backward scan、64段 depth limit 完備。`OrdinaryGet` 相当として使用可能。
  - `$property_set`（`runtime_collections.rs:260-361`）: 配列/オブジェクト分岐、既存キー上書き、新規追記完備。FROZEN フラグチェック追加済み（id 184）。
  - `$property_delete`（`runtime_collections.rs:363-426`）: backward scan、エントリ削除、count decrement 完備。FROZEN フラグチェック追加済み。
  - `LoweredExpr::PropertyGet` / `PropertySet` / `PropertyDelete` / `PropertyDeleteDynamic`: コンパイラ側の lowering は既存の runtime call 経由で統一済み。
  - **未統合**: `array.length` と `array[n]` の fast path は `$property_get`/`$property_set` をバイパス。W5.2 で統合検討。
  - **ブロッカー**: `Object.defineProperty` の WAT emitter に括弧バグ（id 174）。修正後に W5.2 の OrdinaryDefineOwnProperty 統合を再開。

### W5.1: プロパティ記述子の導入

- Data descriptor: `value`, `writable`, `enumerable`, `configurable`
- `Object.defineProperty` / `Object.getOwnPropertyDescriptor` の最小実装。

### W5.2: 操作の集約 (Centralized Lowering)

- `obj.x`, `obj["x"]`, `obj.x = v` などをすべて `OrdinaryGet`/`OrdinarySet` 経由に統一する。

### W5.3: プロトタイプチェーンの統合

- `[[Prototype]]` スロットの導入。
- `Object.create(proto)` および継承を伴う `Get`/`HasProperty` の実装。

### W5.4: 属性フラグの強制 (Enforcement)

- `writable: false` への代入拒否、`configurable: false` の削除拒否などを実装。

### W5.5: 列挙とキー取得 (Enumeration)

- `Object.keys`, `Object.getOwnPropertyNames` の実装。
- `enumerable` 属性に基づくフィルタリング。

### W5.6: アクセス記述子 (Getter/Setter)

- `get`, `set` プロパティの呼び出し。
- `this` (receiver) の正しい束縛。

### W5.7: クラス・プロトタイプ統合

- クラスメソッドを `C.prototype` へ放出。
- `extends` によるプロトタイプリンクの構築と `super` プロパティアクセス。

## 成功条件 (Success Gates)

- **Gate W5-A**: 全プロパティ操作の Lowering が共通ランタイム関数経由になる。
- **Gate W5-C**: プロトタイプチェーンを介したプロパティ取得が Node.js と一致する。
- **Gate W5-F**: ゲッター/セッターの MVP が動作し、副作用が正しく発生する。
- **Gate W5-G**: クラス継承を伴うメソッド呼び出しが正常に動作する。

## 非目標 (Non-goals)

- `Proxy` / `Reflect` の完全実装（別 Epic）。
- `Array` 特有の `length` セマンティクス（Exotic object として分離）。
- Wasm GC バックエンドへの即時移行。
- シンボルプロパティの完全なサポート。
