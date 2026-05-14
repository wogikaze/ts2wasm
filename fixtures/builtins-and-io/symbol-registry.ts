const first = Symbol.for("shared");
const second = Symbol.for("shared");

console.log(first === second);
console.log(Symbol.keyFor(first));
console.log(first.description);

// Symbol.for / Symbol.keyFor global registry and Symbol.prototype.description
let s1: any = Symbol.for("foo");
let s2: any = Symbol.for("foo");
// Symbol.for should return same symbol for same key
console.log(s1 === s2);

let k: any = Symbol.keyFor(s1);
console.log(k);
