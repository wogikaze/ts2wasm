// Global escape - legacy percent-encoding
// Delegated to Node host shim ($host_escape)

let x = escape("hello world");
console.log(x);
let y = escape(" !@#$%");
console.log(y);
let z = escape("test");
console.log(z);
