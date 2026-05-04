// Array.prototype.entries tests
let a = [10, 20];
let e = a.entries();
console.log(e.length);
console.log(e[0][0]);
console.log(e[0][1]);
console.log(e[1][0]);
console.log(e[1][1]);

// Single element
let b = [42];
let e2 = b.entries();
console.log(e2[0][0]);
console.log(e2[0][1]);

// Empty array
let c: any = [];
let e3 = c.entries();
console.log(e3.length);
