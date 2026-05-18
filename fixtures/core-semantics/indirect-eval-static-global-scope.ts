// Static indirect eval runs in global scope and must not capture caller locals.
let y = "local";

console.log((0, eval)("typeof y"));
globalThis.y = "global";
console.log((0, eval)("y"));
console.log(y);
