// RegExp plus quantifier (+)
console.log(/a+/.test("aaa"));
console.log(/a+/.test(""));
console.log(/a+/.test("b"));
console.log("" + "aaab".match(/a+/));
console.log("" + "cat".match(/a+/));
console.log(/ab+c/.test("abc"));
console.log(/ab+c/.test("ac"));
console.log(/ab+c/.test("abbc"));
console.log("" + "abbc".match(/ab+c/));
