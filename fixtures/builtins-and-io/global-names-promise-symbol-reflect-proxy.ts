// Test that Promise, Symbol, Reflect, Proxy are recognized as known global names
// without requiring runtime implementation.
const p: any = Promise;
const s: any = Symbol;
const r: any = Reflect;
const pr: any = Proxy;
console.log(p, s, r, pr);