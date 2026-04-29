let obj = JSON.parse('{"s":"\\u00e9"}');
console.log(obj.s);
console.log(JSON.stringify(obj));
