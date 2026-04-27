# Add spans to receiver this diagnostics

**Status**: open
**Created**: 2026-04-28
**Updated**: 2026-04-28
**ID**: 223
**Type**: bug
**Area**: frontend/diagnostics
**Priority**: P1
**Depends on**: 211
**Orchestration class**: implementation-ready

Problem: Issue 211 added issue-linked diagnostics for unsupported receiver/`this` forms, but those
diagnostics currently originate in IR/lowering paths that only emit `span: None`. Gatekeeper policy
requires source-origin diagnostics to preserve spans.

Scope:

- Carry enough source location information through resolved/lowered receiver expressions to report spans for issue-211 unsupported forms.
- Cover top-level `this`, extracted method calls, non-identifier receivers, and unknown receiver-class diagnostics where supported by existing frontend spans.
- Add regression tests that assert diagnostics include a source span.

Out of scope:

- Implementing additional receiver semantics.
- Arrow lexical `this`, tracked by issue 210.

Acceptance Criteria:

- [ ] Issue-211 unsupported receiver/`this` diagnostics include a source span.
- [ ] Regression tests fail if those diagnostics regress to `span: None`.
- [ ] Existing issue-211 semantic fixtures continue to pass or fail with the same issue-linked diagnostic messages.

Validation:

```sh
cargo fmt --all --check
cargo nextest run
```
