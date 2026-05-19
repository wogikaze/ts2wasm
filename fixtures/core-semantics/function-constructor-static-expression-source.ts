let concatBody = Function("return " + "1");
let templateBody = Function(`return ${2}`);
let numberBody = Function(1 + 2);

console.log(concatBody());
console.log(templateBody());
console.log(numberBody());
console.log(numberBody.toString());
