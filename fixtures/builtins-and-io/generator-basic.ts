// Basic generator function — ID 212 (W5)
// Parser erases generator body; this tests that the compiler accepts
// function* syntax and registers the function so calls resolve.
function* gen() {}
const g = gen();
console.log(g);
