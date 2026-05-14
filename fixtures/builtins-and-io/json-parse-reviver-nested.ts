function reviver(key, value) {
  if (typeof value === "number") {
    return value + 10;
  }
  return value;
}

var obj = JSON.parse('{"x":{"y":1,"z":2}}', reviver);
console.log(obj.x.y);
console.log(obj.x.z);
