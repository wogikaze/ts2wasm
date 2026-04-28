let obj = JSON.parse('{"a":"\\u0041\\u005a","b":["x\\u002fy"]}');
console.log(obj.a);
console.log(obj.b[0]);
