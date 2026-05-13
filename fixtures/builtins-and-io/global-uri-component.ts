// Global URI component encoding/decoding differs from encodeURI/decodeURI
// for reserved URI punctuation.

let encoded = encodeURIComponent("AZaz09-_.!~*'();,/?:@&=+$# ");
console.log(encoded);

let decoded = decodeURIComponent("%3B%2F%3F%3A%40%26%3D%2B%24%2C%23%20");
console.log(decoded);

let uriEncoded = encodeURI(";/?:@&=+$,# ");
console.log(uriEncoded);

let uriDecoded = decodeURI("%3B%2F%3F%3A%40%26%3D%2B%24%2C%23%20");
console.log(uriDecoded);
