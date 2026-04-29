# map-set

```typescript
let m = new Map();
console.log(m.has("a"));
console.log(m.get("a"));
console.log(m.set("a", 1) === m);
console.log(m.get("a"));
console.log(m.has("a"));
console.log(m.set("a", 2) === m);
console.log(m.get("a"));
console.log(m.delete("a"));
console.log(m.has("a"));
console.log(m.get("a"));
console.log(m.delete("a"));

let s = new Set();
console.log(s.has("x"));
console.log(s.add("x") === s);
console.log(s.has("x"));
console.log(s.has("missing"));
console.log(s.delete("x"));
console.log(s.has("x"));
console.log(s.delete("x"));

```

**Path:** `fixtures/builtins-and-io/map-set.ts`
