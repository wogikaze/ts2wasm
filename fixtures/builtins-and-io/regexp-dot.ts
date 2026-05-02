// RegExp dot (.) matches any character except newline
console.log(/./.test("a"));
console.log(/./.test(" "));
console.log(/./.test("\n"));
console.log(/.../.test("abc"));
console.log(/.../.test("ab"));
console.log("" + "abc".match(/./));
console.log("" + "abc".match(/b./));
console.log("" + "hello".match(/l./));
console.log("" + "cat".match(/c.t/));
console.log("" + "ct".match(/c.t/));
