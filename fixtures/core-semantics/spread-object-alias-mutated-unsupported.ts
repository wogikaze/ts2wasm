let base = { a: 1 };
let values = base;
base.a = 2;
let copy = { b: 3, ...values };
console.log(copy.a);
