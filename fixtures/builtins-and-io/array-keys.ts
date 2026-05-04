// Array.prototype.keys tests
let a = [10, 20, 30];
let k = a.keys();
console.log(k[0]);
console.log(k[1]);
console.log(k[2]);
console.log(k.length);

// Single element
let b = [42];
let k2 = b.keys();
console.log(k2[0]);

// Empty array
let c: any = [];
let k3 = c.keys();
console.log(k3.length);

// Sparse-ish: 2 elements
let d = [1, 2];
let k4 = d.keys();
console.log(k4[0]);
console.log(k4[1]);
