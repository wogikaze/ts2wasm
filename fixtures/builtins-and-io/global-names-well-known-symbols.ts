// Test that well-known Symbol properties are recognized.
const iter: any = Symbol.iterator;
const tag: any = Symbol.toStringTag;
const inst: any = Symbol.hasInstance;
const prim: any = Symbol.toPrimitive;
const symfor: any = Symbol.for;
const symkey: any = Symbol.keyFor;
console.log(iter, tag, inst, prim, symfor, symkey);
