// Proxy should produce a clear unsupported diagnostic
const target = { x: 42 };
const handler = {};
const proxy = new Proxy(target, handler);
console.log(proxy.x);
