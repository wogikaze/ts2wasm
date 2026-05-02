// RegExp digit \d
console.log(/\d/.test("5"));
console.log(/\d/.test("a"));
console.log(/\d/.test(""));
console.log("" + "abc123".match(/\d+/));
console.log("" + "abc".match(/\d/));
console.log("" + "a1b2c3".match(/\d\d/));
console.log(/\d\d\d/.test("123"));
console.log(/\d\d\d/.test("12a"));
