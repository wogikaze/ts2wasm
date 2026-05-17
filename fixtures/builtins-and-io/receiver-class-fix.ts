// Various methods that need receiver class resolution
// str.substr
const s = "hello world";
console.log(s.substr(0, 5));
console.log(s.substr(6));

// Date getters
const d = new Date(2024, 0, 15);
console.log(d.getFullYear());
console.log(d.getMonth());
console.log(d.getDate());

// RegExp compile
const r = /abc/;
console.log(r.test("abc"));
