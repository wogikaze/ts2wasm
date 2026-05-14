// Symbol.for(key) global registration and Symbol.keyFor(sym) reverse lookup
console.log(Symbol.for("foo"));
console.log(Symbol.for("foo") === Symbol.for("foo"));
console.log(typeof Symbol.keyFor(Symbol.for("bar")));
console.log(Symbol.keyFor(Symbol.for("bar")));
console.log(Symbol.keyFor(Symbol("local")));
