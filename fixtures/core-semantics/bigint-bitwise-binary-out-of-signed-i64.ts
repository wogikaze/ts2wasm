console.log(18446744073709551616n & 18446744073709551615n);
console.log(18446744073709551616n | 1n);
console.log(18446744073709551616n ^ 3n);
console.log(-18446744073709551617n & 255n);
console.log(-18446744073709551617n | 255n);
console.log(-18446744073709551617n ^ 255n);

let wideBinary = 18446744073709551616n;
let lowMask = 255n;
console.log(wideBinary & lowMask);
console.log(wideBinary | 1n);
console.log(wideBinary ^ 3n);
wideBinary = wideBinary ^ lowMask;
console.log(wideBinary);
