// Array.prototype.values tests
let a = [1, 2, 3];
let v = a.values();
let r1 = v.next();
console.log(r1.value);
console.log(r1.done);
let r2 = v.next();
console.log(r2.value);
console.log(r2.done);
let r3 = v.next();
console.log(r3.value);
console.log(r3.done);
let r4 = v.next();
console.log(r4.value);
console.log(r4.done);

// Single element
let b = [42];
let v2 = b.values();
let r5 = v2.next();
console.log(r5.value);
console.log(r5.done);

// Empty array
let c: any = [];
let v3 = c.values();
let r6 = v3.next();
console.log(r6.value);
console.log(r6.done);

// Original unchanged
let d = [1, 2, 3];
let v4 = d.values();
console.log(d[0]);
let r7 = v4.next();
console.log(r7.value);
