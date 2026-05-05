---
id: 5009
title: "Remaining static ES module export forms (named list, default import, namespace, re-export, side-effect) (audit reopened #5009)"
type: feature
area: ir/compiler
class: done
priority: P1
depends_on: []
blocks: [5010]
status: open
created: 2026-05-02
updated: 2026-05-05
---

## Summary

Issue 5008 implemented `export const x = 1` (ExportDecl) and `export default <expr>` for entry modules without imports. The remaining static ES module export/import forms still hit `issue-055` unsupported diagnostics.

## Remaining forms

- `export { x, y }` / `export { x as y }` (named export lists in entry module)
- `import x from "./mod"` (default import from another module)
- `import x, { y } from "./mod"` (combined default + named import)
- `import * as ns from "./mod"` (namespace import)
- `import "./mod"` (side-effect import)
- `export * from "./mod"` (star re-export)
- `export { x } from "./mod"` (named re-export from)
- `export * as ns from "./mod"` (namespace re-export)

## Scope

- [ ] Rewrite `ExportNamed` in `lower_static_named_import_bindings_for_build` for `export { x, y }`
- [ ] Rewrite `ImportDefault` for `import x from "./mod"`
- [ ] Rewrite `ImportNamespace` for `import * as ns from "./mod"`
- [ ] Rewrite `ImportSideEffect` for `import "./mod"`
- Remaining forms (ImportDefaultNamed, re-exports, differential tests, issue-055 narrowing) tracked in issue 5010

## Acceptance criteria

- [ ] `export { x, y }` builds to WASM and both names are accessible
- [ ] `import x from "./mod"` builds to WASM and reads the default export
- [ ] `import * as ns from "./mod"` builds to WASM and `ns.x` accesses named exports
- [ ] `import "./side-effect"` triggers module initialization
- Remaining acceptance criteria tracked in issue 5010

## Validation

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-cli module
cargo nextest run -p ts2wasm-compiler
```

## Reopened by audit

Date: 2026-05-05

Classification: acceptance-not-actually-met.

Reopen reason: no `## Completion evidence` section is present, so close evidence cannot be cited from the issue file.

Violated acceptance: the issue cannot provide repo-local close evidence for its checked acceptance criteria while it remains in this state. Acceptance checkboxes were reset for re-verification.

Evidence files:
- `issues/open/5009-remaining-es-module-export-forms.md` before this move
- `issues/open/5009-remaining-es-module-export-forms.md` after this move

Split follow-up: none created in this audit wave; this reopened issue remains the tracking item.
