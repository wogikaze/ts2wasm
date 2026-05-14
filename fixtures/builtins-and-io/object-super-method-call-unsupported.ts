// Test fixture: should fail to compile (super method call in object literal method)
class Base {
    greet() {
        return "hello";
    }
}

class Derived extends Base {
    method() {
        return super.greet();
    }
}

const obj = new Derived();
console.log(obj.method());
