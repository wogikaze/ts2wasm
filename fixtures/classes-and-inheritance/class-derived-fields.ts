class Base {
    constructor(x) {
        this.x = x;
    }
    getX() {
        return this.x;
    }
}

class Child extends Base {
    y = 0;
    constructor(x, y) {
        super(x);
        this.y = y;
    }
    sum() {
        return this.getX() + this.y;
    }
}

let c = new Child(3, 7);
console.log(c.sum());
