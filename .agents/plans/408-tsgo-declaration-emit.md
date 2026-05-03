# Issue 408: Implement tsgo declaration emit — AsConstSatisfies/const generic method cases

## Context

Two tsgo `declaration-emit` cases:
- `declarationEmitAsConstSatisfiesNonReadonlyResult.ts`
- `declarationEmitConstObjectLiteralGenericMethod1.ts`

Both fail with `UnsupportedSyntax: declaration-emit`.

## Plan

### Phase 1: Investigate the current failure

Run the tsgo reference coverage to see the actual diagnostics:

```sh
mise run reference-coverage -- tsgo --limit 166 --detail --no-web-ui | rg 'declarationEmit'
```

### Phase 2: Determine fix scope

Check whether these require upstream parser changes, IR changes, or are already supported by adjusting test classification.

### Phase 3: Implement

Add declaration-emit constructs as needed.

### Phase 4: Verify

```sh
mise run reference-coverage -- tsgo --limit 166 --detail --no-web-ui | rg 'declarationEmit'
cargo fmt --all --check
cargo nextest run
```
