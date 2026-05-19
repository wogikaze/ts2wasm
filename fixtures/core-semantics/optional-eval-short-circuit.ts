// Optional chaining short-circuits before reaching eval.
let obj = null;
console.log(obj?.eval?.("not-executed"));

let a = { b: null };
console.log(a?.b?.eval?.("not-executed"));

let obj2 = { eval: (s) => "result:" + s };
console.log(obj2?.eval?.("executed"));
