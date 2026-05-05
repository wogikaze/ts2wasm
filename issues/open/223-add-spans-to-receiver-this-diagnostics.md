# Add spans to receiver this diagnostics (audit reopened #223)

**Status**: open
**Created**: 2026-04-28
**Updated**: 2026-04-28
**Completed**: 2026-04-28
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

Completion evidence:

- Implementation commit: `e8ebbca` (`issue-223: add spans to receiver diagnostics`)
- `cargo fmt --all --check`: pass
- `cargo nextest run -E 'test(this_receiver_method_unsupported_forms_report_issue_211) | test(labeled_control_invalid_fixtures_report_source_diagnostics)'`: pass (2 passed)
- Manual CLI smoke confirmed issue-211 diagnostics include byte spans:
  - `this-top-level-unsupported.ts`: `at 12..16`
  - `this-extracted-method-unsupported.ts`: `at 161..167`
  - `this-non-identifier-receiver-unsupported.ts`: `at 12..30`
  - `this-unknown-receiver-class-unsupported.ts`: `at 42..57`
- `cargo nextest run`: pass (239 passed, 4 skipped)
- `mise run check-issue-health`: pass
- `mise run update-issue-index -- --check`: pass
- `mise run check-agent-state`: pass
- `mise run check-repo-smoke`: pass

## Reopened by audit

Date: 2026-05-05

Classification: acceptance-not-actually-met.

Reopen reason: no `## Completion evidence` section is present, so close evidence cannot be cited from the issue file.

Violated acceptance: the issue cannot provide repo-local close evidence for its checked acceptance criteria while it remains in this state. Acceptance checkboxes were reset for re-verification.

Evidence files:
- `issues/open/223-add-spans-to-receiver-this-diagnostics.md` before this move
- `issues/open/223-add-spans-to-receiver-this-diagnostics.md` after this move

Split follow-up: none created in this audit wave; this reopened issue remains the tracking item.
