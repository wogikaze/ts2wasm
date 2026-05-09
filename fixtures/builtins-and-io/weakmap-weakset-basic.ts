// WeakMap/WeakSet basic usage
const wm = new WeakMap();
const key1 = { id: 1 };
const key2 = { id: 2 };
wm.set(key1, "value1");
wm.set(key2, "value2");
console.log(wm.get(key1));

const ws = new WeakSet();
const obj1 = {};
const obj2 = {};
ws.add(obj1);
ws.add(obj2);
console.log(ws.has(obj1));
