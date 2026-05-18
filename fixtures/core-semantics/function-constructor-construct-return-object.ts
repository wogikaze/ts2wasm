let F = Function("return { value: 7 }");
let obj = new F();
console.log(obj.value);
console.log(obj instanceof F);

let G = Function("return 5");
let other = new G();
console.log(typeof other);
console.log(other instanceof G);
