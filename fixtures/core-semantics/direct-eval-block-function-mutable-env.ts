let initialBV;
let currentBV;
let varBinding;

(function() {
  eval('{ function f() { initialBV = f; f = 123; currentBV = f; return "decl"; } }varBinding = f; f();');
}());

console.log(initialBV());
console.log(currentBV);
console.log(varBinding());
