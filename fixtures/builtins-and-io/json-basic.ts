// Basic JSON.parse and JSON.stringify operations

// JSON.stringify with various value types
console.log(JSON.stringify(42));
console.log(JSON.stringify("hello"));
console.log(JSON.stringify(true));
console.log(JSON.stringify(false));
console.log(JSON.stringify(null));
console.log(JSON.stringify({ a: 1, b: "two" }));
console.log(JSON.stringify([1, 2, 3]));
console.log(JSON.stringify({ x: [1, { y: 2 }] }));

// JSON.parse with various JSON strings
console.log(JSON.parse("42"));
console.log(JSON.parse('"hello"'));
console.log(JSON.parse("true"));
console.log(JSON.parse("false"));
console.log(JSON.parse("null"));
let obj = JSON.parse('{"a":1,"b":"two"}');
console.log(obj.a);
console.log(obj.b);
let arr = JSON.parse("[1,2,3]");
console.log(arr.length);
console.log(arr[1]);

// Round-trip test: parse then stringify
console.log(JSON.stringify(JSON.parse('{"a":1,"b":2}')));
