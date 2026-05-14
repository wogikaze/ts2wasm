const key = "slot";
const obj: any = {
  get [key]() {
    return this._slot;
  },
  set [key](value) {
    this._slot = value;
  },
};

obj[key] = 42;
console.log(obj[key]);
