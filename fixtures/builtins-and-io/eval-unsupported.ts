// eval nested inside another expression — dynamic-code classification is not fully supported here.
// Static direct eval support is limited to focused compile-time expansion slices.
console.log(eval("1+1"));

// eval with template literal — not a compile-time string literal in the current eval lane
console.log(eval(`1+1`));

// eval with variable — runtime source needs the audited host eval lane
let x = "1+1";
console.log(eval(x));

// eval with concatenation — Binary expr, not a single static String
console.log(eval("1" + "+1"));

// indirect eval via comma expression — parser preserves the shape; semantics remain incomplete
console.log((0, eval)("1+1"));

// eval assigned to another variable — indirect eval lane remains incomplete
let e = eval;
console.log(e("1+1"));

// eval via object index — indirect eval lane remains incomplete
console.log(this["eval"]("1+1"));
