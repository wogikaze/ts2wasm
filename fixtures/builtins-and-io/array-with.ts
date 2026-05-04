// Array.prototype.with tests
// Test 1: basic replace
let a = [1, 2, 3];
let r1 = a.with(1, 10);
console.log(r1[0]);
console.log(r1[1]);
console.log(r1[2]);
console.log(r1.length);

// Test 2: negative index
let b = [1, 2, 3];
let r2 = b.with(-1, 100);
console.log(r2[0]);
console.log(r2[1]);
console.log(r2[2]);

// Test 3: first element
let c = [1, 2, 3];
let r3 = c.with(0, 99);
console.log(r3[0]);

// Test 4: single element
let d = [42];
let r4 = d.with(0, 7);
console.log(r4[0]);
console.log(r4.length);

// Test 5: original unchanged
let e = [1, 2, 3];
let r5 = e.with(0, 99);
console.log(e[0]);
console.log(r5[0]);

// Test 6: empty check
let f: any = [];
let r6 = f.with(0, 1);
console.log(r6.length);
