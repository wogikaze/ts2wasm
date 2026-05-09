// W5.3: Prototype chain inheritance with property semantics
// Tests OrdinaryGet prototype walk, shadowing, deletion

const parent = { inherited: 42 };
const child = {};
Object.setPrototypeOf(child, parent);

// 1. Reading inherited property
console.log(child.inherited);  // 42 — prototype walk

// 2. Writing creates own property (shadows inherited)
child.inherited = 99;
console.log(child.inherited);  // 99
console.log(parent.inherited); // 42 — parent unchanged

// 3. Delete own property → inherited visible again
delete child.inherited;
console.log(child.inherited);  // 42

// 4. Two-level prototype chain
const grandparent = { top: "root" };
Object.setPrototypeOf(parent, grandparent);
console.log(child.top);        // "root" — two-level walk

// 5. Multi-level write shadowing
child.top = "shadow";
console.log(child.top);        // "shadow"
console.log(grandparent.top);  // "root"
console.log(parent.top);       // "root" (parent doesn't have own "top")
