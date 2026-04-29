# error-instanceof

```typescript
// @ts-nocheck
let generic = new Error("generic");
let type_error = new TypeError("type");
let reference = new ReferenceError("reference");
let syntax = new SyntaxError("syntax");
let plain = {};

console.log(generic instanceof Error);
console.log(type_error instanceof TypeError);
console.log(type_error instanceof Error);
console.log(reference instanceof ReferenceError);
console.log(reference instanceof Error);
console.log(syntax instanceof SyntaxError);
console.log(syntax instanceof Error);
console.log(generic instanceof TypeError);
console.log(type_error instanceof ReferenceError);
console.log(plain instanceof Error);
console.log(1 instanceof Error);

```

**Path:** `fixtures/builtins-and-io/error-instanceof.ts`
