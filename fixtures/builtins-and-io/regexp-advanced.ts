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
