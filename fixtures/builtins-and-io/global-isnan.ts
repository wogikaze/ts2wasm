// Global isNaN - ECMAScript ToNumber semantics within the tagged number model

let x = isNaN(42);
console.log(x);
let y = isNaN(undefined);
console.log(y);
let z = isNaN("hello");
console.log(z);
let w = isNaN("42");
console.log(w);
let n = isNaN(NaN);
console.log(n);
let inf = isNaN(Infinity);
console.log(inf);
