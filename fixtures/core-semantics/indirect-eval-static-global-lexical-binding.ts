// Global lexical binding vs global object property in indirect eval.
// var at top level creates a globalThis property, let does not.
// Indirect eval can read globalThis properties.
var globalVar = "global-var-value";

// var is readable through indirect eval (becomes globalThis property)
console.log((0, eval)("globalVar"));

// let at top level does NOT create globalThis property
let globalLet = "global-let-value";
console.log(typeof globalThis.globalLet);

// var inside indirect eval creates globalThis property
(0, eval)("var evalVar = 'from-var'");
console.log(globalThis.evalVar);

// let inside indirect eval does NOT create globalThis property
(0, eval)("let evalLet = 'from-let';");
console.log(typeof globalThis.evalLet);
