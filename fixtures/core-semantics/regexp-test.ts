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
console.log("" + /abc/.exec("zabcx"));
console.log(/abc/.exec("zabx") === null);
let execNeedle = new RegExp("needle");
console.log("" + execNeedle.exec("haystack needle"));
console.log(execNeedle.exec("haystack") === null);
let execPlain = new RegExp("plain");
console.log("" + execPlain.exec("very plain text"));
