// RegExp.prototype.compile
const r = /abc/;
console.log(r.test("abc"));
r.compile("xyz");
console.log(r.test("abc"));
console.log(r.test("xyz"));
r.compile("def", "i");
console.log(r.test("DEF"));
