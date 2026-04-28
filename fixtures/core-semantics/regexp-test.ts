// Issue 051: literal-backed RegExp.prototype.test plain-pattern subset.
console.log(/abc/.test("zabcx"));
console.log(/abc/.test("zabx"));
console.log(/needle/g.test("haystack needle"));
