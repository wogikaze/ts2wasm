// Array.prototype.reverse tests
let a: any = [1, 2, 3];
let r1: any = a.reverse();
console.log(r1[0]);
console.log(r1[1]);
console.log(r1[2]);

let b: any = [1];
let r2: any = b.reverse();
console.log(r2[0]);

let c: any = [];
let r3: any = c.reverse();
console.log(r3.length);
