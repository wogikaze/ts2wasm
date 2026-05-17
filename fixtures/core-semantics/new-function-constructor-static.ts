// Static literal new Function(...) constructor fixture; compiler expands this at compile time.
let f = new Function("return 1");
console.log(f());
