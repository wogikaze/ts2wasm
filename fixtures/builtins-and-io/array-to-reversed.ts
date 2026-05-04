// Array.prototype.toReversed tests
// Test 1: basic reverse
let a = [1, 2, 3];
let r1 = a.toReversed();
console.log(r1[0]);
console.log(r1[1]);
console.log(r1[2]);
console.log(r1.length);

// Test 2: single element
let b = [42];
let r2 = b.toReversed();
console.log(r2[0]);

// Test 3: empty array
let c: any = [];
let r3 = c.toReversed();
console.log(r3.length);

// Test 4: even length
let d = [1, 2, 3, 4];
let r4 = d.toReversed();
console.log(r4[0]);
console.log(r4[1]);
console.log(r4[2]);
console.log(r4[3]);

// Test 5: original unchanged
let e = [1, 2, 3];
let r5 = e.toReversed();
console.log(e[0]);
console.log(r5[0]);
