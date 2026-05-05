// String.prototype.isWellFormed basic tests
// All strings are byte-level (no UTF-16 surrogates), so all return true
console.log("hello".isWellFormed());
console.log("".isWellFormed());
console.log("abc123".isWellFormed());
console.log(" ".isWellFormed());
console.log("line1\nline2".isWellFormed());
console.log("emoji 👍".isWellFormed());
console.log("中文 español".isWellFormed());
