let fromString = BigInt("10");
let fromNumber = BigInt(12);
let fromNegativeNumber = BigInt(-3);
let fromTrue = BigInt(true);
let fromFalse = BigInt(false);
let fromBigInt = BigInt(13n);

console.log(fromString);
console.log(fromNumber);
console.log(fromNegativeNumber);
console.log(fromTrue);
console.log(fromFalse);
console.log(fromBigInt);
console.log(String(fromString));
console.log(String(-14n));
console.log("value=" + fromString);
console.log(`${fromString}:${fromFalse}`);
