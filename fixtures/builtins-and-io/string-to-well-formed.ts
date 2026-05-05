// String.prototype.toWellFormed basic tests
// All strings are byte-level (no UTF-16 surrogates), so strings are returned unchanged
console.log("hello".toWellFormed());
console.log("".toWellFormed());
console.log("abc123".toWellFormed());
console.log(" ".toWellFormed());
console.log("line1\nline2".toWellFormed());
console.log("emoji 👍".toWellFormed());
console.log("中文 español".toWellFormed());
