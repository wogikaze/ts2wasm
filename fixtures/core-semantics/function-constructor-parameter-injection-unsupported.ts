let f = Function("a) { return 1; } function injected(", "return 2");
console.log(f());
