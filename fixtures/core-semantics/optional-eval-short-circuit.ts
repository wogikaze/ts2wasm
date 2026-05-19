// Optional eval with nullish callee: test result returns undefined.
// Tests both null and undefined callee produce undefined result.
let eval = null;
let r1 = eval?.("not-executed");
console.log(r1 === undefined);

eval = undefined;
let r2 = eval?.("not-executed");
console.log(r2 === undefined);
