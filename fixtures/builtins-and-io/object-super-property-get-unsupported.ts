// Test fixture: should fail to compile (super property get in object literal method)
class Base {
    x = 42;
}

class Derived extends Base {
    method() {
        return super.x;
    }
}

const obj = new Derived();
console.log(obj.method());
