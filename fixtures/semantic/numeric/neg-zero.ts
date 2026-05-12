// -0 equality
// Note: ts2wasm runtime does not distinguish -0 from 0 for display.
// The equality comparisons (===, ==) are correct in JS semantics:
// -0 === 0 and -0 == 0 are both true per spec.

console.log(-0 === 0);
console.log(-0 == 0);

// -0 in comparisons
console.log(-0 < 0);
console.log(-0 > 0);
console.log(-0 <= 0);
console.log(-0 >= 0);
