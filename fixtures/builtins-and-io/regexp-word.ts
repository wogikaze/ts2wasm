// RegExp word \w matches [a-zA-Z0-9_]
console.log(/\w/.test("a"));
console.log(/\w/.test("5"));
console.log(/\w/.test("_"));
console.log(/\w/.test("."));
console.log(/\w/.test(" "));
console.log("" + "hello world".match(/\w+/));
console.log("" + "a b c".match(/\w/));
