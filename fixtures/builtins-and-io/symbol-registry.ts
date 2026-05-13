const first = Symbol.for("shared");
const second = Symbol.for("shared");

console.log(first === second);
console.log(Symbol.keyFor(first));
console.log(first.description);
