// Sparse array iteration with identity predicates
// These tests verify sparse array handling in identity WAT functions.
// Note: identity-method WAT functions do NOT skip holes (they read stored values).
// Holes are stored as undefined, which is falsy.

// find: [1, , 3] — first truthy element is 1 (hole is undefined/falsy, skipped implicitly)
let sparse = [1, , 3];
let found = sparse.find(x => x);
if (found === 1) { console.log(1); } else { console.log(0); }

// find with no truthy elements in sparse array
let all_falsy = [0, , undefined, null];
let none = all_falsy.find(x => x);
if (none === undefined) { console.log(1); } else { console.log(0); }

// filter: truthy elements only (holes/undefined are falsy)
let filtered = sparse.filter(x => x);
if (filtered.length === 2) { console.log(1); } else { console.log(0); }
if (filtered[0] === 1) { console.log(1); } else { console.log(0); }
if (filtered[1] === 3) { console.log(1); } else { console.log(0); }

// some: find any truthy element — should find 3 even past hole
let sparse_some = [0, , 3];
let some_result = sparse_some.some(x => x);
if (some_result) { console.log(1); } else { console.log(0); }

// sparse array of all null/undefined (no truthy elements)
let all_none = [undefined, , null];
let none_found = all_none.find(x => x);
if (none_found === undefined) { console.log(1); } else { console.log(0); }
