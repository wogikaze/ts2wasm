# bigint-runtime-mul-div-rem

```typescript
let a = 6n;
let b = 4n;
console.log(a * b);
console.log(a / b);
console.log(a % b);

let neg = -a;
console.log(neg * b);
console.log(neg / b);
console.log(neg % b);
console.log(a / -b);
console.log(a % -b);

let z = 0n;
console.log(z * b);
console.log(z / b);
console.log(z % b);

```

**Path:** `fixtures/core-semantics/bigint-runtime-mul-div-rem.ts`
