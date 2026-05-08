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
        // Access parent prototype method via computed super access
        let key = "value";
        let v = super[key];
        return typeof v;
    }
}

let c = new Child();
console.log(c.parentValue());
