// Promise static methods: resolve, reject, all, race
let p1 = Promise.resolve(1);
let p2 = Promise.resolve(2);

// Verify all with already-settled promises
let allResult = Promise.all([p1, p2]);
console.log("all-done");

// Verify race with already-settled promise
let raceResult = Promise.race([p1, p2]);
console.log("race-done");

// Verify resolve works
let resolved = Promise.resolve(42);
console.log("resolve-done");
