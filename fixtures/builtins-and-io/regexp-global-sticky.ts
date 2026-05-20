// Test RegExp global (/g) flag with multiple calls
// In JavaScript, global regexp tracks lastIndex across calls

// Global flag: first call with test()
const globalRe = /a/g;
console.log(globalRe.test("aaa"));
// After first test(), lastIndex is 1 (or last match end)

// Sticky flag with test()
const stickyRe = /a/y;
console.log(stickyRe.test("aaa"));

// Sticky flag: no match at position 0
const stickyNoMatch = /b/y;
console.log(stickyNoMatch.test("aaa"));

// Global with exec (single call)
const globalExec = /abc/g;
console.log(globalExec.exec("abcabc") !== null);

// Global with String.match
console.log("abcabc".match(/a/g) !== null);

// Global with replace
console.log("abcabc".replace(/a/g, "x") === "xbcxbc");
