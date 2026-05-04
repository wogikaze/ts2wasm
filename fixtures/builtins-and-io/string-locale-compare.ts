// String.prototype.localeCompare tests
// Note: simple lexicographic comparison (no locale-specific behavior)

// Equal strings
let eq = "hello".localeCompare("hello");
console.log(eq);

// Less than
let lt = "apple".localeCompare("banana");
console.log(lt);

// Greater than
let gt = "banana".localeCompare("apple");
console.log(gt);

// Shorter string is less when prefix
let prefix = "hello".localeCompare("hello world");
console.log(prefix);

// Longer string is greater when prefixed
let longer = "hello world".localeCompare("hello");
console.log(longer);

// Empty string
let empty = "".localeCompare("");
console.log(empty);

// Empty vs non-empty
let emptyVsNon = "".localeCompare("a");
console.log(emptyVsNon);
