// Array.prototype.flat tests
// Test 1: basic flatten (default depth = 1)
let a = [1, [2, 3]];
let r1 = a.flat();
console.log(r1.length === 2 ? 1 : 0);
console.log(r1[0] === 1 ? 1 : 0);
console.log(r1[1] === 2 ? 1 : 0);

// Test 2: depth = 2
let b = [1, [2, [3]]];
let r2 = b.flat(2);
console.log(r2.length === 3 ? 1 : 0);
console.log(r2[2] === 3 ? 1 : 0);

// Test 3: depth = 0 (no flattening)
let c = [1, [2, 3]];
let r3 = c.flat(0);
console.log(r3.length === 2 ? 1 : 0);
console.log(r3[1] === c[1] ? 1 : 0);

// Test 4: depth > actual nesting
let d = [1, [2, 3]];
let r4 = d.flat(5);
console.log(r4.length === 3 ? 1 : 0);

// Test 5: empty array
let e: number[] = [];
let r5 = e.flat();
console.log(r5.length === 0 ? 1 : 0);

// Test 6: already flat
let f = [1, 2, 3];
let r6 = f.flat();
console.log(r6.length === 3 ? 1 : 0);

// Test 7: mixed nesting depth 2
let g = [[1], [[2]], [[[3]]]];
let r7 = g.flat(2);
console.log(r7.length === 3 ? 1 : 0);
console.log(r7[0] === 1 ? 1 : 0);
console.log(r7[1] === 2 ? 1 : 0);
console.log(JSON.stringify(r7[2]) === "[[3]]" ? 1 : 0);

// Test 8: non-array + array mixed
let h = [1, "hello", [2]];
let r8 = h.flat();
console.log(r8.length === 3 ? 1 : 0);
console.log(r8[1] === "hello" ? 1 : 0);
console.log(r8[2] === 2 ? 1 : 0);
