// RegExp capture groups, alternation, and quantifiers
console.log(/a(b)c/.test("abc"));
console.log(/a|b/.test("a"));
console.log(/a|b/.test("b"));
console.log(/a|b/.test("c"));

// Quantifiers
console.log(/a{3}/.test("aaa"));
console.log(/a{3}/.test("aa"));
console.log(/a{3}/.test("aaaa"));

console.log(/a{2,}/.test("aa"));
console.log(/a{2,}/.test("a"));

console.log(/a{1,3}/.test("a"));
console.log(/a{1,3}/.test("aa"));
console.log(/a{1,3}/.test("aaa"));

// Alternation with grouping
console.log(/(a|b)c/.test("ac"));
console.log(/(a|b)c/.test("bc"));
console.log(/(a|b)c/.test("xc"));
