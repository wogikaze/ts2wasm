// Array.prototype.toSpliced tests
// Test 1: basic delete in middle
let a = [1, 2, 3, 4, 5];
let r1 = a.toSpliced(1, 2);
console.log(r1[0]);
console.log(r1[1]);
console.log(r1[2]);
console.log(r1.length);

// Test 2: delete from start
let b = [1, 2, 3];
let r2 = b.toSpliced(0, 1);
console.log(r2[0]);
console.log(r2[1]);
console.log(r2.length);

// Test 3: delete from end
let c = [1, 2, 3];
let r3 = c.toSpliced(2, 1);
console.log(r3[0]);
console.log(r3[1]);
console.log(r3.length);

// Test 4: delete all
let d = [1, 2, 3];
let r4 = d.toSpliced(0, 3);
console.log(r4.length);

// Test 5: empty array
let e: any = [];
let r5 = e.toSpliced(0, 0);
console.log(r5.length);

// Test 6: original unchanged
let f = [1, 2, 3];
let r6 = f.toSpliced(1, 1);
console.log(f[0]);
console.log(f[1]);
console.log(f[2]);
console.log(r6[0]);
console.log(r6[1]);
console.log(r6.length);
