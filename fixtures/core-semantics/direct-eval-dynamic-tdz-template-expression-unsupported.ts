// Expected diagnostic: the eval source references the later `let later`
// binding from a template expression. Until descriptor v2 models TDZ
// bindings precisely, this remains a guarded UnsupportedEval boundary.
function run() {
  let source = "`${later}`";
  let result = eval(source);
  let later = "after";
  console.log(result);
}

run();
