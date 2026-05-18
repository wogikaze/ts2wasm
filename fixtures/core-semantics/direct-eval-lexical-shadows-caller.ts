function shadowLet() {
  let value = "caller";
  let result = eval('let value = "eval"; value');
  return result + ":" + value;
}

function shadowVar() {
  var value = "caller";
  let result = eval('const value = "eval"; value');
  return result + ":" + value;
}

console.log(shadowLet());
console.log(shadowVar());
