---
id: 5004
title: "Meta: Runtime Builtins Coverage (test262)"
type: meta
area: runtime/builtins
class: design-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-02
updated: 2026-05-02
---

## Summary

Covers missing JavaScript runtime builtin implementations (~40 triage-needed issues + 6 blocked issues). These are test262 failures requiring builtin function/object implementations in the runtime-abi layer.

## Problem

~46 reference test262 test families fail due to missing or incomplete runtime builtin implementations including Array, String, Object, Date, JSON, RegExp, Math, and others.

## Scope

In scope:

- Builtin JavaScript object and function implementations
- Runtime-abi and backend integration for builtins
- Individual child issues for each builtin family

Out of scope:

- Parser/frontend changes
- Non-builtin runtime semantics

## Affected paths

Expected:

- `crates/runtime-abi/src/`

## Acceptance criteria

- [ ] Builtin implementation coverage increases per child issue resolution

## Validation

```sh
mise run reference-coverage -- test262 --limit 50 --detail
```
