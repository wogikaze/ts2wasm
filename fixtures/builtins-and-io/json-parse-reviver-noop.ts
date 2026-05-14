function reviver(key, value) {
  return value;
}

var obj = JSON.parse('{"a":1,"b":"hello"}', reviver);
console.log(obj.a);
console.log(obj.b);
