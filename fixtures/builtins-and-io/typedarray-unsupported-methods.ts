// TypedArray methods now supported — join, entries, keys, values, toReversed, toSorted, with
const a = new Uint8Array([1, 2, 3]);
console.log(a.join(","));
console.log(a.entries());
console.log(a.keys());
console.log(a.values());
console.log(a.toReversed());
console.log(a.toSorted());
console.log(a.with(0, 9));
