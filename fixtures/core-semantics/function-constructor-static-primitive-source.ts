let nullBody = Function(null);
let boolBody = new Function(true);

console.log(nullBody());
console.log(boolBody());
console.log(boolBody.name);
console.log(boolBody.length);
console.log(boolBody.toString());
