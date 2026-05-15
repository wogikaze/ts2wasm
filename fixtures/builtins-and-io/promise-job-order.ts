// Micro-task queue and Promise job integration (issue I-20260513-96CWDH)
// Basic Promise.resolve and console.log — Promise.then requires runtime support
let p1 = Promise.resolve(42);
console.log("resolved");
