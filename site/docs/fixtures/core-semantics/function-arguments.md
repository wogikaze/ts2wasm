# function-arguments

```typescript
function count() {
    console.log(arguments.length);
}

function first() {
    console.log(arguments.length);
    console.log(arguments[0]);
}

function many(head) {
    console.log(head);
    console.log(arguments.length);
    console.log(arguments[0]);
    console.log(arguments[1]);
    console.log(arguments[2]);
}

count();
first(7);
many(4, 5, 6);

```

**Path:** `fixtures/core-semantics/function-arguments.ts`
