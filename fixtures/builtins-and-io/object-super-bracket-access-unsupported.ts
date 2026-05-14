// Test fixture: should fail to compile (super bracket property access in object method)
class Base {
    x = 99;
}

class Derived extends Base {
    method() {
        return super["x"];
    }
}

const obj = new Derived();
console.log(obj.method());
