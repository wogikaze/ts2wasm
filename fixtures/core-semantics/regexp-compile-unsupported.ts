let r = new RegExp("abc");
console.log(r.test("abc"));
r.compile("def");
console.log(r.test("abc"));
console.log(r.test("def"));
console.log(r.compile("ghi"));
console.log(r.test("def"));
console.log(r.test("ghi"));
