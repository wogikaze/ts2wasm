// RegExp question mark quantifier (?)
console.log(/a?/.test("a"));
console.log(/a?/.test(""));
console.log(/ab?c/.test("ac"));
console.log(/ab?c/.test("abc"));
console.log(/ab?c/.test("abbc"));
console.log("" + "ac".match(/ab?c/));
console.log("" + "abc".match(/ab?c/));
console.log(/\d?/.test("5"));
console.log(/\d?/.test("a"));
