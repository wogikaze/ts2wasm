// Global decodeURI - basic URI decoding

let x = decodeURI("hello%20world");
console.log(x);
let y = decodeURI("a%3Db%26c%3Dd");
console.log(y);
let z = decodeURI("https%3A%2F%2Fexample.com%2Fpath%3Fname%3Dvalue");
console.log(z);
let u = decodeURI("https://example.com/%E3%81%82?q=%C3%A9%20%E2%82%AC");
console.log(u);
