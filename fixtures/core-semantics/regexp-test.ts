// Issue 051: literal-backed RegExp.prototype.test plain-pattern subset.
console.log(/abc/.test("zabcx"));
console.log(/abc/.test("zabx"));
console.log(/needle/g.test("haystack needle"));
let plain = new RegExp("abc");
console.log(plain.test("zabcx"));
console.log(plain.test("zabx"));
console.log("" + "zabcx".match(/abc/));
console.log("" + "zabx".match(/abc/));
console.log("" + "haystack needle".match(new RegExp("needle")));
console.log("haystack".match(new RegExp("needle")) === null);
