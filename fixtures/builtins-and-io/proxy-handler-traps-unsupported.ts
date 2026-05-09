// Proxy handler traps — should produce unsupported diagnostic
const target = { x: 10 };
const handler = {
  get(obj: any, prop: string) {
    return prop in obj ? obj[prop] : 42;
  }
};
const proxy = new Proxy(target, handler);
console.log(proxy.x);
console.log(proxy.y);
