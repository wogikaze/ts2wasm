// Promise.then should produce a precise unsupported diagnostic
const p: Promise<number> = Promise.resolve(42);
p.then((v) => console.log(v));
