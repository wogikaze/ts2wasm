---
id: 064
title: "Implement name resolution (triaged - superseded by test262 metadata issues)"
type: spike
area: frontend/resolver
class: blocked
priority: P1
depends_on: []
blocks: []
created: 2026-04-26
updated: 2026-04-30
---

## Summary

**This issue has been triaged and superseded by child issues 336 and 337.** The original "name resolution" failures were actually caused by missing test262 metadata processing (includes: and features: directives), not by core name resolution functionality.

## Problem

Reference test results showed 72 cases failing with name-resolution diagnostic. Investigation revealed these failures were caused by test262 metadata processing gaps, not core name resolution issues.

Problem: triage revealed that "name resolution" failures are actually test262 metadata processing issues (includes: and features: directives).

## Current failure

**Triage completed:** Representative cases showed two distinct failure patterns:

1. **includes: directive failure** (e.g., `verifyProperty`):

   ```sh
   mise run reference-triage -- test262 reference/test262/test/annexB/built-ins/Date/prototype/getYear/B.2.4.js
   ```

   Result: `UnresolvedName: unresolved name: verifyProperty`
   Cause: `includes: [propertyHelper.js]` ignored

2. **features: directive failure** (e.g., `$262.IsHTMLDDA`):

   ```sh
   mise run reference-triage -- test262 reference/test262/test/annexB/built-ins/Object/is/emulates-undefined.js
   ```

   Result: `UnsupportedTest262Metadata: test262 feature IsHTMLDDA is not supported`
   Cause: `features: [IsHTMLDDA]` ignored, `$262` object not provided

## Desired final state

This issue has been split into implementation-ready child issues:
- **336**: Implement test262 includes directive processing
- **337**: Implement test262 features directive and $262 object

## Scope

In scope:

- [x] Run representative `mise run reference-triage -- ...` commands
- [x] Confirm failure patterns are test262 metadata issues, not core name resolution
- [x] Split into child issues 336 and 337
- [x] Carry source context, diagnostic code, and validation commands into child issues

Out of scope:

- Direct implementation from this generated bucket (superseded by child issues)
- Core name resolution changes (issue 056 already covers basic name resolution)

## Affected paths

Expected:

- (Handled by child issues 336 and 337)

Do not touch:

- (Handled by child issues 336 and 337)

## Acceptance criteria

- [x] Duplicate candidates confirmed as no-match (triage revealed test262 metadata issue)
- [x] Child issues 336 and 337 created with exact `mise run reference-triage -- ...` commands
- [x] Child issues include failing path, diagnostic code, source context, and validation commands
- [x] Child issue acceptance names exact fixture/reference path and diagnostic/stdout change

## Validation

Required commands:

```sh
mise run update-issue-index
mise run check issues
```

Impacted commands:

- (Handled by child issues 336 and 337)

Not run:

- none (triage complete)

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] created: `issues/open/336-implement-test262-includes-directive.md`
- [x] created: `issues/open/337-implement-test262-features-directive.md`

## Notes

**Triage findings:**
- Core name resolution (issue 056) already implements basic variable/function resolution
- These 72 failures are specifically test262 metadata processing issues
- Pattern 1: `includes: [propertyHelper.js]` causes UnresolvedName for helper functions
- Pattern 2: `features: [IsHTMLDDA]` causes UnsupportedTest262Metadata for $262 object
- Both patterns require test262 runner infrastructure changes, not core compiler changes

## Affected test files

- `reference/test262/test/annexB/built-ins/Date/prototype/getYear/B.2.4.js`
- `reference/test262/test/annexB/built-ins/Date/prototype/getYear/length.js`
- `reference/test262/test/annexB/built-ins/Date/prototype/getYear/name.js`
- `reference/test262/test/annexB/built-ins/Date/prototype/setYear/B.2.5.js`
- `reference/test262/test/annexB/built-ins/Date/prototype/setYear/length.js`
- `reference/test262/test/annexB/built-ins/Date/prototype/setYear/name.js`
- `reference/test262/test/annexB/built-ins/Date/prototype/toGMTString/value.js`
- `reference/test262/test/annexB/built-ins/Object/is/emulates-undefined.js`
- `reference/test262/test/annexB/built-ins/RegExp/prototype/compile/B.RegExp.prototype.compile.js`
- `reference/test262/test/annexB/built-ins/RegExp/prototype/compile/length.js`
- ... and 62 more files

## Duplicate detection

- none found by path/title/feature scan

## Completion evidence

**Triage completed 2026-04-30**

Commits:

- (none - triage only, no implementation)

Validation result:

```text
command: mise run reference-triage -- test262 reference/test262/test/annexB/built-ins/Date/prototype/getYear/B.2.4.js
result: Confirmed UnresolvedName for verifyProperty due to missing includes: processing
date: 2026-04-30

command: mise run reference-triage -- test262 reference/test262/test/annexB/built-ins/Object/is/emulates-undefined.js
result: Confirmed UnsupportedTest262Metadata for IsHTMLDDA feature
date: 2026-04-30
```

Remaining risks:

- None (triage complete, child issues 336 and 337 track implementation)
