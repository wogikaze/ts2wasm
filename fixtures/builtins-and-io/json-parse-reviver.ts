function revive(key, value) {
  if (key === "0") {
    return "first";
  }
  if (key === "n") {
    return value + 1;
  }
  if (key === "drop") {
    return undefined;
  }
  return value;
}

let obj = JSON.parse('{"n":1,"drop":2,"items":[1,2]}', revive);
console.log(JSON.stringify(obj));
console.log(JSON.stringify(obj.items));

function reviver(key, value) {
  if (key === "a") {
    return 999;
  }
  return value;
}

let obj2 = JSON.parse('{"a":1,"b":2}', reviver);
console.log(obj2.a);
console.log(obj2.b);

function identity(key, value) {
  return value;
}

let obj3 = JSON.parse('{"x":10}', identity);
console.log(obj3.x);
