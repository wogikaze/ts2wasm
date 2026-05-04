// Array.prototype.toSorted tests — non-mutating numeric sort
// Test 1: basic sort
let a = [3, 1, 2];
let r1 = a.toSorted();
console.log(r1[0]);
console.log(r1[1]);
console.log(r1[2]);

// Test 2: already sorted
let b = [1, 2, 3];
let r2 = b.toSorted();
console.log(r2[0]);
console.log(r2[1]);
console.log(r2[2]);

// Test 3: reverse sorted
let c = [3, 2, 1];
let r3 = c.toSorted();
console.log(r3[0]);
console.log(r3[1]);
console.log(r3[2]);

// Test 4: original unchanged
let d = [3, 1, 2];
let r4 = d.toSorted();
console.log(d[0]);
console.log(d[1]);
console.log(d[2]);
console.log(r4[0]);
console.log(r4[1]);
console.log(r4[2]);

// Test 5: single element
let e = [42];
let r5 = e.toSorted();
console.log(r5[0]);

// Test 6: empty array
let f: number[] = [];
let r6 = f.toSorted();
console.log(r6.length);
