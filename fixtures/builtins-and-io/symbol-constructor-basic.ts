// Basic Symbol constructor test
// Symbols are represented as "Symbol(description)" strings internally.

const s1 = Symbol("myDesc");
const s2 = Symbol();
console.log(s1, s2);
