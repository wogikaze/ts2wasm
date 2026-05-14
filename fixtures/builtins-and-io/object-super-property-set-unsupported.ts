// Test fixture: should fail to compile (super property set in object literal method)
class Base {
    x = 1;
}

class Derived extends Base {
    method() {
        super.x = 42;
    }
}

const obj = new Derived();
obj.method();
console.log(obj.x);
