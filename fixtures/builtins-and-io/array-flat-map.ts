// Array.prototype.flatMap tests
let a: any = [1, 2, 3];
let r1: any = a.flatMap((x: any) => [x, x * 2]);
console.log(r1.length);
console.log(r1[0]);
console.log(r1[1]);
console.log(r1[2]);
console.log(r1[3]);
console.log(r1[4]);

let r2: any = a.flatMap((x: any) => x + 1);
console.log(r2[0]);
console.log(r2[1]);
console.log(r2[2]);

let b: any = [];
let r3: any = b.flatMap((x: any) => [x]);
console.log(r3.length);
