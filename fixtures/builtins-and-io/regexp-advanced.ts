// Test advanced RegExp features: dotAll (s flag), unicode (u flag), sticky (y flag)
// Build-smoke test: confirms compiler accepts these flags for method calls.
const dotAll = /hello.world/s;
console.log(dotAll.test("hello\nworld"));

const unicode = /abc/u;
console.log(unicode.test("abc"));

const sticky = /abc/y;
console.log(sticky.test("abc"));

const multiFlags = /test/gim;
console.log(multiFlags.test("TEST"));

// Case-insensitive with i flag
const caseInsensitive = /hello/i;
console.log(caseInsensitive.test("HELLO"));

// Global flag
const globalFlag = /a/g;
console.log(globalFlag.test("aaa"));

// Combined flags: dotAll + global
const dotAllGlobal = /hello.world/gs;
console.log(dotAllGlobal.test("hello\nworld"));

// Basic exec with flags
const execTest = /abc/g;
const execResult = execTest.exec("abcabc");
console.log(execResult !== null);

// String match with global flag
const matchGlobal = "abcabc".match(/a/g);
console.log(matchGlobal !== null);

// String replace with global flag
const replaceGlobal = "abcabc".replace(/a/g, "x");
console.log(replaceGlobal === "xbcxbc");
