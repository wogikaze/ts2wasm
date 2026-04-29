# json-stringify-replacer-array-boxed

```typescript
console.log(JSON.stringify({ "1": "one", a: 2 }, [new Number(1), new String("a")]));
console.log(JSON.stringify({ "": "empty", "0": "zero", "-1": "minus" }, [new String(), new Number(), new Number(-1)]));
console.log(JSON.stringify({ a: 1, b: 2 }, [new String("a"), new String("a"), "b"]));

```

**Path:** `fixtures/builtins-and-io/json-stringify-replacer-array-boxed.ts`
