// Array.prototype.values tests
let a = [1, 2, 3];
let v = a.values();
console.log(v[0]);
console.log(v[1]);
console.log(v[2]);
console.log(v.length);

// Single element
let b = [42];
let v2 = b.values();
console.log(v2[0]);

// Empty array
let c: any = [];
let v3 = c.values();
console.log(v3.length);

// Original unchanged
let d = [1, 2, 3];
let v4 = d.values();
console.log(d[0]);
console.log(v4[0]);
