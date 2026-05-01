// Global isNaN - ECMAScript semantics within integer f64 model
// Known limitation: NaN/Infinity not representable as tagged numbers
// isNaN returns boolean, which is not affected

let x = isNaN(42);
console.log(x);
let y = isNaN(undefined);
console.log(y);
let z = isNaN("hello");
console.log(z);
let w = isNaN("42");
console.log(w);
