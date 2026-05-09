// W5.5: Enumerable filtering via Object.keys
// Object.keys should only return enumerable own properties

const obj = { a: 1, b: 2, c: 3 };

// Object.keys returns enumerable own keys (all literal props are enumerable)
let keys = Object.keys(obj);
console.log(keys.length);     // 3

// Object.values returns corresponding values
let vals = Object.values(obj);
console.log(vals.length);     // 3
console.log(vals[0]);         // 1
console.log(vals[1]);         // 2
console.log(vals[2]);         // 3

// Object.entries returns [key, value] pairs
let entries = Object.entries(obj);
console.log(entries.length);  // 3

// After freeze: keys/values/entries still work (freeze doesn't change enumerability)
Object.freeze(obj);
let keys2 = Object.keys(obj);
console.log(keys2.length);    // 3
let vals2 = Object.values(obj);
console.log(vals2.length);    // 3
