// RegExp pattern extensions: backreferences, character class ranges,
// quantifier patterns, non-capturing groups, lookahead/lookbehind.
// (I-20260517-88A8FY)

// Backreferences
console.log(/(.)\1/.test("aa"));
console.log(/(.)\1/.test("ab"));
console.log(/^(\w)\1$/.test("zz"));

// Character class ranges
console.log(/[a-z]{3}/.test("abc"));
console.log(/[0-9]+/.test("123"));
console.log(/[A-Za-z0-9]+/.test("Hello123"));

// Quantifier patterns
console.log(/\d{5}/.test("12345"));
console.log(/\d{2,5}/.test("123"));
console.log(/\d{2,}/.test("12345"));

// Non-capturing groups
console.log(/(?:abc)+/.test("abcabc"));
console.log(/(?:abc|def)/.test("def"));

// Positive lookahead
console.log(/abc(?=def)/.test("abcdef"));

// Negative lookahead
console.log(/abc(?!def)/.test("abcxyz"));

// Positive lookbehind
console.log(/(?<=abc)def/.test("abcdef"));

// Negative lookbehind
console.log(/(?<!abc)def/.test("xyzdef"));

// Combined: char class + quantifier + backreference
console.log(/[\d][\12-\14]{1,}[^\d]/.test("5abc"));

// Character class with metacharacters
console.log(/[.+*?^$|()]/.test("."));
console.log(/[.+*?^$|()]/.test("+"));

// Named capture group (basic)
console.log(/(?<word>\w+)/.test("hello"));

// Named backreference (\k<name>)
console.log(/(?<word>\w+) \k<word>/.test("hello hello"));

// Alternation
console.log(/abc|def/.test("abc"));
console.log(/abc|def/.test("def"));

// Escaped forward slash
console.log(/^\/abc\/$/.test("/abc/"));

// Forward slash inside character class
console.log(/[/]/.test("/"));
