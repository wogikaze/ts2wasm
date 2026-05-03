# Plan: [backend-wasm] Implement real class declaration emission

## Summary
Implement ClassDecl WAT emission for constructor, prototype, static members, extends, and private elements.

## Steps
1. Audit current ClassDecl handling in expr_emit.rs (find todo!() and placeholder paths)
2. Implement constructor WAT emission (prototype setup + constructor body)
3. Implement prototype method wiring
4. Implement static member emission
5. Implement extends (single inheritance)
6. Add fixture tests
7. Verify with fmt + nextest
