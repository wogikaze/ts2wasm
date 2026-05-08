// Promise static methods — should produce precise unsupported diagnostics
const p1 = Promise.resolve(1);
const p2 = Promise.reject("err");
const p3 = Promise.all([p1]);
const p4 = Promise.race([p1]);
