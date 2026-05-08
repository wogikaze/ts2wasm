// Dynamic eval should produce a clear unsupported diagnostic
const fn = "1 + 2";
console.log(eval(fn));
