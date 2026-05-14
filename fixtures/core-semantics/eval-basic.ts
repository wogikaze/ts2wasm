function rewriteLocal() {
  let value = "before";
  eval('value = "after";');
  return value;
}

console.log(rewriteLocal());
