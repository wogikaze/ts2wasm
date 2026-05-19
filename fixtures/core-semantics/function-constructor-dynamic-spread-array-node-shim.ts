let parts = ["value", "return value + 5"];
let generated = Function(...parts);

console.log(generated(7));
