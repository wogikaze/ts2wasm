// Number.prototype.toString() tests
// Test 1: basic number toString
let a: any = 42;
console.log(a.toString());

// Test 2: negative number
let b: any = -7;
console.log(b.toString());

// Test 3: zero
let c: any = 0;
console.log(c.toString());

// Test 4: large number
let d: any = 1000;
console.log(d.toString());

// Test 5: Number(42).toString() via Number wrapper
let e: any = 123;
console.log(e.toString());

// Test 6: toString with radix 16
let f: any = 255;
console.log(f.toString(16));

// Test 7: toString with radix 2
let g: any = 42;
console.log(g.toString(2));

// Test 8: toString with radix 8
let h: any = 255;
console.log(h.toString(8));

// Test 9: toString with radix 36
let i: any = 123456;
console.log(i.toString(36));

// Test 10: toString with radix 10 (explicit)
let j: any = 42;
console.log(j.toString(10));

// Test 11: negative number with radix
let k: any = -255;
console.log(k.toString(16));
