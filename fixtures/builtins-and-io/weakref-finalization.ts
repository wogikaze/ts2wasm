// WeakRef and FinalizationRegistry (issue I-20260513-BQTVQV)
var wm = new WeakMap<object, string>();
var obj: object = { x: 1 };
wm.set(obj, "value");
console.log("weakmap_ok");
console.log(wm.get(obj));

// WeakRef
var wr = new WeakRef(obj);
console.log("weakref_ok");
var derefed = wr.deref();
console.log(derefed != null ? "deref_ok" : "deref_null");
