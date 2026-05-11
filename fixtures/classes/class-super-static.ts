class Base {
    static label() {
        return "base";
    }
}

class Child extends Base {
    static label() {
        return super.label();
    }
}

console.log(Child.label());
