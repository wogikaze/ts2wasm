// Array.prototype.reduce should produce a clear unsupported diagnostic
const arr = [1, 2, 3];
const sum = arr.reduce((a, b) => a + b, 0);
console.log(sum);
