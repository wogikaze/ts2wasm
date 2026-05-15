// WeakRef and FinalizationRegistry (issue I-20260513-BQTVQV)
// WeakMap/WeakSet are supported; WeakRef/FinalizationRegistry are not yet implemented.
var wm = new WeakMap<object, string>();
var obj: object = { x: 1 };
wm.set(obj, "value");
console.log("weakmap_ok");
console.log(wm.get(obj));
