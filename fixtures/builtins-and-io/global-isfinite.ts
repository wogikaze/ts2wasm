// Global isFinite - ECMAScript ToNumber semantics within the tagged number model

let x = isFinite(42);
console.log(x);
let y = isFinite(Infinity);
console.log(y);
let z = isFinite(NaN);
console.log(z);
let neg = isFinite(-Infinity);
console.log(neg);
let w = isFinite("42");
console.log(w);
let u = isFinite(undefined);
console.log(u);
