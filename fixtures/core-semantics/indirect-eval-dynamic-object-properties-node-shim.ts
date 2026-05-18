let source = "({ value: 7, label: 'ok' })";
let obj = globalThis.eval(source);
console.log(obj.value);
console.log(obj.label);
console.log(obj.missing);
