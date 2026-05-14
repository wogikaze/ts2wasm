// TypedArray reverse, fill, at, findLast, findLastIndex, reduceRight
const a = new Uint8Array([1, 2, 3]);
a.reverse();
console.log(a[0]);
console.log(a[1]);
console.log(a[2]);

const b = new Uint8Array([1, 2, 3, 4]);
b.fill(9);
console.log(b[0]);
console.log(b[1]);
console.log(b[2]);
console.log(b[3]);

const c = new Uint8Array([1, 2, 3]);
b.fill(0, 1, 3);
console.log(b[0]);
console.log(b[1]);
console.log(b[2]);
console.log(b[3]);

console.log(c.at(0));
console.log(c.at(-1));
console.log(c.at(5));

const d = new Uint8Array([3, 1, 4, 1, 5]);
console.log(d.findLast(x => x > 2));
console.log(d.findLastIndex(x => x > 2));
console.log(d.findLast(x => x > 10));
console.log(d.findLastIndex(x => x > 10));

const e = new Uint8Array([1, 2, 3]);
console.log(e.reduceRight((acc, x) => acc + x, 0));
console.log(e.reduceRight((acc, x) => acc + String(x), ""));
