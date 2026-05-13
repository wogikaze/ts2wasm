const proto = { marker: 5 };
const nextProto = { marker: 9 };
const target = { x: 1, y: 2 };
Object.setPrototypeOf(target, proto);

function proxyOwnKeys(obj: any) {
  return ["x", "y"];
}

function proxyGetOwnPropertyDescriptor(obj: any, prop: string) {
  return Object.getOwnPropertyDescriptor(obj, prop);
}

function proxyDefineProperty(obj: any, prop: string, desc: any) {
  Object.defineProperty(obj, prop, desc);
  return true;
}

function proxyGetPrototypeOf(obj: any) {
  return Object.getPrototypeOf(obj);
}

function proxySetPrototypeOf(obj: any, newProto: any) {
  Object.setPrototypeOf(obj, newProto);
  return true;
}

const handler = {
  ownKeys: proxyOwnKeys,
  getOwnPropertyDescriptor: proxyGetOwnPropertyDescriptor,
  defineProperty: proxyDefineProperty,
  getPrototypeOf: proxyGetPrototypeOf,
  setPrototypeOf: proxySetPrototypeOf,
};

const proxy = new Proxy(target, handler);

console.log(Object.keys(proxy).join(","));

const desc = Object.getOwnPropertyDescriptor(proxy, "x");
console.log(desc.value);
console.log(desc.enumerable);

Object.defineProperty(proxy, "z", { value: 3, enumerable: true, configurable: true, writable: true });
console.log(target.z);

console.log(Object.getPrototypeOf(proxy).marker);

Object.setPrototypeOf(proxy, nextProto);
console.log(Object.getPrototypeOf(target).marker);

const revocable = Proxy.revocable(target, handler);
console.log(Object.keys(revocable).join(","));
