// Property descriptor with getter/setter (W5)
const obj: any = {};
Object.defineProperty(obj, "x", {
  get() { return this._x; },
  set(val) { this._x = val; }
});
obj.x = 42;
console.log(obj.x);
