// Basic integer arithmetic operations
console.log(1 + 2);
console.log(10 - 4);
console.log(3 * 5);
console.log(20 / 4);
console.log(17 % 5);

// Integer division truncation toward zero
// Note: ts2wasm uses integer division, so 7/2 gives 3 not 3.5
console.log(8 / 4);
console.log(9 / 3);
console.log(12 / 4);

// Modulo with negative operands (sign follows dividend)
console.log(17 % 5);
console.log(-17 % 5);
console.log(17 % -5);
console.log(-17 % -5);

// Operator precedence
console.log(1 + 2 * 3);
console.log((1 + 2) * 3);
console.log(10 - 4 - 2);
console.log(10 - (4 - 2));

// Compound assignment
let a = 5;
a += 3;
console.log(a);

let b = 10;
b -= 4;
console.log(b);

let c = 3;
c *= 4;
console.log(c);

let d = 20;
d /= 5;
console.log(d);

let e = 17;
e %= 5;
console.log(e);
