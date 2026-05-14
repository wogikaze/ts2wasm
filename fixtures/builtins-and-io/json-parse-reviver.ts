function reviver(key, value) {
  if (key === "a") return 999;
  return value;
}

let obj = JSON.parse('{"a":1,"b":2}', reviver);
console.log(obj.a);
console.log(obj.b);

function identity(key, value) {
  return value;
}

let obj2 = JSON.parse('{"x":10}', identity);
console.log(obj2.x);
