// Array.prototype.flat tests
// Test 1: basic flatten (default depth = 1)
let a: any = [1, [2, 3]];
let r1: any = a.flat();
console.log(r1.length);
console.log(r1[0]);
console.log(r1[1]);

// Test 2: depth = 2
let b: any = [1, [2, [3]]];
let r2: any = b.flat(2);
console.log(r2.length);
console.log(r2[2]);

// Test 3: depth = 0 (no flattening)
let c: any = [1, [2, 3]];
let r3: any = c.flat(0);
console.log(r3.length);

// Test 4: depth > actual nesting
let d: any = [1, [2, 3]];
let r4: any = d.flat(5);
console.log(r4.length);

// Test 5: empty array
let e: any = [];
let r5: any = e.flat();
console.log(r5.length);

// Test 6: already flat
let f: any = [1, 2, 3];
let r6: any = f.flat();
console.log(r6.length);

// Test 7: mixed nesting
let g: any = [[1], [[2]], [[[3]]]];
let r7: any = g.flat(2);
console.log(r7.length);
console.log(r7[0]);
console.log(r7[1]);

// Test 8: non-array + array mixed
let h: any = [1, "hello", [2]];
let r8: any = h.flat();
console.log(r8.length);
console.log(r8[1]);
