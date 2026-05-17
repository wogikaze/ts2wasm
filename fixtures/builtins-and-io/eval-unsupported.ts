// Static direct eval nested inside another expression is supported; later
// runtime-source cases keep this fixture in the unsupported set.
console.log(eval("1+1"));

// eval with template literal — not a compile-time string literal in the current eval lane
console.log(eval(`1+1`));

// eval with variable — runtime source needs the audited host eval lane
let x = "1+1";
console.log(eval(x));

// eval with concatenation — Binary expr, not a single static String
console.log(eval("1" + "+1"));

// indirect eval via comma expression with runtime source — host lane is still incomplete
console.log((0, eval)(x));

// eval assigned to another variable — eval alias analysis remains incomplete
let e = eval;
console.log(e("1+1"));

// eval via arbitrary object index — not one of the supported static globalThis shapes
console.log(this["eval"]("1+1"));
