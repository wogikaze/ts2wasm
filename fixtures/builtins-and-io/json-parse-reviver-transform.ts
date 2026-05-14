function reviver(key, value) {
  if (typeof value === "number") {
    return value * 2;
  }
  return value;
}

var obj = JSON.parse('{"a":1,"b":2}', reviver);
console.log(obj.a);
console.log(obj.b);
