// W5.4: configurable:false via Object.freeze + descriptor inspection
// Tests that Object.freeze makes configurable:false on all properties

const obj = { a: 1, b: 2 };
Object.freeze(obj);

// Descriptor inspection: configurable should be false
let da = Object.getOwnPropertyDescriptor(obj, "a");
console.log(da.value);        // 1
console.log(da.writable);     // false
console.log(da.configurable); // false

let db = Object.getOwnPropertyDescriptor(obj, "b");
console.log(db.value);        // 2
console.log(db.configurable); // false

// delete on frozen object: silently rejected
let deleted = delete obj.a;
console.log(deleted);         // false
console.log(obj.a);           // 1 — still exists
