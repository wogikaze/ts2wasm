// Object.prototype methods test
let obj: any = { x: 42, y: "hello" };

// hasOwnProperty
let h1: any = obj.hasOwnProperty("x");
let h2: any = obj.hasOwnProperty("z");

// propertyIsEnumerable
let p1: any = obj.propertyIsEnumerable("x");
let p2: any = obj.propertyIsEnumerable("z");

// isPrototypeOf
let proto: any = { a: 1 };
let child: any = Object.create(proto);
let ip1: any = proto.isPrototypeOf(child);
let ip2: any = child.isPrototypeOf(proto);
let ip3: any = proto.isPrototypeOf(proto);

// valueOf
let v1: any = obj.valueOf() === obj;

// toString
let ts1: any = ({}.toString());

// toLocaleString
let tls1: any = ({}.toLocaleString());

console.log(h1);
console.log(h2);
console.log(p1);
console.log(p2);
console.log(ip1);
console.log(ip2);
console.log(ip3);
console.log(v1);
console.log(ts1);
console.log(tls1);
