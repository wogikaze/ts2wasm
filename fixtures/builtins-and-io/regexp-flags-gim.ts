// Test RegExp literal with g, i, m flags (parser level)
const re1 = /hello/g;
const re2 = /world/i;
const re3 = /test/m;
console.log(typeof re1, typeof re2, typeof re3);
