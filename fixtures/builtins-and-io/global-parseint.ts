// Global parseInt - radix, NaN sentinel, and UTF-8 whitespace coverage

let x = parseInt("42");
console.log(x);
let y = parseInt("0xFF");
console.log(y);
let z = parseInt("  101");
console.log(z);
let w = parseInt("  -99");
console.log(w);
let r = parseInt("z$", 36);
console.log(r);
let b = parseInt("10", 2);
console.log(b);
let u = parseInt("\u20001");
console.log(u);
let n = parseInt("\u2000");
console.log(n !== n);
