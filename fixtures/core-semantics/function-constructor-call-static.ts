// Static literal Function(...) constructor fixture; compiler expands this at compile time.
let f = Function("return 1");
console.log(f());
