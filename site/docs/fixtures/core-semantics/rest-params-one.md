# rest-params-one

```typescript
function collect(head, ...rest) {
    console.log(head);
    console.log(rest.length);
    console.log(rest[0]);
}

collect(7, 8);

```

**Path:** `fixtures/core-semantics/rest-params-one.ts`
