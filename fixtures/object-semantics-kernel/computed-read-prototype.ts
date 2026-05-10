// Test computed read through prototype chain
const proto = { inherited: 42 };
const child = Object.create(proto);
// Dynamic key access through prototype
const key = "inherited";
console.log(child[key]); // Should be 42 (prototype chain)

// Own property should still work
child.own = 99;
const ownKey = "own";
console.log(child[ownKey]); // Should be 99

// Own property overrides prototype
child[key] = 7;
console.log(child[key]); // Should be 7 (own overrides proto)

// Nonexistent key
console.log(child["nonexistent"]); // Should be undefined

console.log(0);
