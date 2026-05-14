// TypedArray indexOf, includes, lastIndexOf, findIndex methods
const ta = new Uint8Array([10, 20, 30, 20, 10]);

console.log(ta.indexOf(20));
console.log(ta.indexOf(20, 2));
console.log(ta.indexOf(99));

console.log(ta.includes(20));
console.log(ta.includes(99));

console.log(ta.lastIndexOf(20));
console.log(ta.lastIndexOf(20, 1));

console.log(ta.findIndex(x => x > 15));
console.log(ta.findIndex(x => x > 100));

const empty = new Uint8Array(0);
console.log(empty.indexOf(1));
console.log(empty.includes(1));
console.log(empty.findIndex(x => true));
