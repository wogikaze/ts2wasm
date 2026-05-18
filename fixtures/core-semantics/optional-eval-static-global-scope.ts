// Optional eval is indirect eval and must not capture caller locals.
let y = "local";

console.log(eval?.("typeof y"));
globalThis.y = "global";
console.log(eval?.("y"));
console.log(y);
