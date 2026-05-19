// Optional eval with nullish callee short-circuits to undefined.
let eval = null;
console.log(eval?.("not-executed"));

eval = undefined;
console.log(eval?.("not-executed"));
