// Regression: default constructor for derived classes must accept any
// number of arguments (constructor(...args) { super(...args); } in JS).
class Base {
    constructor(a: number) {
        console.log(a);
    }
}

// no explicit constructor -- should accept any args
class Derived extends Base {
}

let x = new Derived(10);
console.log(x);
