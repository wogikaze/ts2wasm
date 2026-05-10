// Test in operator with prototype chain
const proto = { inherited: 42 };
const child = Object.create(proto);
// inherited is on proto, not own property of child
console.log("inherited" in child ? "true" : "false");
console.log("nonexistent" in child ? "true" : "false");

// own property should still work (own overrides proto)
child.inherited = 99;
console.log("inherited" in child ? "true" : "false");

// setPrototypeOf also works
const obj = {};
Object.setPrototypeOf(obj, { method: true });
console.log("method" in obj ? "true" : "false");

console.log(0);
