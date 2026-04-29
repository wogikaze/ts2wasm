let bits = 8;
bits = bits + 0;

let value = -1n;
value = value + 0n;
console.log(BigInt.asUintN(bits, value));

let converted = BigInt(-1 + 0);
console.log(BigInt.asUintN(bits, converted));

let wideBits = 64;
wideBits = wideBits + 0;
console.log(BigInt.asUintN(wideBits, converted));
