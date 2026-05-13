// Array.prototype.entries tests
let a = [10, 20];
let e = a.entries();
let r1 = e.next();
let p1 = r1.value;
console.log(p1[0]);
console.log(p1[1]);
console.log(r1.done);
let r2 = e.next();
let p2 = r2.value;
console.log(p2[0]);
console.log(p2[1]);
console.log(r2.done);
let r3 = e.next();
console.log(r3.value);
console.log(r3.done);

// Single element
let b = [42];
let e2 = b.entries();
let r4 = e2.next();
let p4 = r4.value;
console.log(p4[0]);
console.log(p4[1]);
console.log(r4.done);

// Empty array
let c: any = [];
let e3 = c.entries();
let r5 = e3.next();
console.log(r5.value);
console.log(r5.done);
