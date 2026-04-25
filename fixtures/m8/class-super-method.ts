class Base {
    value() {
        return 4;
    }
}

class Child extends Base {
    value() {
        return super.value();
    }
}

let c = new Child();
console.log(c.value());
