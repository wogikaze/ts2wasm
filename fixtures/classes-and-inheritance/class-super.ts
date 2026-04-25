class Base {
    constructor(v) {
        this.v = v;
    }

    value() {
        return this.v;
    }
}

class Child extends Base {
    constructor(v) {
        super(v);
    }
}

let c = new Child(9);
console.log(c.value());
