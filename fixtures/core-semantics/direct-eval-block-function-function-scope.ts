function outer() {
  eval('{ function f() { return "function-scope"; } }');
  return f();
}

console.log(outer());
