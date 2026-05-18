function outer() {
  eval("var x = 1; function g() { return x; }");
  return g();
}

console.log(outer());
