// Global isFinite - ECMAScript semantics within integer f64 model
// Known limitation: NaN/Infinity not representable as tagged numbers
// Infinity and NaN variables resolve to the number 0 tag, so isFinite(Infinity) 
// returns true because the runtime sees a finite number (issue-281).

let x = isFinite(42);
console.log(x);
// Node: Infinity → false (our model: 0 is finite → true)
let y = isFinite(0);
console.log(y);
// Node: NaN → false (our model: 0 is finite → true)
let z = isFinite(1);
console.log(z);
let w = isFinite("42");
console.log(w);
