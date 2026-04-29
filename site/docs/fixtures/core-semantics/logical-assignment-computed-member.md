# logical-assignment-computed-member

```typescript
function rhs(value) {
  console.log("rhs");
  return value;
}

function key() {
  console.log("key");
  return "value";
}

let orSkip = { value: "kept" };
function getOrSkip(target) {
  console.log("receiver");
  return target;
}
console.log(getOrSkip(orSkip)[key()] ||= rhs("bad"));
console.log(orSkip.value);

let orRun = { value: false };
function getOrRun(target) {
  console.log("receiver");
  return target;
}
console.log(getOrRun(orRun)[key()] ||= rhs("filled-computed"));
console.log(orRun.value);

let andSkip = { value: false };
function getAndSkip(target) {
  console.log("receiver");
  return target;
}
console.log(getAndSkip(andSkip)[key()] &&= rhs("bad"));
console.log(andSkip.value);

let andRun = { value: true };
function getAndRun(target) {
  console.log("receiver");
  return target;
}
console.log(getAndRun(andRun)[key()] &&= rhs("updated-computed"));
console.log(andRun.value);

let nullishSkip = { value: "kept-nullish" };
function getNullishSkip(target) {
  console.log("receiver");
  return target;
}
console.log(getNullishSkip(nullishSkip)[key()] ??= rhs("bad"));
console.log(nullishSkip.value);

let nullishRun = { value: null };
function getNullishRun(target) {
  console.log("receiver");
  return target;
}
console.log(getNullishRun(nullishRun)[key()] ??= rhs("fallback-computed"));
console.log(nullishRun.value);

```

**Path:** `fixtures/core-semantics/logical-assignment-computed-member.ts`
