let s = require("fs").readFileSync(0, "utf8");
let one = { x: 1n };
console.log(one.x == s);
console.log(s == one.x);
console.log(one.x < s);
console.log(s > one.x);
