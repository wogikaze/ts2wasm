class ParameterPropertyDefaults {
  constructor(public x = 2, private y = 3, protected z = 4, readonly label = "p") {}

  sum() {
    return this.x + this.y + this.z;
  }

  name() {
    return this.label;
  }
}

class OptionalParameterProperty {
  constructor(readonly value?: number) {}
}

let first = new ParameterPropertyDefaults();
let second = new ParameterPropertyDefaults(5);
let third = new ParameterPropertyDefaults(5, 6, 7, "q");
let optional = new OptionalParameterProperty();

console.log(first.sum());
console.log(first.name());
console.log(second.sum());
console.log(third.sum());
console.log(third.name());
console.log(optional.value);
