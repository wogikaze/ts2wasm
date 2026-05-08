class Base {
    value() {
        return 42;
    }
}

class Child extends Base {
    constructor() {
        super();
    }
    parentValue() {
        // Access parent prototype method as a property (returns the function)
        let v = super.value;
        return typeof v;
    }
}

let c = new Child();
console.log(c.parentValue());
