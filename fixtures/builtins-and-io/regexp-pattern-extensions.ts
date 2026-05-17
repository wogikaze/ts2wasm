// RegExp backreferences and extended patterns
console.log(/(.)\1/.test("aa"));
console.log(/(.)\1/.test("ab"));
console.log(/(.)\1/.test("bb"));
console.log(/(.)\1/.test("ba"));
console.log(/^(\w)\1$/.test("zz"));
console.log(/^(\w)\1$/.test("zx"));
