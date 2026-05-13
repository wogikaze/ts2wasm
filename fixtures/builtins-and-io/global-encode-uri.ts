// Global encodeURI - basic URI encoding

let x = encodeURI("hello world");
console.log(x);
let y = encodeURI("a=b&c=d");
console.log(y);
let z = encodeURI("https://example.com/path?name=value");
console.log(z);
let u = encodeURI("https://example.com/あ?q=é €");
console.log(u);
