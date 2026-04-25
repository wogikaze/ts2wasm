class Box {
    constructor(v) {
        this.v = v;
    }

    get() {
        return this.v;
    }
}

let b = new Box(42);
console.log(b.get());
