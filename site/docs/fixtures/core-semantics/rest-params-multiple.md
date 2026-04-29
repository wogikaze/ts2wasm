# rest-params-multiple

```typescript
function collect(head, ...rest) {
    console.log(head);
    console.log(rest.length);
    console.log(rest[0]);
    console.log(rest[1]);
    console.log(rest[2]);
}

collect(7, 8, 9, 10);

```

**Path:** `fixtures/core-semantics/rest-params-multiple.ts`
