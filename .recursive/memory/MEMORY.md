# MEMORY.md

## Memory Router

`ts2wasm` の長期記憶を管理する。

### Registry

- `domains/`: 機能領域ごとの安定知識。
- `patterns/`: 再利用可能な解決パターン。
- `incidents/`: 過去の不具合とその対策。
- `skills/`: エージェントスキルの利用知見。

### Retrieval Rules

1. タスクに関連するメモリのみをロードする。
2. `Status: CURRENT` を優先し、`SUSPECT` は再検証する。
