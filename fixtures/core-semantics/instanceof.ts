// Test instanceof operator
// Note: Full instanceof semantics require prototype chain support
// This test verifies the operator parses correctly
let x = 5;
let y = 10;
let result = x instanceof y; // will be false (primitives)
console.log(result);
console.log(0); // exit marker
