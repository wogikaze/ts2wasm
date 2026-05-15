// GC heap-kind survival: verify that values of different GC heap kinds
// survive after forced allocation pressure. Each heap-kind tracks its own
// GC_KIND_* tag in the object descriptor's metadata bits; the mark loop
// must preserve reachable objects regardless of their heap kind.

// --- string survival ---
let strVal: string = "surviving-string-value";
// Allocation pressure
let i: number = 0;
let junk: string = "";
while (i < 5000) {
  junk = "pressure-" + i;
  i = i + 1;
}
console.log(strVal);

// --- array survival ---
let arrVal: number[] = [10, 20, 30, 40, 50];
i = 0;
while (i < 5000) {
  junk = "junk-" + i;
  i = i + 1;
}
console.log(arrVal[2]);

// --- ordinary object survival ---
let objVal: object = { a: 100, b: 200 };
i = 0;
while (i < 5000) {
  junk = "junk-" + i;
  i = i + 1;
}
console.log((objVal as any)["a"]);

// --- BigInt survival ---
let bigVal: bigint = 9007199254740993n;
i = 0;
while (i < 5000) {
  junk = "junk-" + i;
  i = i + 1;
}
console.log(bigVal.toString());

// --- Immutable returned closure with captured heap object ---
function makeClosure(): () => string {
  let captured: string = "closure-captured-value";
  return (): string => {
    return captured;
  };
}
let fn: () => string = makeClosure();
i = 0;
while (i < 5000) {
  junk = "junk-" + i;
  i = i + 1;
}
console.log(fn());
