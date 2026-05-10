// Test super in arrow function inside method
class Base {
  name() {
    return "Base";
  }
}

class Child extends Base {
  constructor() {
    super();
  }

  callSuperViaArrow() {
    const fn = () => super.name();
    return fn();
  }
}

const c = new Child();
console.log(c.callSuperViaArrow());
