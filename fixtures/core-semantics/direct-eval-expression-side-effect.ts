function rewriteLocalThroughEvalExpression() {
  let x = "before";
  let result = eval('x = "after"; x');
  console.log(result);
  return x;
}

console.log(rewriteLocalThroughEvalExpression());
