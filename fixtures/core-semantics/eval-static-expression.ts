// static direct eval of a literal expression — compile-time expanded
let result = eval("1 + 2");
console.log(result);

// multi-statement eval — completion value is the last expression
let result2 = eval("1; 2;");
console.log(result2);
