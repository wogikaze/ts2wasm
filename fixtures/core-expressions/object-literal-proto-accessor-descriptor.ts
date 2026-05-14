let stored = 0;
let object: any = {
  __proto__: null,
  ["__proto__"]: 1,
  get __proto__() {
    return 33;
  },
  set __proto__(value: any) {
    stored = value;
  },
};

let desc: any = Object.getOwnPropertyDescriptor(object, "__proto__");
console.log(desc.get());
desc.set(44);
console.log(stored);
