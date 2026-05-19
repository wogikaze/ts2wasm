// Global lexical binding vs global object property in indirect eval.
// `let` at global scope creates a global lexical binding (not on globalThis).
// Indirect eval reads from the global lexical environment, so both work.
// But `var` declarations inside indirect eval create globalThis properties,
// while `let` declarations inside indirect eval do not.
let lexicalGlobal = "lexical-value";
var varGlobal = "var-value";

console.log((0, eval)("lexicalGlobal"));
console.log((0, eval)("varGlobal"));

(0, eval)("var indirectVar = 'from-var'");
console.log(globalThis.indirectVar);

(0, eval)("let indirectLet = 'from-let'");
console.log(typeof globalThis.indirectLet);
console.log((0, eval)("indirectLet"));
