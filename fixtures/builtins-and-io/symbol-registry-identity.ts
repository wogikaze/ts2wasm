const registryFirst = Symbol.for("shared");
const registrySecond = Symbol.for("sha" + "red");
const ordinary = Symbol("shared");

console.log(registryFirst === registrySecond);
console.log(registryFirst === ordinary);
console.log(Symbol.keyFor(registryFirst));
console.log(Symbol.keyFor(ordinary));
console.log(registryFirst.description);
console.log(ordinary.description);
