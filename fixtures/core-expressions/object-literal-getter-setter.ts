// Object literal with getter/setter (ES5 style accessor)
const obj = {
  _val: 10,
  get value() {
    return this._val;
  },
  set value(v) {
    this._val = v;
  }
};

console.log(obj.value);
obj.value = 42;
console.log(obj.value);
