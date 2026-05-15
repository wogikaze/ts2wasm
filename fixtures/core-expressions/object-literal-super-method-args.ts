let proto = {
  scale(value, delta) {
    return value + delta + 1;
  },
};

let object = {
  read(value) {
    return super.scale(value, 1);
  },
  *gen() {
    yield super.scale(39, 2);
  },
};

Object.setPrototypeOf(object, proto);

console.log(object.read(40));
console.log(object.gen().next().value);
