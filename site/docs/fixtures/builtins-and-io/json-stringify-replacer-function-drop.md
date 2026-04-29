# json-stringify-replacer-function-drop

```typescript
function dropB(key, value) {
  if (key === "b") {
    return undefined;
  }
  return value;
}

console.log(JSON.stringify({ a: 1, b: 2 }, dropB));

```

**Path:** `fixtures/builtins-and-io/json-stringify-replacer-function-drop.ts`
