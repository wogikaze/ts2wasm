let bits = 8;
bits = bits + 0;

let value = 255n;
value = value + 0n;
console.log(BigInt.asIntN(bits, value));

let negative = -129n;
negative = negative + 0n;
console.log(BigInt.asIntN(bits, negative));

let converted = BigInt(255 + 0);
console.log(BigInt.asIntN(bits, converted));
