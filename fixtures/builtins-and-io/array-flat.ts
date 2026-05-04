// Array.prototype.flat tests
// Test 1: basic flatten (default depth = 1)
let a = [1, [2, 3]];
let r1 = a.flat();
console.log(r1.length);
console.log(r1[0]);
console.log(r1[1]);

// Test 2: depth = 2
let b = [1, [2, [3]]];
let r2 = b.flat(2);
console.log(r2.length);
console.log(r2[2]);

// Test 3: depth = 0 (no flattening)
let c = [1, [2, 3]];
let r3 = c.flat(0);
console.log(r3.length);

// Test 4: depth > actual nesting
let d = [1, [2, 3]];
let r4 = d.flat(5);
console.log(r4.length);

// Test 5: empty array
let e = [];
let r5 = e.flat();
console.log(r5.length);

// Test 6: already flat
let f = [1, 2, 3];
let r6 = f.flat();
console.log(r6.length);

// Test 7: mixed nesting
let g = [[1], [[2]], [[[3]]]];
let r7 = g.flat(2);
console.log(r7.length);
console.log(r7[0]);
console.log(r7[1]);

// Test 8: non-array + array mixed
let h = [1, "hello", [2]];
let r8 = h.flat();
console.log(r8.length);
console.log(r8[1]);
