// Test RegExp literal with s, u, y flags (parser level).
// The d/indices flag remains unsupported by issue-202.
const re1 = /abc/s;
const re2 = /abc/u;
const re3 = /abc/y;
console.log(typeof re1, typeof re2, typeof re3);
