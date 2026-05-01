// Global encodeURI - basic URI encoding
// Delegated to Node host shim ($host_encode_uri)

let x = encodeURI("hello world");
console.log(x);
let y = encodeURI("a=b&c=d");
console.log(y);
let z = encodeURI("https://example.com/path?name=value");
console.log(z);
