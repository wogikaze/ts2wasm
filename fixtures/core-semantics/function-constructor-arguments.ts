let f = Function("return arguments.length + ':' + arguments[0] + ':' + arguments[1]");
console.log(f("a", "b"));

let g = new Function("return arguments.length");
console.log(g(1, 2, 3));
