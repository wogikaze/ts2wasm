// Test that super() constructor call compiles successfully
class Base {
  constructor(value: number) {
    console.log(value);
  }
}

class Derived extends Base {
  constructor() {
    super(42);
  }
}

const obj = new Derived();
