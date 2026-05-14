function dropB(key, value) {
  if (key === "b") {
    return undefined;
  }
  return value;
}

var obj = JSON.parse('{"a":1,"b":2,"c":3}', dropB);
console.log(obj.a);
console.log(obj.b === undefined ? "undefined" : obj.b);
console.log(obj.c);
