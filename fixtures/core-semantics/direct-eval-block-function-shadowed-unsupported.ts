let eval = 1;

eval('{ function directEvalBlockFunctionShadowed() { return "wrong"; } }');
console.log(directEvalBlockFunctionShadowed());
