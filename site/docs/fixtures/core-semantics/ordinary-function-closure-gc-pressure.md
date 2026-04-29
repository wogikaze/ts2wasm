# ordinary-function-closure-gc-pressure

```typescript
function makeReader() {
  let holder = { keep: "closure-object-alive" };

  function read() {
    return holder.keep;
  }

  return read;
}

let reader = makeReader();
let i = 0;
let s = "";

while (i < 2600) {
  s = "closure-pressure-" + i;
  i = i + 1;
}

console.log(reader());

```

**Path:** `fixtures/core-semantics/ordinary-function-closure-gc-pressure.ts`
