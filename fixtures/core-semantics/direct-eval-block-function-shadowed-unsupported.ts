// Expected diagnostic: this call is not intrinsic direct eval because the name
// `eval` is shadowed by a local binding. The resolver must keep it as an
// ordinary user call, so the later reference is unresolved instead of being
// created by AOT eval block-function lowering.
let eval = 1;

eval('{ function directEvalBlockFunctionShadowed() { return "wrong"; } }');
console.log(directEvalBlockFunctionShadowed());
