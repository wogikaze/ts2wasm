let decimal = "10";
decimal = decimal + "";
console.log(BigInt(decimal));

let spaced = "  -12  ";
spaced = "" + spaced;
console.log(BigInt(spaced));

let binary = "0b101";
binary = binary + "";
console.log(BigInt(binary));

let octal = "0o17";
octal = "" + octal;
console.log(BigInt(octal));

let hex = "0Xff";
hex = hex + "";
console.log(BigInt(hex));

let empty = "";
empty = empty + "";
console.log(BigInt(empty));
