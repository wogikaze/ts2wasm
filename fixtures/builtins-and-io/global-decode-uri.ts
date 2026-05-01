// Global decodeURI - basic URI decoding
// Delegated to Node host shim ($host_decode_uri)

let x = decodeURI("hello%20world");
console.log(x);
let y = decodeURI("a%3Db%26c%3Dd");
console.log(y);
let z = decodeURI("https%3A%2F%2Fexample.com%2Fpath%3Fname%3Dvalue");
console.log(z);
