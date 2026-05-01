// Global unescape - legacy percent-decoding
// Delegated to Node host shim ($host_unescape)

let x = unescape("hello%20world");
console.log(x);
let y = unescape("%21%40%23%24%25");
console.log(y);
let z = unescape("test");
console.log(z);
