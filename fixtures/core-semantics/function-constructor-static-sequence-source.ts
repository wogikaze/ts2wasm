let body = Function((0, "return 9"));
let withParam = Function((false, "value"), (1, "return value + 2"));
let numberBody = Function(("ignored", 1 + 2));

console.log(body());
console.log(withParam(5));
console.log(numberBody());
console.log(body.toString());
console.log(withParam.toString());
console.log(numberBody.toString());
