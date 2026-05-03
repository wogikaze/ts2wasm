# Plan: Implement module-resolution support

## Scope
Implement basic module resolution: bare specifier resolution,
node_modules traversal, package.json fields, .d.ts/.tsx support.

## Implementation (completed)
- `e42d9571` - basic bare specifier and node_modules resolution
- `d328e0da` - .d.ts extension support
- `1e5ba813` - parent directory node_modules traversal
- `d1a199c0` - package.json types/main field resolution
- `3a74e378` - .tsx extension support
- `e53f9bf7` - package.json exports field resolution

## Verification
- module_graph tests: 8/8 passed
- m9_modules integration: 31/32 passed

## Risk
- Pre-existing test failure (`build_smoke_module_exports_assign`) unrelated
- Complete module resolution requires more work (tsconfig paths, @types, symlinks)
