let unary = Function(...["return 11"]);
let withParam = Function(...["value", "return value + 1"]);
let combined = Function("left", ...["right", "return left + right"]);

console.log(unary());
console.log(withParam(12));
console.log(combined(2, 3));
console.log(withParam.toString());
