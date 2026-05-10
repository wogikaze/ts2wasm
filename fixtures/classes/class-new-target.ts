// Test new.target in constructor
class Base {
  constructor() {
    console.log(typeof new.target);
  }
}

class Child extends Base {
  constructor() {
    super();
  }
}

new Base();
new Child();
