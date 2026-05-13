class Greeter {
    prefix = "Hello";
    name = "";
    constructor(n) {
        this.name = n;
    }
    greet() {
        return this.prefix;
    }
    getName() {
        return this.name;
    }
    setPrefix(p) {
        this.prefix = p;
    }
}

let g = new Greeter("World");
console.log(g.greet());
console.log(g.getName());
g.setPrefix("Hi");
console.log(g.greet());
console.log(g.getName());
