# AnyAsReturnType Pattern Detection Implementation

## Problem Statement
AnyAsReturnType is a type feature where the `typeof()` operator yields "any" for variables annotated with type "any" or variables that could hold any type. This results in `super()` calls being passed `undefined` call sites rather than the actual class, breaking call resolution.

## Implementation Plan

### 1. Static Analysis Phase
Add AnyAsReturnType feature to the semantic analysis phase that tracks:
- Variables with type "any" annotation
- Variables that could hold any type during type inference
- How these affect method resolution patterns like super()/superCall()/superCallExternal()

### 2. Method Resolution Middleware
- Modify method resolution to handle AnyAsReturnType for super references
- When object type is AnyAsReturnType, defer to runtime dispatch rather than immediate resolution
- Route through `HeapClosureCall` patterns that resolve based on runtime value characteristics

### 3. Test Coverage
- Fix node_diff (issue 5053-5054) for Array.toString() on any-typed arrays
- Fix Array.toString() for any-typed arrays (item 17 in parser cleanup)
- Verify ArrayJoin callback routing works correctly

## Expected Outcomes
- Any-typed variable calls to Array methods correctly route through runtime cast
- Array prototype methods work correctly for any-typed arrays
- Build-smoke, node_diff, workspace tests maintain 0 failures
