class Base {
    constructor() {
        this.x = 10;
    }
    getX() {
        return this.x;
    }
}

class Child extends Base {
    constructor() {
        super();
        // super.prop = value writes to `this`
        super.x = 30;
    }
}

let c = new Child();
console.log(c.getX());
