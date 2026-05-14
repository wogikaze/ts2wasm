const parseIntDesc = Object.getOwnPropertyDescriptor(Number, "parseInt");
const parseFloatDesc = Object.getOwnPropertyDescriptor(Number, "parseFloat");

console.log(Number.parseInt === parseInt);
console.log(Number.parseFloat === parseFloat);
console.log(typeof Number.parseInt);
console.log(typeof Number.parseFloat);
console.log(Number.parseInt.name);
console.log(Number.parseInt.length);
console.log(Number.parseFloat.name);
console.log(Number.parseFloat.length);
console.log(parseIntDesc.value === parseInt);
console.log(parseIntDesc.writable);
console.log(parseIntDesc.enumerable);
console.log(parseIntDesc.configurable);
console.log(parseFloatDesc.value === parseFloat);
console.log(parseFloatDesc.writable);
console.log(parseFloatDesc.enumerable);
console.log(parseFloatDesc.configurable);
