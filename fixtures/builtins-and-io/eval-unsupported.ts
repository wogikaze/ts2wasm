// eval nested inside another expression — name resolver rejects (UnsupportedEval)
// The parser's extract-and-replace only works for top-level eval('...') statements.
console.log(eval("1+1"));

// eval with template literal — not Expr::String, name resolver rejects
console.log(eval(`1+1`));

// eval with variable — not a literal, name resolver rejects
let x = "1+1";
console.log(eval(x));

// eval with concatenation — Binary expr, not a single String, name resolver rejects
console.log(eval("1" + "+1"));

// indirect eval via comma expression — parser rejects (indirect_eval_call_diagnostic)
console.log((0, eval)("1+1"));

// eval assigned to another variable — callee is not Ident("eval"), name resolver rejects
let e = eval;
console.log(e("1+1"));

// eval via object index — parser rejects as indirect eval
console.log(this["eval"]("1+1"));
