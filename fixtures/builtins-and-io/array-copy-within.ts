// Array.prototype.copyWithin tests
// Test 1: basic copy forward (target < start)
let a: any = [1, 2, 3, 4, 5];
let r1: any = a.copyWithin(0, 3);
console.log(r1.length);
console.log(r1[0]);
console.log(r1[1]);
console.log(r1[2]);
console.log(r1[3]);

// Test 2: copy backward (target > start, overlapping)
let b: any = [1, 2, 3, 4, 5];
let r2: any = b.copyWithin(3, 0, 2);
console.log(r2[3]);
console.log(r2[4]);

// Test 3: negative target
let c: any = [1, 2, 3, 4, 5];
let r3: any = c.copyWithin(-2, 0);
console.log(r3[3]);
console.log(r3[4]);

// Test 4: negative start
let d: any = [1, 2, 3, 4, 5];
let r4: any = d.copyWithin(0, -3);
console.log(r4[0]);
console.log(r4[1]);

// Test 5: negative end
let e: any = [1, 2, 3, 4, 5];
let r5: any = e.copyWithin(0, 1, -1);
console.log(r5[0]);
console.log(r5[1]);
console.log(r5[2]);
console.log(r5[3]);

// Test 6: returns the array itself
let f: any = [1, 2, 3];
let r6: any = f.copyWithin(0, 0);
if (r6 === f) { console.log(1); } else { console.log(0); }

// Test 7: empty array
let g: any = [];
let r7: any = g.copyWithin(0, 1);
console.log(r7.length);
