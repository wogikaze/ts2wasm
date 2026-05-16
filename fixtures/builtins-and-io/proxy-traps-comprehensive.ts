// Proxy handler traps: compile-time dispatch for all 13 traps
const target: any = { x: 10, y: 20 };
const handler: any = {
  get(t: any, key: any, receiver?: any): any {
    const v = Reflect.get(t, key, receiver);
    return v;
  },
  set(t: any, key: any, value: any, receiver?: any): boolean {
    return Reflect.set(t, key, value, receiver);
  },
  has(t: any, key: any): boolean {
    return Reflect.has(t, key);
  },
  deleteProperty(t: any, key: any): boolean {
    return Reflect.deleteProperty(t, key);
  },
  ownKeys(t: any): any {
    return Reflect.ownKeys(t);
  },
  getOwnPropertyDescriptor(t: any, key: any): any {
    return Reflect.getOwnPropertyDescriptor(t, key);
  },
  defineProperty(t: any, key: any, desc: any): boolean {
    return Reflect.defineProperty(t, key, desc);
  },
  getPrototypeOf(t: any): any {
    return Reflect.getPrototypeOf(t);
  },
  setPrototypeOf(t: any, proto: any): boolean {
    return Reflect.setPrototypeOf(t, proto);
  },
  isExtensible(t: any): boolean {
    return Reflect.isExtensible(t);
  },
  preventExtensions(t: any): boolean {
    return Reflect.preventExtensions(t);
  },
  construct(t: any, argList: any): any {
    return Reflect.construct(t, argList);
  },
  apply(t: any, thisArg: any, argList: any): any {
    return Reflect.apply(t, thisArg, argList);
  },
};

const proxy: any = new Proxy(target, handler);

// --- Trap 1: get ---
const v = proxy.x;
console.log("t1-get:" + v);

// --- Trap 2: set ---
proxy.x = 42;
console.log("t2-set:" + proxy.x);

// --- Trap 3: has ---
console.log("t3-has:" + ("x" in proxy));

// --- Trap 4: deleteProperty ---
console.log("t4-del:" + delete proxy.x);

// --- Trap 5: ownKeys ---
const keys = Reflect.ownKeys(proxy);
console.log("t5-ok:" + keys.length);

// --- Trap 6: getOwnPropertyDescriptor ---
const d = Reflect.getOwnPropertyDescriptor(proxy, "y");
console.log("t6-gopd:" + (d !== undefined));

// --- Trap 7: defineProperty ---
Reflect.defineProperty(proxy, "z", { value: 100, writable: true });
console.log("t7-dp:" + proxy.z);

// --- Trap 8: getPrototypeOf ---
const p = Reflect.getPrototypeOf(proxy);
console.log("t8-gpo:" + (p !== null));

// --- Trap 9: setPrototypeOf ---
console.log("t9-spo:" + Reflect.setPrototypeOf(proxy, Object.prototype));

// --- Trap 10: isExtensible ---
console.log("t10-ie:" + Reflect.isExtensible(proxy));

// --- Trap 11: preventExtensions ---
console.log("t11-pe:" + Reflect.preventExtensions(proxy));

// --- Trap 12: apply ---
function add(a: number, b: number): number {
  return a + b;
}
const ap: any = new Proxy(add, handler);
console.log("t12-ap:" + Reflect.apply(ap, undefined, [1, 2]));

// --- Trap 13: construct ---
class Pt {
  x: number;
  y: number;
  constructor(x: number, y: number) {
    this.x = x;
    this.y = y;
  }
}
const cp: any = new Proxy(Pt, handler);
const pt = Reflect.construct(cp, [3, 4]);
console.log("t13-co:" + pt.x + "," + pt.y);
