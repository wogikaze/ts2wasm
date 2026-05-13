// Global unescape - legacy percent-decoding

let x = unescape("hello%20world");
console.log(x);
let y = unescape("%21%40%23%24%25");
console.log(y);
let z = unescape("test");
console.log(z);
let u = unescape("%E9%20%u3042%20%u20AC");
console.log(u);
