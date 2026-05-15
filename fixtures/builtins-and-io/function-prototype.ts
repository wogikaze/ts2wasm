function add(a: number, b: number) {
    return a + b;
}

function greet(name: string) {
    return "Hello " + name;
}

function empty() {
    return 42;
}

function single(x: number) {
    return x;
}

console.log(add.name);
console.log(add.length);
console.log(greet.name);
console.log(greet.length);
console.log(empty.name);
console.log(empty.length);
console.log(single.name);
console.log(single.length);

// toString
console.log(add.toString());
console.log(greet.toString());

// arrow function toString
var arrowFn = (x: number) => x * 2;
console.log(arrowFn.toString());

// anonymous function expression toString
var anonExpr = function(x: number) { return x + 1; };
console.log(anonExpr.toString());
