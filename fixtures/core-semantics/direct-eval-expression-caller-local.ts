function readLocalThroughEval() {
  let x = 1;
  let y = eval("x + 2");
  return y;
}

console.log(readLocalThroughEval());
