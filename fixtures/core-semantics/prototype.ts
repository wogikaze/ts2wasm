// Test prototype chain lookup
const parent = { a: 1 };
const child = { b: 2 };
// Note: Object.create not yet supported, so we'll test own properties only for now
console.log(child.a); // Should be undefined (no prototype yet)
console.log(child.b); // Should be 2 (own property)
