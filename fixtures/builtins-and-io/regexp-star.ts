// RegExp star quantifier (*)
console.log(/a*/.test("aaa"));
console.log(/a*/.test(""));
console.log(/a*/.test("b"));
console.log("" + "aaab".match(/a*/));
console.log("" + "bbb".match(/a*/));
console.log(/ab*c/.test("abc"));
console.log(/ab*c/.test("ac"));
console.log(/ab*c/.test("abbc"));
console.log("" + "ac".match(/ab*c/));
console.log("" + "abbc".match(/ab*c/));
