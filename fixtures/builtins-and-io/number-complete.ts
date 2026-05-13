// Number static and prototype methods test
let n: any = 123.456;

// Number.isNaN — use a non-number expression
let nan_val: any = "abc" / 1;
console.log(Number.isNaN(nan_val));
console.log(Number.isNaN(123));

// Number.isFinite
console.log(Number.isFinite(123));
console.log(Number.isFinite(1 / 0));

// Number.isInteger
console.log(Number.isInteger(42));
console.log(Number.isInteger(42.5));

// Number.isSafeInteger
console.log(Number.isSafeInteger(42));

// parseInt/parseFloat aliases
console.log(Number.parseInt("42"));
console.log(Number.parseFloat("3.14"));

// toFixed
console.log(n.toFixed(2));
console.log(n.toFixed(0));

// toExponential
console.log(n.toExponential(2));

// toPrecision
let n2: any = 123.456;
console.log(n2.toPrecision(4));
console.log(n2.toPrecision(2));
