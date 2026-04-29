let init = "unset";
let changed = "unset";

eval('init = f; f = 123; changed = f;{ function f() { return "decl"; } }');

console.log(init === undefined);
console.log(changed);
