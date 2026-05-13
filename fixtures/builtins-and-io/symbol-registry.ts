// Symbol.for / Symbol.keyFor global registry and Symbol.prototype.description
let s1: any = Symbol.for("foo");
let s2: any = Symbol.for("foo");
// Symbol.for should return same symbol for same key
console.log(s1 === s2);

let k: any = Symbol.keyFor(s1);
console.log(k);
