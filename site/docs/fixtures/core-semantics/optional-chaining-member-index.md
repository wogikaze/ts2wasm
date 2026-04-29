# optional-chaining-member-index

```typescript
let missing = null;
let present = { x: 7 };

function key() {
  console.log("key");
  return "x";
}

console.log(missing?.x);
console.log(present?.x);
console.log(missing?.[key()]);
console.log(present?.[key()]);

```

**Path:** `fixtures/core-semantics/optional-chaining-member-index.ts`
