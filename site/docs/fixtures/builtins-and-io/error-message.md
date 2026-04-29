# error-message

```typescript
let generic = new Error("generic message");
let type_error = new TypeError("type message");
let reference = new ReferenceError("reference message");
let syntax = new SyntaxError("syntax message");
let empty = new Error();
let number_message = new Error(42);
let bool_message = new TypeError(false);
let null_message = new ReferenceError(null);
let explicit_undefined = new SyntaxError(undefined);

console.log(generic.message);
console.log(type_error.message);
console.log(reference.message);
console.log(syntax.message);
console.log(empty.message);
console.log(number_message.message);
console.log(bool_message.message);
console.log(null_message.message);
console.log(explicit_undefined.message);

```

**Path:** `fixtures/builtins-and-io/error-message.ts`
