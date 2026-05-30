let result = 0;
(function() {
  {
    function f() { result = 1; }
  }
  f();
})();
console.log(result);
