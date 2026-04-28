let obj = JSON.parse('{"a":"x\\\"y","b":["c\\\\d"]}');
console.log(obj.a);
console.log(obj.b[0]);
