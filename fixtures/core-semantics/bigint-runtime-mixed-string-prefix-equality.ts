let value = { x: 16n };
let hex = "0x10";
hex = hex + "";
let binary = "0b10000";
binary = binary + "";
let octal = "0o20";
octal = octal + "";
let negativeHex = "-0x10";
negativeHex = negativeHex + "";

console.log(value.x == hex);
console.log(hex == value.x);
console.log(value.x == binary);
console.log(binary == value.x);
console.log(value.x == octal);
console.log(octal == value.x);
console.log(value.x == negativeHex);
console.log(negativeHex != value.x);
