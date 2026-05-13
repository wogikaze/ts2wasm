// Proxy handler traps - get/set/has/deleteProperty static slice
const target = { x: 10 };
function proxyGet(obj: any, prop: string) {
  return prop in obj ? obj[prop] : 42;
}
function proxySet(obj: any, prop: string, value: number) {
  obj[prop] = value;
  return true;
}
function proxyHas(obj: any, prop: string) {
  return prop === "x";
}
function proxyDeleteProperty(obj: any, prop: string) {
  delete obj[prop];
  return true;
}
const handler = {
  get: proxyGet,
  set: proxySet,
  has: proxyHas,
  deleteProperty: proxyDeleteProperty,
};
const proxy = new Proxy(target, handler);
console.log(proxy.x);
console.log(proxy.y);
proxy.y = 7;
console.log(target.y);
console.log("x" in proxy);
console.log("y" in proxy);
delete proxy.x;
console.log("x" in target);
