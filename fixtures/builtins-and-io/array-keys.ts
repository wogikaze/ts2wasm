// Array.prototype.keys tests
let a = [10, 20, 30];
let k = a.keys();
let r1 = k.next();
console.log(r1.value);
console.log(r1.done);
let r2 = k.next();
console.log(r2.value);
console.log(r2.done);
let r3 = k.next();
console.log(r3.value);
console.log(r3.done);
let r4 = k.next();
console.log(r4.value);
console.log(r4.done);

// Single element
let b = [42];
let k2 = b.keys();
let r5 = k2.next();
console.log(r5.value);
console.log(r5.done);

// Empty array
let c: any = [];
let k3 = c.keys();
let r6 = k3.next();
console.log(r6.value);
console.log(r6.done);

// Sparse-ish: 2 elements
let d = [1, 2];
let k4 = d.keys();
let r7 = k4.next();
console.log(r7.value);
let r8 = k4.next();
console.log(r8.value);
