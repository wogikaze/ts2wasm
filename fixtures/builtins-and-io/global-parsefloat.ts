// Global parseFloat on integer-valued strings (decimal truncation is a known issue-281 limitation)

let x = parseFloat("42");
console.log(x);
let y = parseFloat("  100");
console.log(y);
let z = parseFloat("  -7");
console.log(z);
