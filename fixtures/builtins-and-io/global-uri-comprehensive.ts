// Comprehensive URI encode/decode coverage with edge cases round-tripping

// --- encodeURI ---
// Basic ASCII: unreserved chars pass through unchanged
console.log(encodeURI("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789"));
// Reserved characters are not encoded by encodeURI
console.log(encodeURI(";/?:@&=+$,#"));
// Space and control-ish
console.log(encodeURI("a b"));
// Empty string
console.log(encodeURI(""));

// --- decodeURI ---
// Basic percent-encoded ASCII
console.log(decodeURI("hello%20world"));
// Mixed literal + encoded
console.log(decodeURI("a%3Db"));
// Empty string
console.log(decodeURI(""));
// No encoding needed
console.log(decodeURI("plaintext"));

// --- encodeURIComponent ---
// Reserved chars that encodeURIComponent encodes (unlike encodeURI)
console.log(encodeURIComponent(";/?:@&=+$,#"));
// Space
console.log(encodeURIComponent("a b"));
// Empty
console.log(encodeURIComponent(""));

// --- decodeURIComponent ---
// Basic
console.log(decodeURIComponent("%3B%2F%3F%3A%40%26%3D%2B%24%2C%23%20"));
// Empty
console.log(decodeURIComponent(""));

// --- Round-trip tests ---
// encodeURI -> decodeURI
let original = "hello world & more";
let encoded = encodeURI(original);
console.log(decodeURI(encoded));

// encodeURIComponent -> decodeURIComponent
let original2 = "a=b&c=d e/f";
let encoded2 = encodeURIComponent(original2);
console.log(decodeURIComponent(encoded2));

// --- UTF-8 multi-byte sequences ---
// 2-byte UTF-8: é (U+00E9)
console.log(encodeURI("é"));
// 3-byte UTF-8: あ (U+3042)
console.log(encodeURI("あ"));
// 3-byte UTF-8: € (U+20AC)
console.log(encodeURI("€"));
// Mixed
console.log(encodeURIComponent("é あ €"));

// Round-trip UTF-8
let utfOriginal = "Hello 世界 🌍";
console.log(decodeURIComponent(encodeURIComponent(utfOriginal)));

// --- decodeURI with UTF-8 percent-encoded ---
// 2-byte sequence
console.log(decodeURI("%C3%A9"));
// 3-byte sequence
console.log(decodeURI("%E3%81%82"));
// Decode what encodeURIComponent produces
let u = encodeURIComponent("Hello 世界");
console.log(decodeURIComponent(u));
