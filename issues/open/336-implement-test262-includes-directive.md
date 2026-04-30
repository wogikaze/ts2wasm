---
id: 336
title: "Implement test262 includes directive processing"
type: feature
area: cli/reference
class: partially-complete
priority: P1
depends_on: []
blocks: []
created: 2026-04-30
updated: 2026-04-30
---

## Summary

Implement test262 `includes:` directive to load helper files (e.g., propertyHelper.js) that define test helper functions like `verifyProperty`, `assert`, etc.

## Problem

Test262 tests use `includes: [propertyHelper.js]` metadata to load helper files that define functions like `verifyProperty`, `assert.sameValue`, etc. The compiler currently ignores this directive, causing UnresolvedName diagnostics when these functions are referenced.

Problem: test262 `includes:` directive is ignored, causing UnresolvedName for helper functions like `verifyProperty`.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- test262 reference/test262/test/annexB/built-ins/Date/prototype/getYear/B.2.4.js
```

Current result:

```text
error: [UnresolvedName] unresolved name: `verifyProperty` at 317..331
```

Source context:

```javascript
/*---
includes: [propertyHelper.js]
---*/

verifyProperty(Date.prototype, "getYear", {
  enumerable: false,
  writable: true,
  configurable: true
});
```

Visible symbols before failure: `[]` (helper file not loaded)

## Desired final state

Test262 `includes:` directive is processed during compilation, and helper files are pre-parsed to make their functions available in the test file's scope.

## Scope

In scope:

- [ ] Parse test262 YAML frontmatter to extract `includes:` directive
- [ ] Resolve include file paths relative to test262 helper directory
- [ ] Pre-parse included helper files and merge their symbol table with test file
- [ ] Update diagnostic to only emit UnresolvedName when name is truly unresolved

Out of scope:

- test262 `features:` directive (separate issue)
- test262 `$262` object (separate issue)
- Non-test262 file processing

## Affected paths

Expected:

- `crates/cli/src/` (reference runner)
- `scripts/run/reference-triage.py` (may need updates for includes processing)
- `reference/test262/harness/` (helper files)

Do not touch:

- `crates/frontend/src/` (unless includes processing requires parser changes)
- `crates/runtime-abi/`
- `crates/backend-wasm/`

## Acceptance criteria

- [x] `verifyProperty` and other helper functions resolve without UnresolvedName diagnostic
- [ ] Representative test `reference/test262/test/annexB/built-ins/Date/prototype/getYear/B.2.4.js` builds successfully (blocked by issue 050)
- [ ] At least 50 test262 tests with `includes:` directive transition from unsupported to build_pass (requires full helper parsing)
- [x] Regression test added for includes processing

**Partially complete:** Helper functions resolve using hardcoded stubs, but full helper file parsing and comprehensive coverage require parser support for more complex JavaScript syntax.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-triage -- test262 reference/test262/test/annexB/built-ins/Date/prototype/getYear/B.2.4.js
mise run reference-coverage -- test262 --path-filter reference/test262/test/annexB/built-ins/Date/prototype/ --detail
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [ ] not affected

Current state:

- [ ] not affected

Follow-up issues:

- [ ] created: `issues/open/337-implement-test262-features-directive.md`

## Notes

Test262 helper files are located in `reference/test262/harness/`. Common helpers include:
- `propertyHelper.js` (verifyProperty, assert.sameValue)
- `assert.js` (assert module)
- `isConstructor.js` (isConstructor helper)

**Implementation approach:**
- Created test262 preprocessor in `crates/compiler/src/test262_preprocessor.rs`
- Preprocessor extracts `includes:` directive from YAML frontmatter
- Currently uses hardcoded function stubs instead of full helper file parsing
- Stub approach was necessary because helper files contain complex syntax that the parser cannot yet handle
- Helper functions are now resolved (verifyProperty, verifyCallableProperty, assert)
- Representative test still fails due to Date being UnresolvedName (issue 050), not helper resolution

**Known limitations:**
- Current implementation uses hardcoded stubs for common helper functions
- Full helper file parsing requires parser support for more complex JavaScript syntax
- Some helper functions (assert.sameValue, etc.) are not yet stubbed

## Completion evidence

**Completed 2026-04-30**

Commits:

- `12e419b0` implement: test262 includes directive processing

Validation result:

```text
command: cargo fmt --all --check
result: pass
date: 2026-04-30

command: cargo nextest run
result: pass (606 tests)
date: 2026-04-30

command: mise run reference-coverage -- test262 --path-filter reference/test262/test/annexB/built-ins/Date/prototype/getYear/ --detail
result: executed=7, build_pass=0, unsupported=7 (all UnsupportedSyntax: date)
date: 2026-04-30

command: mise run update-issue-index
result: pass
date: 2026-04-30

command: mise run check issues
result: pass
date: 2026-04-30
```

Remaining risks:

- Date UnsupportedSyntax requires issue 050 (Implement Date)
- Hardcoded function stubs limit full helper functionality
- Some helper functions (assert.sameValue, etc.) not yet stubbed
