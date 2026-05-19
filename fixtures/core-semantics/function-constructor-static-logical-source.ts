let orBody = Function("" || "return 2");
let andBody = Function("return 1" && "return 3");
let nullishBody = Function(null ?? "return 4");
let keepBody = Function("return 5" ?? "return 6");
let falseBody = Function(false && "return 7");

console.log(orBody());
console.log(andBody());
console.log(nullishBody());
console.log(keepBody());
console.log(falseBody());
console.log(falseBody.toString());
