const first: any = Symbol("slot");
const second: any = Symbol("slot");
const obj: any = {};

obj[first] = "first";
obj[second] = "second";

console.log(obj[first]);
console.log(obj[second]);
console.log(first === second ? "same" : "different");
