# Runtime ABI

このドキュメントは ts2wasm の runtime ABI を定める。
`RawValue` の tagged representation、heap レイアウト、`RuntimeFn` カタログ、host import ABI を定義する。

## M0 Value Representation

### RawValue (i32 tagged encoding)

M0 では JavaScript の値を `i32` の tagged encoding として表現する。
これは意図的な **M0 small-int subset** であり、JS 全体の `number` セマンティクスではない。

```text
i32 tagged value (RawValue):

  undefined: 0b000 = 0
  null:      0b001 = 1
  false:     0b010 = 2
  true:      0b011 = 3
  number:    (n << 3) | 0b100   — n は i32 の範囲内の整数のみ
  array:     ptr | 0b101        — ptr はヒープ上の Array object のアドレス (8-byte aligned)
  string:    ptr | 0b110        — ptr はヒープ上の String object のアドレス (8-byte aligned)
  object:    ptr | 0b111        — ptr はヒープ上の Object のアドレス (8-byte aligned)

TAG_MASK:  0b111
HEAP_MASK: !0b111 (= -8 in two's complement)
```

定数は `runtime/value.rs` の `ValueTag` で定義する。
backend が直接数値を埋め込むことは禁止。

### M0 制約（明文）

```text
対応する数値:
  i32 の範囲内の整数（-2^28 ～ 2^28 - 1 の有効な値域 ※ tagged shift 後）

対応しない数値（M0 非対応）:
  浮動小数点数（f64 / double）
  NaN, Infinity, -Infinity
  大整数（BigInt）

文字列変換（$value_to_string_into）の M0 制約:
  数値 → 文字列は 1 桁の非負整数のみ正しく動作する。
  多桁・負数・小数は M0 では未定義動作。
```

将来の対応方針: M3 以降で `f64` boxing または NaN-boxing に移行する予定。
その際は `runtime/value.rs` の `ValueTag` 定義を変更し、全 backend を追従させる。

## Memory Layout

```text
linear memory (4 MiB):

  [0 .. 8)            — iovec 領域 (IOVEC_PTR=8, IOVEC_LEN=12)
  [8 .. DATA_START)   — 予約
  [DATA_START .. SCRATCH_OFFSET)  — static data segment (interned strings)
  [SCRATCH_OFFSET .. HEAP_START)  — scratch buffer ($value_to_string_into 用)
  [HEAP_START .. )    — heap ($heap global, bump allocator)
```

定数は `runtime/layout.rs` の `Layout` で定義する。

| 定数 | 値 | 用途 |
|---|---|---|
| `DATA_START` | 256 | interned string data の開始オフセット |
| `ALIGN` | 8 | data segment / heap の alignment |
| `SCRATCH_OFFSET` | 1500 | 一時バッファの開始オフセット |
| `HEAP_START` | 2048 | heap bump allocator の開始アドレス |
| `IOVEC_PTR` | 8 | iovec の ptr フィールドオフセット |
| `IOVEC_LEN` | 12 | iovec の len フィールドオフセット |

### Heap String Object

interned string / runtime string はヒープ上に以下の形式で配置する。

```text
[offset + 0 .. +4)   : i32 length (バイト数)
[offset + 4 .. +4+N) : UTF-8 bytes

RawValue = ptr | 0b110  (ptr は 8-byte aligned)
```

ptr を取り出すには: `ptr = raw_value & HEAP_MASK`
length を読むには: `len = i32.load(ptr)`
文字列バイトは: `ptr + 4` から `len` バイト

> **M5 制約**: 文字列リテラルは ASCII のみ。`byte_length == JS .length` が保証される。非 ASCII は compile error (`DiagCode::UnsupportedSyntax`)。

### Heap Array Object (M5)

```text
[offset + 0 .. +4)           : i32 element count
[offset + 4 .. +4 + n*4)     : i32 elem₀, elem₁, ...  (RawValue each)

RawValue = ptr | 0b101  (tag 5, ptr は 8-byte aligned)
```

`ARRAY_HEADER_SIZE = 4` (定数は `runtime/layout.rs` の `Layout`)

要素アクセス: `elem_i = i32.load(ptr + 4 + i*4)` — bounds check は `$array_get` が実施。

### Heap Object (M5)

```text
[offset + 0 .. +4)                     : i32 property count
[offset + 4 + i*8 .. +4 + i*8 + 4)    : i32 key_raw  (tagged string RawValue)
[offset + 4 + i*8 + 4 .. +4 + i*8 + 8): i32 value    (any RawValue)

RawValue = ptr | 0b111  (tag 7, ptr は 8-byte aligned)
```

`OBJECT_HEADER_SIZE = 4`, `OBJECT_ENTRY_SIZE = 8`, `OBJECT_VALUE_OFFSET = 4`
(定数は `runtime/layout.rs` の `Layout`)

property lookup は `$property_get` が reverse scan (後勝ち、JS duplicate key semantics)。

## RuntimeFn Catalog

runtime 関数は `RuntimeFn` カタログとして管理する（M1 以降に実装）。
M0 では巨大な WAT template として `runtime_builder.rs` に存在するが、将来は以下の形に移行する。

```rust
pub struct RuntimeFn {
    pub name: RuntimeSymbol,
    pub deps: &'static [RuntimeSymbol],
    pub imports: &'static [HostImport],
    pub capabilities: &'static [Capability],
    pub emit: fn(&mut ModuleBuilder),
}
```

### 現在の runtime 関数一覧（M0–M5）

| 関数 | 依存 | host import | capability | 説明 |
|---|---|---|---|---|
| `$write` | — | `fd_write` | Stdout | ptr/len をバッファに書く |
| `$copy` | — | — | — | メモリ間コピー |
| `$value_to_string_into` | `$copy` | — | — | RawValue → string bytes |
| `$log` | `$value_to_string_into`, `$write` | `fd_write` | Stdout | console.log の実装 |
| `$truthy_bool` | — | — | — | RawValue → bool (JS truthiness) |
| `$not` | `$truthy_bool` | — | — | logical not |
| `$string_equal` | — | — | — | string 同士の等値比較 |
| `$strict_equal` | `$string_equal` | — | — | JS `===` |
| `$concat` | — | — | — | string concat、heap alloc |
| `$is_string` | — | — | — | RawValue が string か判定 |
| `$add` | `$concat`, `$is_string` | — | — | JS `+` |
| `$sub` | — | — | — | JS `-` (number のみ) |
| `$less` | — | — | — | JS `<` (number のみ) |
| `$alloc_heap` | — | — | — | bump allocator; 指定バイト確保し ptr を返す |
| `$mem_equal` | — | — | — | メモリ範囲の byte 比較; string key 照合に使用 |
| `$array_get` | — | — | — | array RawValue + number index → element; tag check あり |
| `$get_length` | — | — | — | string / array RawValue → length (number); tag check あり |
| `$property_get` | `$mem_equal` | — | — | object RawValue + key ptr/len → value; reverse scan; tag check あり |

### Tree-shaking 方針

`console.log` を使っていない program には `$log`, `$write`, `fd_write` を含めない。
`+` 演算子を使っていない program には `$add`, `$concat` を含めない。
これは `RuntimeFn.deps` と `RuntimeFn.capabilities` を静的に解析することで実現する（M1 以降）。

## Host Import ABI

host import は capability manifest から生成する。
backend が直接 import 文字列を持つことは禁止（M2 以降）。

### 現在使用している host import（M0）

| module | name | 型シグネチャ | capability |
|---|---|---|---|
| `wasi_snapshot_preview1` | `fd_write` | `(i32 i32 i32 i32) -> i32` | Stdout |

### API 分類

| 分類 | 説明 | 例 |
|---|---|---|
| Wasm-native | runtime 内で完結する | 算術演算、string concat、===  |
| WASI-backed | fd_read / fd_write / clock_time_get などで実装 | console.log, Date.now |
| Host-shim-backed | Node.js host shim が必要 | process.argv, fs.readFileSync |
| Unsupported | compile error。必要なら fallback plan を表示 | crypto.createHash (M0) |

この分類は `docs/03-api-and-host-capability.md` と `docs/11-shared-definitions.md` に正式定義する。
backend が API 分類を決めることは禁止。必ず capability manifest / semantic pass を通す。

## Bump Allocator

`$heap` global は `HEAP_START` 初期値を持つ。
string alloc 時は以下の手順で行う。

```text
1. align_to($heap, ALIGN) → base
2. i32.store(base, len)
3. $copy(src, base + 4, len)
4. $heap = align_to(base + 4 + len, ALIGN)
5. return base | STRING_TAG
```

M0 では GC を実装しない。bump allocator のみ。
heap overflow の検査は M1 以降。

> **M5 現状**: `$alloc_heap` は `memory.size` を参照しない。配列・オブジェクトのサイズが線形メモリを超えた場合の動作は未定義。heap OOM check は P0 負債として M6 前に対処する。

## 将来の Value Representation 移行方針

M0 → M3+ での段階的拡張:

```text
M0: i32 tagged small-int
M3: f64 NaN-boxing または boxed f64 ヒープ object
M4: i64 tagged encoding（HeapPtr 拡張時）
```

移行時は `runtime/value.rs` の `ValueTag` 定義を変更し、
`encode_number`, `decode_number`, `is_string`, `is_number` などのプリミティブを更新する。
backend は `ValueTag` のみを参照し、raw tag 値を持たないため、移行コストを最小化できる。
