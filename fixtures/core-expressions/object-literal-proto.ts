let proto = { kind: 7 };
let object = { __proto__: proto, own: 3 };

console.log(object.kind);
console.log(object.own);
console.log(object.__proto__ === proto);
