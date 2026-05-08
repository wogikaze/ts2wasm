---
id: 3408
title: "Close multipleBaseInterfaesWithIncompatibleProperties as stale build-pass"
type: maintenance
area: frontend/syntax
class: done
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-08
completed: 2026-05-08
status: done
---

## Summary

Closed the generated `multipleBaseInterfaesWithIncompatibleProperties` bucket as stale. The representative reference path now build-passes and the TypeScript oracle reports no diagnostics.

## Problem

The original generated bucket recorded an import/export blocker for `multipleBaseInterfaesWithIncompatibleProperties2.ts`. Fresh focused triage no longer reproduces that blocker.

Focused coverage:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/multipleBaseInterfaesWithIncompatibleProperties2.ts --detail --no-dashboard-data
result: executed=1, build_pass=1, unsupported=0, blocked=0
date: 2026-05-08
```

Focused triage:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/multipleBaseInterfaesWithIncompatibleProperties2.ts
result: BuildPass; ts2wasm build succeeded
date: 2026-05-08
```

## Evidence

Source shape:

```ts
namespace http {
  export interface TcpSocketConnectOpts {
    port: number;
  }

  export interface AgentOptions extends Partial<TcpSocketConnectOpts> {
    keepAlive?: boolean | undefined;
  }
}

namespace tls {
  export interface ConnectionOptions {
    port?: number | undefined;
  }
}

interface AgentOptions extends http.AgentOptions, tls.ConnectionOptions {
  maxCachedSessions?: number | undefined;
}
```

Compiler evidence:

```text
tokens: ok through namespace/interface declarations and exactOptionalPropertyTypes directive
ast: []
resolved: []
typescript oracle: ok=true, diagnostics=[]
```

No child issue is needed because there is no current compiler blocker or TypeScript oracle diagnostic for this representative path.

## Validation

Issue sync and health checks:

```text
python scripts/manager.py update-issue-index
git diff --check
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
```

Focused reference checks:

```text
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/multipleBaseInterfaesWithIncompatibleProperties2.ts --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/multipleBaseInterfaesWithIncompatibleProperties2.ts
```

Rust gates were not run because this slice only changes issue metadata.

## Completion evidence

Commits:

- filled by commit

Remaining risks:

- none for this representative path.
