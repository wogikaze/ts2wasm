# ordinary-function-closure-make-adder

```typescript
function makeAdder(x) {
  function add(y) {
    return x + y;
  }

  return add;
}

let add4 = makeAdder(4);
console.log(add4(5));

```

**Path:** `fixtures/core-semantics/ordinary-function-closure-make-adder.ts`
