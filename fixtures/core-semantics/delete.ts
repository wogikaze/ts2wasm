// Test delete operator
const obj = { a: 1 };
console.log(obj.a); // Should be 1
delete obj.a;
console.log(obj.a); // Should be undefined (0)
console.log(0); // Exit marker
